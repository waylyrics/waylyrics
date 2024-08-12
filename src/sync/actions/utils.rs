use crate::sync::{lyric::cache::update_lyric_cache, TrackState, TRACK_PLAYING_STATE};

pub fn update_cache() {
    TRACK_PLAYING_STATE.with_borrow(|TrackState { cache_path, .. }| {
        if let Some(cache_path) = cache_path {
            update_lyric_cache(cache_path);
        }
    });
}
