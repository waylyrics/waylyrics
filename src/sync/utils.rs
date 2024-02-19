use std::{path::PathBuf, time::Duration};

use fuzzy_match::algorithms::{SimilarityAlgorithm, SorensenDice};
use gtk::subclass::prelude::*;

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
        .or(search_result.first().and_then(|song| {
            // if we get only title, it is likely
            // the player is giving messy title, which is not applicable
            // to fuzzt-match. so we skip fuzzy-match and use
            // the first result instead
            if album.is_none() && singer.is_none() {
                Some((song, 2))
            } else {
                None
            }
        }))
        .or_else(|| {
            let mut fuzzy_match = SorensenDice::new();
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
                            &mut fuzzy_match,
                            title,
                            album,
                            singer,
                            r_title,
                            r_album.as_deref(),
                            r_singer,
                        );
                        trace!("p={likelihood} for {s:?}");
                        (s, likelihood)
                    },
                )
                .max_by_key(|(_, likelihood)| (likelihood * 128.) as u32)
                .map(|(s, _)| (s, 1))
        })
        .map(|(song, weight)| (song.id.as_str(), weight))
}

pub fn set_current_lyric(lyric: LyricState) {
    LYRIC.set(lyric);
}

pub fn get_lyric_cache_path() -> Option<PathBuf> {
    TRACK_PLAYING_STATE.with_borrow(|TrackState { cache_path, .. }| cache_path.as_ref().cloned())
}

fn fuzzy_match_song(
    fuzzy_match: &mut impl SimilarityAlgorithm,
    title: &str,
    album: Option<&str>,
    singer: Option<&str>,
    r_title: &str,
    r_album: Option<&str>,
    r_singer: &str,
) -> f64 {
    let title_likelihood = fuzzy_match.get_similarity(title, r_title) as f64;
    let singer_likelihood = fuzzy_match.get_similarity(singer.unwrap_or_default(), r_singer) as f64;
    let album_likelihood =
        fuzzy_match.get_similarity(album.unwrap_or_default(), r_album.unwrap_or_default()) as f64;
    match (singer, album) {
        (Some(_), Some(_)) => {
            title_likelihood * 0.6 + singer_likelihood * 0.3 + album_likelihood * 0.1
        }
        (Some(_), None) => title_likelihood * 0.7 + singer_likelihood * 0.3,
        (None, Some(_)) => title_likelihood * 0.9 + album_likelihood * 0.1,
        (None, None) => title_likelihood,
    }
}
