use std::{path::PathBuf, time::Duration};

use gtk::subclass::prelude::*;
use sorensen::distance;

use crate::log::*;
use crate::{app, lyric_providers::SongInfo};

use super::{LyricState, TrackState, LYRIC, TRACK_PLAYING_STATE};

pub fn clean_lyric(window: &app::Window) {
    LYRIC.set(Default::default());
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
                .map(|song| (song, 0))
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

            #[cfg(feature = "opencc")]
            let title: Vec<char> = opencc.convert(title).chars().collect();
            #[cfg(not(feature = "opencc"))]
            let title: Vec<char> = title.chars().collect();

            let album: Option<Vec<char>> = album.map(|a| {
                #[cfg(feature = "opencc")]
                return opencc.convert(a).chars().collect();
                #[cfg(not(feature = "opencc"))]
                return a.chars().collect();
            });
            let singer: Option<Vec<char>> = singer.map(|s| {
                #[cfg(feature = "opencc")]
                return opencc.convert(s).chars().collect();
                #[cfg(not(feature = "opencc"))]
                return s.chars().collect();
            });
            search_result
                .iter()
                .map(
                    |s @ SongInfo {
                         title: r_title,
                         singer: r_singer,
                         album: r_album,
                         ..
                     }| {
                        let likelihood = fuzzy_match_song(
                            &title,
                            album.as_deref(),
                            singer.as_deref(),
                            #[cfg(feature = "opencc")]
                            &opencc.convert(r_title).chars().collect::<Vec<_>>(),
                            #[cfg(feature = "opencc")]
                            r_album
                                .as_ref()
                                .map(|a| opencc.convert(a).chars().collect::<Vec<_>>())
                                .as_deref(),
                            #[cfg(feature = "opencc")]
                            &opencc.convert(r_singer).chars().collect::<Vec<_>>(),
                            #[cfg(not(feature = "opencc"))]
                            &r_title.chars().collect::<Vec<_>>(),
                            #[cfg(not(feature = "opencc"))]
                            r_album
                                .as_ref()
                                .map(|a| a.chars().collect::<Vec<_>>())
                                .as_deref(),
                            #[cfg(not(feature = "opencc"))]
                            &r_singer.chars().collect::<Vec<_>>(),
                        );
                        trace!("p={likelihood} for {s:?}");
                        (s, likelihood)
                    },
                )
                .max_by_key(|(_, likelihood)| (likelihood * 1024.) as u32)
                .map(|(s, _)| (s, 1))
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
            title_likelihood * 0.6 + singer_likelihood() * 0.3 + album_likelihood() * 0.1
        }
        (Some(_), None) => title_likelihood * 0.7 + singer_likelihood() * 0.3,
        (None, Some(_)) => title_likelihood * 0.9 + album_likelihood() * 0.1,
        (None, None) => title_likelihood,
    }
}
