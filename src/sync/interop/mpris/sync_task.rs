use std::time::{Duration, SystemTime};

use gtk::{
    glib::{self, subclass::types::ObjectSubclassIsExt, WeakRef},
    prelude::*,
    Application,
};

use crate::{log::*, sync::lyric::fetch::LyricHint};
use mpris::{PlaybackStatus, Player, ProgressTracker};

use crate::sync::interop::PlayerStatus;
use crate::{
    glib_spawn,
    sync::{
        interop::common::update_lyric,
        interop::mpris::{PLAYER, PLAYER_FINDER},
        lyric::scroll::refresh_lyric,
    },
};
use anyhow::Result;

mod utils;
use utils::{find_next_player, need_fetch_lyric};

use crate::{
    app,
    sync::{utils::clean_lyric, TrackMeta, TrackState, TRACK_PLAYING_STATE},
    utils::reset_lyric_labels,
};

use super::hint_from_player;

pub fn register_sync_task(app: WeakRef<Application>, interval: Duration) {
    glib::timeout_add_local(interval, move || {
        let Some(app) = app.upgrade() else {
            return glib::ControlFlow::Break;
        };

        let mut windows = app.windows();
        if windows.is_empty() {
            return glib::ControlFlow::Continue;
        }
        let window: app::Window = windows.remove(0).downcast().unwrap();

        match sync_track(&window) {
            Err(PlayerStatus::Missing) => {
                PLAYER_FINDER.with_borrow(|player_finder| {
                    let Some(player) = find_next_player(player_finder) else {
                        PLAYER.set(None);
                        return;
                    };

                    info!("connected to player: {}", player.identity());
                    PLAYER.set(Some(player));
                });
                reset_lyric_labels(&window);
                clean_lyric(&window);
                TRACK_PLAYING_STATE.take();
            }
            Err(PlayerStatus::Unsupported(kind)) => {
                app::get_label(&window, "above").set_label("Unsupported Player");
                app::get_label(&window, "below").set_label(kind);

                clean_lyric(&window);
                error!(kind);
            }
            Err(PlayerStatus::Paused) => {
                TRACK_PLAYING_STATE.with_borrow_mut(|TrackState { paused, .. }| *paused = true)
            }
            Err(PlayerStatus::Stopped) => {
                reset_lyric_labels(&window);
                clean_lyric(&window);
                TRACK_PLAYING_STATE.take();
            }
            _ => (),
        }

        glib::ControlFlow::Continue
    });
}

fn sync_position(player: &Player, window: &app::Window) -> Result<(), PlayerStatus> {
    let position = player
        .get_position()
        .map_err(|_| PlayerStatus::Unsupported("cannot get playback position"))?;
    let start = SystemTime::now()
        .checked_sub(position)
        .ok_or(PlayerStatus::Unsupported(
            "Position is greater than SystemTime",
        ))?;

    let offset = window.imp().lyric_offset_ms.get();
    let start = if offset.is_negative() {
        start.checked_sub(Duration::from_millis(offset.unsigned_abs()))
    } else {
        start.checked_add(Duration::from_millis(offset as _))
    }
    .expect("infinite offset time");

    window.imp().lyric_start.set(Some(start));
    Ok(())
}

fn sync_track(window: &crate::app::Window) -> Result<(), PlayerStatus> {
    let meta = PLAYER.with_borrow(|player| {
        let player = player.as_ref().ok_or(PlayerStatus::Missing)?;

        if !player.is_running() {
            info!("disconnected from player: {}", player.identity());
            return Err(PlayerStatus::Missing);
        }

        let mut progress_tracker = ProgressTracker::new(player, 0)
            .map_err(|_| PlayerStatus::Unsupported("cannot fetch progress"))?;

        let progress_tick = progress_tracker.tick();
        match progress_tick.progress.playback_status() {
            PlaybackStatus::Playing => (),
            PlaybackStatus::Paused => return Err(PlayerStatus::Paused),
            PlaybackStatus::Stopped => return Err(PlayerStatus::Stopped),
        }

        sync_position(player, window)?;

        if let Some(LyricHint::Metadata(meta)) = hint_from_player() {
            return Ok(meta);
        }
        let track_meta = player
            .get_metadata()
            .map_err(|_| PlayerStatus::Unsupported("cannot get metadata of the track playing"))?;
        let meta = match TrackMeta::try_from(track_meta) {
            Ok(meta) => meta,
            Err(e) => {
                // no track_id or title is available
                // reset
                TRACK_PLAYING_STATE.take();
                return Err(e);
            }
        };

        Ok(meta)
    })?;

    if need_fetch_lyric(&meta) {
        let window = gtk::prelude::ObjectExt::downgrade(window);
        crate::log::debug!("spawned update_lyric from sync_track");
        glib_spawn!(async move {
            let Some(window) = window.upgrade() else {
                return;
            };
            reset_lyric_labels(&window);
            if let Err(e) = update_lyric(&meta, &window, false).await {
                error!("{e} occurs fetching lyric")
            }
        });
    }

    refresh_lyric(window);
    Ok(())
}
