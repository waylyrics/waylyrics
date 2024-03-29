use std::{path::PathBuf, time::Duration};

use gtk::subclass::prelude::*;
use sorensen::distance;

use crate::log::trace;
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
            // to fuzzt-match. so we skip fuzzy-match and use
            // the first result instead
            if album.is_none() && singer.is_none() {
                return None;
            }

            let title: Vec<char> = title.chars().collect();
            let album: Option<Vec<char>> = album.map(|a| a.chars().collect());
            let singer: Option<Vec<char>> = singer.map(|s| s.chars().collect());
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
                            &r_title.chars().collect::<Vec<_>>(),
                            r_album
                                .as_ref()
                                .map(|a| a.chars().collect::<Vec<_>>())
                                .as_deref(),
                            &r_singer.chars().collect::<Vec<_>>(),
                        );
                        trace!("p={likelihood} for {s:?}");
                        (s, likelihood)
                    },
                )
                .max_by_key(|(_, likelihood)| (likelihood * 1024.) as u32)
                .map(|(s, _)| (s, 1))
        })
        .or(search_result.first().and_then(|song| Some((song, 2))))
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
    let singer_likelihood = || distance(singer.unwrap_or_default(), r_singer) as f64;
    let album_likelihood =
        || distance(album.unwrap_or_default(), r_album.unwrap_or_default()) as f64;
    match (singer, album) {
        (Some(_), Some(_)) => {
            title_likelihood * 0.6 + singer_likelihood() * 0.3 + album_likelihood() * 0.1
        }
        (Some(_), None) => title_likelihood * 0.7 + singer_likelihood() * 0.3,
        (None, Some(_)) => title_likelihood * 0.9 + album_likelihood() * 0.1,
        (None, None) => title_likelihood,
    }
}
