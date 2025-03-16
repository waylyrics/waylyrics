use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use ahash::{HashMap, HashMapExt};
use lofty::file::TaggedFileExt;
use lofty::read_from_path;
use lofty::tag::ItemKey;
use once_cell::sync::Lazy;

use crate::log::{debug, error, warn};
use crate::lyric_providers::{Lyric, LyricOwned, LyricProvider};
use crate::sync::interop::{OsImp, OS};
use crate::sync::utils::extract_translated_lyric;
use crate::sync::{filter_original_lyric, TrackMeta};
use crate::LYRIC_PROVIDERS;

#[derive(Debug)]
pub enum LyricHint {
    SongId {
        song_id: String,
        provider: &'static dyn LyricProvider,
    },
    LyricFile(PathBuf),
    LyricMetadata(PathBuf),
    Metadata(TrackMeta),
}

pub enum LyricHintResult {
    Lyric {
        olyric: LyricOwned,
        tlyric: LyricOwned,
    },
}

pub async fn get_lyric_hint_from_player() -> Option<LyricHintResult> {
    let hint_from_player: Option<LyricHint> = OS::hint_from_player();

    debug!("got player hint: {:?}", hint_from_player);

    match hint_from_player {
        Some(LyricHint::SongId { song_id, provider }) => {
            if !LYRIC_PROVIDERS.get().iter().any(|&providers| {
                providers
                    .iter()
                    .any(|pro| pro.unique_name() == provider.unique_name())
            }) {
                warn!(
                    "provider {} suggested by hint is not configured, skipping SongId hint",
                    provider.unique_name()
                );
                return None;
            }

            crate::log::debug!("spawned query from get_accurate_lyric");

            let lyric = provider.query_lyric(&song_id).await.ok()?;
            let olyric = provider.parse_lyric(&lyric);
            let tlyric = provider.parse_translated_lyric(&lyric);

            Some(LyricHintResult::Lyric { olyric, tlyric })
        }
        Some(LyricHint::LyricFile(path)) => {
            let (olyric, tlyric) = load_local_lyric(&path)?;
            Some(LyricHintResult::Lyric { olyric, tlyric })
        }
        Some(LyricHint::LyricMetadata(path)) => {
            let (olyric, tlyric) = get_lrc_from_music_metadata(&path)?;
            Some(LyricHintResult::Lyric { olyric, tlyric })
        }
        _ => None,
    }
}

/// replace file extension with .lrc
///
/// `music_path` should be valid file if it's not empty
///
pub fn get_lrc_path(mut music_path: PathBuf) -> Option<PathBuf> {
    if music_path.set_extension("lrc") {
        Some(music_path)
    } else {
        None
    }
}
// 添加一个静态缓存
pub static LYRIC_TAG_CACHE: Lazy<Mutex<HashMap<PathBuf, bool>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub fn lyric_tag_exists(music_path: &Path) -> bool {
    // 尝试从缓存中获取结果
    if let Ok(cache_guard) = LYRIC_TAG_CACHE.lock() {
        if let Some(&result) = cache_guard.get(music_path) {
            return result;
        }
    }
    let result = read_from_path(music_path)
        .map_err(|e| warn!("cannot read music file: {e}"))
        .ok()
        .as_ref()
        .and_then(|tagged_file| tagged_file.primary_tag())
        .and_then(|tag| tag.get(&ItemKey::Lyrics))
        .is_some();
    // 将结果存入缓存
    if let Ok(mut cache_guard) = LYRIC_TAG_CACHE.lock() {
        cache_guard.insert(music_path.to_owned(), result);
    }
    result
}
pub fn get_lrc_from_music_metadata(music_path: &PathBuf) -> Option<(LyricOwned, LyricOwned)> {
    read_from_path(music_path)
        .map_err(|e| error!("cannot read music file: {e}"))
        .ok()
        .as_ref()
        .and_then(|tagged_file| tagged_file.primary_tag())
        .and_then(|tag| tag.get_string(&ItemKey::Lyrics))
        .and_then(parse_local_lyric)
}

pub static EXTRACT_TRANSLATED_LYRIC: OnceLock<bool> = OnceLock::new();

fn parse_local_lyric(lyric: &str) -> Option<(LyricOwned, LyricOwned)> {
    let mut olyric =
        crate::lyric_providers::utils::lrc_iter(lyric.trim_start_matches('\u{feff}').lines())
            .map(|lyrics| Lyric::LineTimestamp(lyrics).into_owned())
            .map_err(|e| error!("cannot parse lyric from hint: {e}"))
            .ok()
            .unwrap_or_default();
    let mut tlyric = LyricOwned::None;
    #[cfg(feature = "i18n-local-lyric")]
    if EXTRACT_TRANSLATED_LYRIC.get().cloned().unwrap_or_default() {
        if let LyricOwned::LineTimestamp(lines) = &olyric {
            let tlyric_lines = extract_translated_lyric(lines);
            if !tlyric_lines.is_empty() {
                let olyric_lines = filter_original_lyric(lines, &tlyric_lines);
                debug!("extracted original lyric: {olyric_lines:#?}");
                debug!("extracted translation lyric: {tlyric_lines:#?}");
                olyric = LyricOwned::LineTimestamp(olyric_lines);
                tlyric = LyricOwned::LineTimestamp(tlyric_lines);
            }
        }
    }

    if olyric.is_none() && tlyric.is_none() {
        return None;
    }

    Some((olyric, tlyric))
}

fn load_local_lyric<P: AsRef<Path>>(path: P) -> Option<(LyricOwned, LyricOwned)> {
    let mut olyric = fs::read_to_string(&path)
        .map_err(|e| error!("cannot read lyric from hint: {e}"))
        .ok()
        .and_then(|lyric| {
            crate::lyric_providers::utils::lrc_iter(lyric.trim_start_matches('\u{feff}').lines())
                .map(|lyrics| Lyric::LineTimestamp(lyrics).into_owned())
                .map_err(|e| error!("cannot parse lyric from hint: {e}"))
                .ok()
        })
        .unwrap_or_default();
    #[cfg(feature = "i18n-local-lyric")]
    let mut tlyric = {
        let mut translation_path = path.as_ref().to_owned();
        let lang = sys_locale::get_locale();
        translation_path.set_extension(format!("{}.lrc", lang.as_deref().unwrap_or("zh")));
        fs::read_to_string(&translation_path)
            .map_err(|e| error!("cannot read translated lyric from hint: {e}"))
            .ok()
            .and_then(|lyric| {
                crate::lyric_providers::utils::lrc_iter(
                    lyric.trim_start_matches('\u{feff}').lines(),
                )
                .map(|lyrics| Lyric::LineTimestamp(lyrics).into_owned())
                .map_err(|e| error!("cannot parse lyric from hint: {e}"))
                .ok()
            })
            .unwrap_or_default()
    };
    #[cfg(not(feature = "i18n-local-lyric"))]
    let mut tlyric = LyricOwned::None;

    if tlyric.is_none() && EXTRACT_TRANSLATED_LYRIC.get().cloned().unwrap_or_default() {
        if let LyricOwned::LineTimestamp(lines) = &olyric {
            let tlyric_lines = extract_translated_lyric(lines);
            if !tlyric_lines.is_empty() {
                let olyric_lines = filter_original_lyric(lines, &tlyric_lines);
                debug!("extracted original lyric: {olyric_lines:#?}");
                debug!("extracted translation lyric: {tlyric_lines:#?}");
                olyric = LyricOwned::LineTimestamp(olyric_lines);
                tlyric = LyricOwned::LineTimestamp(tlyric_lines);
            }
        }
    }

    if olyric.is_none() && tlyric.is_none() {
        return None;
    }

    Some((olyric, tlyric))
}
