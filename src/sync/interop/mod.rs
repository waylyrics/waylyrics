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
    sync::{
        lyric::cache::get_cache_path,
        lyric::{cache, fetch, scroll::refresh_lyric},
        utils, TrackMeta, PLAYER, PLAYER_FINDER, TRACK_PLAYING_STATE,
    },
    utils::reset_lyric_labels,
};

use crate::sync::TrackState;

pub mod acts;

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
        let album = meta.album_name().map(ToOwned::to_owned);
        let artists: Option<Vec<_>> = meta
            .artists()
            .map(|v| v.iter().map(ToString::to_string).collect());
        let length = meta.length();

        Ok(Self {
            track_id,
            title,
            album,
            artists,
            length,
            meta,
        })
    }
}

pub fn list_avaliable_players() -> Vec<Player> {
    PLAYER_FINDER.with_borrow(|player_finder| match player_finder.find_all() {
        Ok(players) => players,
        Err(e) => {
            error!("cannot find players!: {e}");
            panic!("please check your d-bus connection!")
        }
    })
}

pub fn need_fetch_lyric(track_meta: &TrackMeta) -> bool {
    TRACK_PLAYING_STATE.with_borrow_mut(
        |TrackState {
             metainfo,
             paused,
             cache_path,
         }| {
            let track_id_playing = metainfo.as_ref().map(|t| t.track_id.clone());
            let track_id = &track_meta.track_id;
            trace!("got track_id: {track_id}");

            let need = track_id_playing.is_none()
                || track_id_playing.as_ref().is_some_and(|p| p != track_id);

            *metainfo = Some(track_meta.clone());
            *paused = false;
            *cache_path = Some(get_cache_path(track_meta));
            need
        },
    )
}

pub fn update_lyric(
    track_meta: &TrackMeta,
    window: &app::Window,
    ignore_cache: bool,
) -> Result<(), PlayerStatus> {
    crate::sync::utils::clean_lyric(window);

    let fetch_result = if window.imp().cache_lyrics.get() {
        cache::fetch_lyric_cached(track_meta, ignore_cache, window)
    } else {
        fetch::fetch_lyric(track_meta, window)
    };

    if let Err(e) = fetch_result {
        error!("lyric fetch error: {e}");
    }

    reset_lyric_labels(window);
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
                TRACK_PLAYING_STATE.take();
                return Err(e);
            }
        };

        if need_fetch_lyric(&meta) {
            update_lyric(&meta, window, false)?;
        }

        sync_position(player, window)?;
        refresh_lyric(window);
        Ok(())
    })
}

#[derive(Debug)]
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
                reset_lyric_labels(&window);
                utils::clean_lyric(&window);
                TRACK_PLAYING_STATE.take();
            }
            Err(PlayerStatus::Unsupported(kind)) => {
                app::get_label(&window, "above").set_label("Unsupported Player");
                app::get_label(&window, "below").set_label("");

                utils::clean_lyric(&window);
                error!(kind);
            }
            Err(PlayerStatus::Paused) => {
                TRACK_PLAYING_STATE.with_borrow_mut(|TrackState { paused, .. }| *paused = true)
            }
            _ => (),
        }

        Continue(true)
    });
}
