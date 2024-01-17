use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;

use crate::log::{debug, error, warn};
use crate::lyric_providers::{provider_fmt, Lyric, LyricOwned, LyricProvider};
use crate::sync::interop::hint_from_player;
use crate::sync::TrackMeta;
use crate::LYRIC_PROVIDERS;

use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub enum LyricHint {
    SongId {
        song_id: String,
        #[derivative(Debug(format_with = "provider_fmt"))]
        provider: Box<dyn LyricProvider>,
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
            let olyric = provider.get_lyric(&lyric);
            let tlyric = provider.get_translated_lyric(&lyric);

            Some(LyricHintResult::Lyric { olyric, tlyric })
        }
        Some(LyricHint::LyricFile(path)) => fs::read_to_string(path)
            .map_err(|e| error!("cannot read lyric from hint: {e}"))
            .ok()
            .and_then(|lyric| {
                crate::lyric_providers::utils::lrc_iter(lyric.lines())
                    .map(|lyrics| Lyric::LineTimestamp(lyrics).into_owned())
                    .map_err(|e| error!("cannot parse lyric from hint: {e}"))
                    .ok()
            })
            .map(|lyric| LyricHintResult::Lyric {
                olyric: lyric,
                tlyric: LyricOwned::None,
            }),

        _ => None,
    }
}

/// replace file extension with .lrc
///
/// `music_path` should be valid file if it's not empty
///
/// ```rust
/// use std::path::PathBuf;
/// use waylyrics::sync::lyric::fetch::tricks::get_lrc_path as get_lyric_path;
///
/// // do not assert file extension length > 3
/// assert_eq!(get_lyric_path(PathBuf::from("/xxx/yyy/test.ts")),
///             Some(PathBuf::from("/xxx/yyy/test.lrc")));
///
/// // neither = 3
/// assert_eq!(get_lyric_path(PathBuf::from("xxx/yyy/test.m3u8")),
///             Some(PathBuf::from("xxx/yyy/test.lrc")));
///
/// // relative path
/// assert_eq!(get_lyric_path(PathBuf::from("../test.m3u8")),
///             Some(PathBuf::from("../test.lrc")));
///
/// // non-ASCII path
/// assert_eq!(get_lyric_path(PathBuf::from("/测试/试试/你好.诶木皮散")),
///             Some(PathBuf::from("/测试/试试/你好.lrc")));
///
/// // empty string
/// assert_eq!(get_lyric_path(PathBuf::from("")),
///             None);
/// ```
pub fn get_lrc_path(music_path: PathBuf) -> Option<PathBuf> {
    let file_name = music_path.iter().last()?.as_encoded_bytes();

    file_name
        .iter()
        .enumerate()
        .rev()
        .find(|&(_, ch)| ch == &b'.')
        .and_then(|(last_dot_pos, _)| {
            let mut lrc_file_name = file_name.split_at(last_dot_pos + 1).0.to_vec();
            lrc_file_name.extend_from_slice("lrc".as_bytes());
            let lrc_file_name = unsafe { OsStr::from_encoded_bytes_unchecked(&lrc_file_name) };

            music_path
                .parent()
                .map(|music_dir| music_dir.join(lrc_file_name))
        })
}
