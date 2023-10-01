use std::time::{Duration, SystemTime};

use gtk::{subclass::prelude::*, prelude::*, glib::{WeakRef, self}, Application};
use mpris::{Metadata, PlaybackStatus, Player, ProgressTracker};
use tracing::{error, info, trace};

use crate::{
    app,
    sync::{PLAYER, TRACK_PLAYING_PAUSED, utils, PLAYER_FINDER, lyric::refresh_lyric},
    DEFAULT_TEXT,
};

pub mod acts;

pub fn need_fetch_lyric(track_meta: &Metadata) -> bool {
    TRACK_PLAYING_PAUSED.with_borrow_mut(|(track_id_playing, paused)| {
        let Some(track_id) = track_meta.track_id() else {
            *track_id_playing = None;
            *paused = false;
            return false;
        };

        trace!("got track_id: {track_id}");

        let need = track_id_playing.is_none()
            || track_id_playing.as_ref().is_some_and(|p| p != &track_id);

        *track_id_playing = Some(track_id);
        *paused = false;
        need
    })
}

pub fn fetch_lyric(track_meta: &Metadata, window: &app::Window) -> Result<(), PlayerStatus> {
    crate::sync::utils::clean_lyric(window);

    let title = track_meta
        .title()
        .ok_or(PlayerStatus::Unsupported("cannot get song title"))?;
    let album = track_meta.album_name();
    let artists = track_meta.artists();

    let length = track_meta.length();

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

        if need_fetch_lyric(&track_meta) {
            fetch_lyric(&track_meta, window)?;
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
                TRACK_PLAYING_PAUSED.set((None, false));
            }
            Err(PlayerStatus::Unsupported(kind)) => {
                app::get_label(&window, "above").set_label("Unsupported Player");
                app::get_label(&window, "below").set_label("");

                utils::clean_lyric(&window);
                error!(kind);
            }
            Err(PlayerStatus::Paused) => {
                TRACK_PLAYING_PAUSED.with_borrow_mut(|(_, paused)| *paused = true)
            }
            _ => (),
        }

        Continue(true)
    });
}