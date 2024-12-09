use std::{path::PathBuf, time::Duration};

use ahash::HashMap;
use gtk::subclass::prelude::*;
use sorensen::distance;

use crate::log::*;
use crate::lyric_providers::LyricLineOwned;
use crate::{app, lyric_providers::SongInfo};

use super::{LyricState, TrackState, LYRIC, TRACK_PLAYING_STATE};

pub fn clean_lyric(window: &app::Window) {
    LYRIC.set(LyricState::default());
    window.imp().lyric_offset_ms.set(0);
}

/// both singer and album are optional non-empty string
pub fn match_likely_lyric<'a>(
    album: Option<&str>,
    title: &str,
    singer: Option<&str>,
    length: Option<Duration>,
    search_result: &'a [SongInfo],
    length_toleration_ms: u128,
) -> Option<(&'a str, u8)> {
    length
        .and_then(|leng| {
            search_result
                .iter()
                .find(|SongInfo { length, .. }| {
                    length.as_millis().abs_diff(leng.as_millis()) <= length_toleration_ms
                })
                .map(|song| (song, 1))
        })
        .or_else(|| {
            // if we get only title, it is likely
            // the player is giving messy title, which is not applicable
            // to fuzzy-match. so we skip fuzzy-match and use
            // the first result instead
            if album.is_none() && singer.is_none() {
                return None;
            }

            #[cfg(feature = "opencc")]
            let Ok(opencc) = opencc_rust::OpenCC::new("t2s.json") else {
                error!("opencc enabled but missing t2s.json dictionary");
                return None;
            };

            let o_title: Vec<char>;
            let o_album: Option<Vec<char>>;
            let o_singer: Option<Vec<char>>;

            #[cfg(feature = "opencc")]
            {
                o_title = opencc.convert(title).chars().collect();
                o_album = album.map(|a| opencc.convert(a).chars().collect());
                o_singer = singer.map(|s| opencc.convert(s).chars().collect());
            }
            #[cfg(not(feature = "opencc"))]
            {
                o_title = title.chars().collect();
                o_album = album.map(|a| a.chars().collect());
                o_singer = singer.map(|s| s.chars().collect());
            }

            search_result
                .iter()
                .map(
                    |s @ SongInfo {
                         title: _title,
                         singer: _singer,
                         album: _album,
                         ..
                     }| {
                        let r_title;
                        let r_album;
                        let r_singer;

                        #[cfg(feature = "opencc")]
                        {
                            r_title = opencc.convert(_title).chars().collect::<Vec<_>>();
                            r_album = _album
                                .as_ref()
                                .map(|a| opencc.convert(a).chars().collect::<Vec<_>>());
                            r_singer = opencc.convert(_singer).chars().collect::<Vec<_>>();
                        }
                        #[cfg(not(feature = "opencc"))]
                        {
                            r_title = _title.chars().collect::<Vec<_>>();
                            r_album = _album.as_ref().map(|a| a.chars().collect::<Vec<_>>());
                            r_singer = _singer.chars().collect::<Vec<_>>();
                        }

                        let likelihood = fuzzy_match_song(
                            &o_title,
                            o_album.as_deref(),
                            o_singer.as_deref(),
                            &r_title,
                            r_album.as_deref(),
                            &r_singer,
                        );
                        trace!("p={likelihood} for {s:?}");
                        (s, likelihood)
                    },
                )
                .max_by_key(|(_, likelihood)| (likelihood * 1024.) as u32)
                .map(|(s, _)| (s, 0))
        })
        .or(search_result.first().map(|song| (song, 2)))
        .map(|(song, weight)| (song.id.as_str(), weight))
}

pub fn set_current_lyric(lyric: LyricState) {
    LYRIC.set(lyric);
}

pub fn get_lyric_cache_path() -> Option<PathBuf> {
    TRACK_PLAYING_STATE.with_borrow(|TrackState { cache_path, .. }| cache_path.as_ref().cloned())
}

pub fn fuzzy_match_song(
    title: &[char],
    album: Option<&[char]>,
    singer: Option<&[char]>,
    r_title: &[char],
    r_album: Option<&[char]>,
    r_singer: &[char],
) -> f64 {
    let title_likelihood = distance(title, r_title);
    let singer_likelihood = || distance(singer.unwrap_or_default(), r_singer);
    let album_likelihood = || distance(album.unwrap_or_default(), r_album.unwrap_or_default());
    match (singer, album) {
        (Some(_), Some(_)) => {
            title_likelihood * 0.4 + singer_likelihood() * 0.2 + album_likelihood() * 0.4
        }
        (Some(_), None) => title_likelihood * 0.7 + singer_likelihood() * 0.3,
        (None, Some(_)) => title_likelihood * 0.8 + album_likelihood() * 0.2,
        (None, None) => title_likelihood,
    }
}

pub fn extract_translated_lyric(lyric: impl AsRef<[LyricLineOwned]>) -> Vec<LyricLineOwned> {
    let tlyric_lines = lyric
        .as_ref()
        .windows(2)
        .filter_map(|l| <&[LyricLineOwned; 2]>::try_from(l).ok())
        // this should work because we have sorted lyrics by timestamp
        .filter(|&[a, b]| a.start_time == b.start_time)
        .map(|p| &p[1])
        .cloned()
        .collect::<Vec<_>>();
    tlyric_lines
}

pub fn filter_original_lyric(
    lyric: impl AsRef<[LyricLineOwned]>,
    tlyric: impl AsRef<[LyricLineOwned]>,
) -> Vec<LyricLineOwned> {
    let tlyric = tlyric
        .as_ref()
        .iter()
        .map(|LyricLineOwned { text, start_time }| (start_time, text))
        .collect::<HashMap<_, _>>();
    let tlyric_lines = lyric
        .as_ref()
        .iter()
        .filter(|LyricLineOwned { text, start_time }| {
            !tlyric.contains_key(start_time) || tlyric[start_time] != text
        })
        .cloned()
        .collect::<Vec<_>>();
    tlyric_lines
}
