use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use crate::log::{debug, error, warn};
use crate::lyric_providers::{Lyric, LyricLineOwned, LyricOwned, LyricProvider};
use crate::sync::interop::hint_from_player;
use crate::sync::TrackMeta;
use crate::LYRIC_PROVIDERS;

#[derive(Debug)]
pub enum LyricHint {
    SongId {
        song_id: String,
        provider: &'static dyn LyricProvider,
    },
    LyricFile(PathBuf),
    Metadata(TrackMeta),
}

pub enum LyricHintResult {
    Lyric {
        olyric: LyricOwned,
        tlyric: LyricOwned,
    },
}

pub async fn get_lyric_hint_from_player() -> Option<LyricHintResult> {
    let hint_from_player: Option<LyricHint> = hint_from_player();

    debug!("got player hint: {:?}", hint_from_player);

    match hint_from_player {
        Some(LyricHint::SongId { song_id, provider }) => {
            if !LYRIC_PROVIDERS.get().iter().any(|&providers| {
                providers
                    .iter()
                    .any(|pro| pro.unique_name() == provider.unique_name())
            }) {
                warn!(
                    "provider {} suggrested by hint is not configured, skipping SongId hint",
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

pub static EXTRACT_TRANSLATED_LYRIC: OnceLock<bool> = OnceLock::new();

fn load_local_lyric<P: AsRef<Path>>(path: P) -> Option<(LyricOwned, LyricOwned)> {
    let olyric = fs::read_to_string(&path)
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
        translation_path.set_extension(&format!("{}.lrc", lang.as_deref().unwrap_or("zh")));
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
            let tlyric_lines = lines
                .windows(2)
                .filter_map(|l| <&[LyricLineOwned; 2]>::try_from(l).ok())
                // this should work because we have sorted lyrics by timestamp
                .filter(|&[a, b]| a == b)
                .map(|[_, b]| b)
                .cloned()
                .collect::<Vec<_>>();
            if !tlyric_lines.is_empty() {
                tlyric = LyricOwned::LineTimestamp(tlyric_lines);
            }
        }
    }

    if olyric.is_none() && tlyric.is_none() {
        return None;
    }

    Some((olyric, tlyric))
}
