use std::{path::PathBuf, time::Duration};

use gtk::subclass::prelude::*;

use crate::{app, lyric_providers::SongInfo};

use super::{LyricState, TrackState, LYRIC, TRACK_PLAYING_STATE};

pub fn clean_lyric(window: &app::Window) {
    LYRIC.set(Default::default());
    window.imp().lyric_offset_ms.set(0);
}

pub fn match_likely_lyric<'a>(
    album_title: Option<(&str, &str)>,
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
            album_title
                .and_then(|(_album, _title)| {
                    search_result.iter().find(|SongInfo { title, album, .. }| {
                        title == _title && album.as_ref().is_some_and(|album| album == _album)
                    })
                })
                .map(|song| (song, 1))
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
