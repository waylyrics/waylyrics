use std::time::{Duration, SystemTime};

use gtk::{
    glib::{self, WeakRef},
    prelude::*,
    subclass::prelude::*,
    Application,
};
use mpris::{Metadata, PlaybackStatus, Player, ProgressTracker};
use tracing::{error, info, trace};

use crate::{
    app,
    sync::{cache::get_cache_path, utils, PLAYER, PLAYER_FINDER, TRACK_PLAYING_STATE, lyric::refresh_lyric},
    DEFAULT_TEXT,
};

pub mod acts;

/// A struct from metadata in mpris::TrackID to avoid track_id and title unwrapping
pub struct TrackMeta {
    pub track_id: mpris::TrackID,
    pub title: String,
    pub meta: Metadata,
}

impl TryFrom<Metadata> for TrackMeta {
    type Error = PlayerStatus;

    fn try_from(meta: Metadata) -> Result<Self, Self::Error> {
        let track_id = meta
            .track_id()
            .ok_or(())
            .map_err(|_| PlayerStatus::Unsupported("cannot get track id"))?;
        let title = meta
            .title()
            .ok_or(())
            .map_err(|_| PlayerStatus::Unsupported("cannot get title"))?
            .to_string();

        Ok(Self {
            track_id,
            title,
            meta,
        })
    }
}


pub fn need_fetch_lyric(track_meta: &TrackMeta) -> bool {
    TRACK_PLAYING_STATE.with_borrow_mut(|(track_id_playing, paused, cache_path)| {
        let track_id = &track_meta.track_id;
        trace!("got track_id: {track_id}");

        let need =
            track_id_playing.is_none() || track_id_playing.as_ref().is_some_and(|p| p != track_id);

        *track_id_playing = Some(track_id.clone());
        *paused = false;
        *cache_path = Some(get_cache_path(
            &track_meta.title,
            track_meta.meta.album_name(),
            track_meta.meta.artists().as_deref(),
            track_meta.meta.length(),
        ));
        need
    })
}

pub fn fetch_lyric(track_meta: &TrackMeta, window: &app::Window) -> Result<(), PlayerStatus> {

    crate::sync::utils::clean_lyric(window);

    let title = &track_meta.title;
    let album = track_meta.meta.album_name();
    let artists = track_meta.meta.artists();

    let length = track_meta.meta.length();

    let fetch_result = if window.imp().cache_lyrics.get() {
        crate::sync::cache::fetch_lyric_cached(title, album, artists.as_deref(), length, window)
    } else {
        super::fetch_lyric(title, album, artists.as_deref(), length, window)
    };

    if let Err(e) = fetch_result {
        error!("lyric fetch error: {e}");
    }

    app::get_label(window, "above").set_label(DEFAULT_TEXT);
    app::get_label(window, "below").set_label("");
    Ok(())
}

pub fn sync_position(player: &Player, window: &app::Window) -> Result<(), PlayerStatus> {
    let position = player
        .get_position()
        .map_err(|_| PlayerStatus::Unsupported("cannot get playback position"))?;
    let mut start = SystemTime::now()
        .checked_sub(position)
        .ok_or(PlayerStatus::Unsupported(
            "Position is greater than SystemTime",
        ))?;

    let offset = window.imp().lyric_offset_ms.get();
    if offset.is_negative() {
        start = start
            .checked_sub(Duration::from_millis(offset.unsigned_abs()))
            .expect("infinite offset time");
    } else {
        start = start
            .checked_add(Duration::from_millis(offset as _))
            .expect("infinite offset time");
    }

    window.imp().lyric_start.set(Some(start));
    Ok(())
}

pub fn sync_track(window: &crate::app::Window) -> Result<(), PlayerStatus> {
    PLAYER.with_borrow(|player| {
        let player = player.as_ref().ok_or(PlayerStatus::Missing)?;

        if !player.is_running() {
            info!("disconnected from player: {}", player.identity());
            return Err(PlayerStatus::Missing);
        }

        let mut progress_tracker = ProgressTracker::new(player, 0)
            .map_err(|_| PlayerStatus::Unsupported("cannot fetch progress"))?;

        let progress_tick = progress_tracker.tick();
        if progress_tick.progress.playback_status() != PlaybackStatus::Playing {
            return Err(PlayerStatus::Paused);
        }
        let track_meta = player
            .get_metadata()
            .map_err(|_| PlayerStatus::Unsupported("cannot get metadata of track playing"))?;

        let meta = match TrackMeta::try_from(track_meta) {
            Ok(meta) => meta,
            Err(e) => {
                // no track_id or title is available
                // reset
                TRACK_PLAYING_STATE.set((None, false, None));
                return Err(e);
            }
        };

        if need_fetch_lyric(&meta) {
            fetch_lyric(&meta, window)?;
        }

        sync_position(player, window)?;
        refresh_lyric(window);
        Ok(())
    })
}

pub enum PlayerStatus {
    Missing,
    Paused,
    Unsupported(&'static str),
}

pub fn register_mpris_sync(app: WeakRef<Application>, interval: Duration) {
    glib::timeout_add_local(interval, move || {
        let Some(app) = app.upgrade() else {
            return Continue(false);
        };

        let mut windows = app.windows();
        if windows.is_empty() {
            return Continue(true);
        }
        let window: app::Window = windows.remove(0).downcast().unwrap();

        match sync_track(&window) {
            Err(PlayerStatus::Missing) => {
                PLAYER_FINDER.with_borrow(|player_finder| {
                    let Ok(player) = player_finder.find_active() else {
                        PLAYER.set(None);
                        return;
                    };

                    info!("connected to player: {}", player.identity());
                    PLAYER.set(Some(player));
                });
                app::get_label(&window, "above").set_label(DEFAULT_TEXT);
                app::get_label(&window, "below").set_label("");
                utils::clean_lyric(&window);
                TRACK_PLAYING_STATE.set((None, false, None));
            }
            Err(PlayerStatus::Unsupported(kind)) => {
                app::get_label(&window, "above").set_label("Unsupported Player");
                app::get_label(&window, "below").set_label("");

                utils::clean_lyric(&window);
                error!(kind);
            }
            Err(PlayerStatus::Paused) => {
                TRACK_PLAYING_STATE.with_borrow_mut(|(_, paused, _)| *paused = true)
            }
            _ => (),
        }

        Continue(true)
    });
}
