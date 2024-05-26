use std::sync::OnceLock;
use std::sync::RwLock;
use std::time::Duration;
use std::time::SystemTime;

use anyhow::Result;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use tokio::runtime::Runtime;
use windows::Media::Control::GlobalSystemMediaTransportControlsSession as GSMTCSession;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager as GSMTCSessionManager;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionMediaProperties as GSMTCSessionMediaProperties;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus as GSMTCSessionPlaybackStatus;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionTimelineProperties as GSMTCSessionTimelineProperties;

use crate::glib_spawn;
use crate::log::*;
use crate::sync::interop::common::{need_fetch_lyric, update_lyric};
use crate::sync::interop::{OsImp, PlayerId, PlayerStatus};
use crate::sync::lyric::scroll::refresh_lyric;
use crate::sync::TrackMeta;
use crate::utils::reset_lyric_labels;

pub struct GSMTC;

static SESSION: RwLock<Option<GSMTCSession>> = RwLock::new(None);
static TOKIO_RUNTIME: OnceLock<Runtime> = OnceLock::new();

fn media_properties(session: &GSMTCSession) -> Result<GSMTCSessionMediaProperties> {
    let runtime = TOKIO_RUNTIME.get_or_init(|| Runtime::new().unwrap());
    Ok(runtime.block_on(async { session.TryGetMediaPropertiesAsync()?.await })?)
}

fn session_manager() -> &'static GSMTCSessionManager {
    static SESSION_MANAGER: OnceLock<GSMTCSessionManager> = OnceLock::new();
    SESSION_MANAGER.get_or_init(|| {
        let runtime = TOKIO_RUNTIME.get_or_init(|| Runtime::new().unwrap());
        runtime.block_on(async move {
            GSMTCSessionManager::RequestAsync()
                .expect("cannot request GSMTC Session Manager!")
                .await
                .expect("request GSMTC Session Manager failed!")
        })
    })
}

impl OsImp for GSMTC {
    fn clean_player() {
        if let Ok(mut g) = SESSION.write() {
            g.take();
        }
    }

    fn connect_player_with_id(player_id: impl AsRef<str>) {
        let Some(mut sessions) = list_sessions() else {
            return;
        };

        let Some(session) = sessions.find(|s| {
            s.SourceAppUserModelId()
                .is_ok_and(|id| id == player_id.as_ref())
        }) else {
            return;
        };
        if let Ok(mut guard) = SESSION.write() {
            info!("connected to {}", player_id.as_ref());
            guard.replace(session);
        }
    }

    fn hint_from_player() -> Option<crate::sync::lyric::fetch::LyricHint> {
        None
    }

    fn list_players() -> Vec<PlayerId> {
        let Some(sessions) = list_sessions() else {
            return vec![];
        };

        sessions
            .filter_map(|s| {
                let app_user_model_id = s.SourceAppUserModelId().ok()?;
                Some(PlayerId {
                    player_name: app_user_model_id.to_string(),
                    inner_id: app_user_model_id.to_string(),
                })
            })
            .collect()
    }

    fn reconnect_player() -> bool {
        let session_manager = session_manager();
        if let Ok(session) = session_manager.GetCurrentSession() {
            SESSION
                .write()
                .expect("poisoned SESSION RwLock!")
                .replace(session);
            true
        } else {
            false
        }
    }

    fn try_sync_track(window: &crate::app::Window) -> Result<(), PlayerStatus> {
        let Some(session) = SESSION.read().unwrap().clone() else {
            // `PlayerStatus::Missing` if SESSION was not set
            return Err(PlayerStatus::Missing);
        };

        // `PlayStatus::Missing` if player terminates
        session
            .SourceAppUserModelId()
            .map_err(|_| PlayerStatus::Missing)?;

        let timeline_properties = session.GetTimelineProperties().map_err(|e| {
            error!("try_sync_track failed: {e}");
            PlayerStatus::Unsupported("failed to get TimelineProperties!")
        })?;
        update_position(window, &timeline_properties)?;

        let playback_info = session.GetPlaybackInfo().map_err(|e| {
            error!("try_sync_track failed: {e}");
            PlayerStatus::Unsupported("failed to get PlaybackInfo!")
        })?;

        let GSMTCSessionPlaybackStatus(playback_status) =
            playback_info.PlaybackStatus().map_err(|e| {
                error!("try_sync_track failed: {e}");
                PlayerStatus::Unsupported("failed to get PlaybackStatus!")
            })?;
        trace!("PlaybackStatus = {}", playback_status);
        match playback_status {
            // Closed
            0 => Err(PlayerStatus::Missing)?,
            // Opened
            1 => Err(PlayerStatus::Paused)?,
            // Changing
            2 => Err(PlayerStatus::Paused)?,
            3 => Err(PlayerStatus::Stopped)?,
            // Playing
            4 => (),
            5 => Err(PlayerStatus::Paused)?,
            s => panic!("unknown PlaybackStatus {s}!"),
        }
        let media_properties = media_properties(&session).map_err(|e| {
            error!("try_sync_track failed: {e}");
            PlayerStatus::Unsupported("failed to get MediaProperties!")
        })?;

        let album = media_properties.AlbumTitle().ok().map(|t| t.to_string());
        let title = media_properties.Title().ok().map(|t| t.to_string());
        let artist = media_properties.Artist().ok().map(|t| t.to_string());

        let mut length = timeline_properties.EndTime().ok().map(Duration::from);

        // workaround: go-musicfox v4.4.0 will give zero EndTime
        if length.is_some_and(|l| l.is_zero()) {
            length = None;
        }

        let new_trackmeta = TrackMeta {
            unique_song_id: None,
            title,
            album,
            artists: artist.map(|a| vec![a]),
            length,
        };

        if need_fetch_lyric(&new_trackmeta) {
            let window = gtk::prelude::ObjectExt::downgrade(window);
            crate::log::debug!("spawned update_lyric from try_sync_track");
            glib_spawn!(async move {
                let Some(window) = window.upgrade() else {
                    return;
                };
                reset_lyric_labels(&window, None);
                if let Err(e) = update_lyric(&new_trackmeta, &window, false).await {
                    error!("{e} occurs fetching lyric")
                }
            });
        }

        refresh_lyric(window, false);
        Ok(())
    }
}

fn update_position(
    window: &crate::app::Window,
    timeline_properties: &GSMTCSessionTimelineProperties,
) -> Result<(), PlayerStatus> {
    // days from 1601-01-01 to 1970-01-01
    const UNIX_EPOCH_UNIVERSAL_TIME_DIFF_DAY: u64 = 134774;

    let position: Duration = timeline_properties
        .Position()
        .map_err(|e| {
            error!("try_sync_track failed: {e}");
            PlayerStatus::Unsupported("failed to get Position!")
        })?
        .into();
    trace!("got position: {:.02}s", position.as_secs_f64());
    let start = match timeline_properties.LastUpdatedTime() {
        Ok(update_time) => {
            // UniversalTime is times of 100ns
            // that since UTC 1601.01.01
            let update_time_us = update_time.UniversalTime / 10;
            // update_time - unix_univer_diff = time since unix epoch
            let update_time = Duration::from_micros(update_time_us as _)
                - Duration::from_secs(UNIX_EPOCH_UNIVERSAL_TIME_DIFF_DAY * 24 * 60 * 60);
            SystemTime::UNIX_EPOCH
                .checked_add(update_time)
                .ok_or(PlayerStatus::Unsupported(
                    "Bug from WinRT: infinite LastUpdateTime!",
                ))?
                .checked_sub(position)
                .ok_or(PlayerStatus::Unsupported("Infinite position!"))?
        }
        Err(_) => SystemTime::now()
            .checked_sub(position)
            .ok_or(PlayerStatus::Unsupported("Infinite position!"))?,
    };

    trace!(
        "start time: {:?}",
        start.duration_since(SystemTime::UNIX_EPOCH)
    );

    let offset = window.imp().lyric_offset_ms.get();
    let start_time = if offset.is_negative() {
        start.checked_sub(Duration::from_millis(offset.unsigned_abs()))
    } else {
        start.checked_add(Duration::from_millis(offset as _))
    }
    .ok_or(PlayerStatus::Unsupported("infinite offset time"))?;
    window.imp().lyric_start.set(Some(start_time));

    Ok(())
}

fn list_sessions() -> Option<impl Iterator<Item = GSMTCSession>> {
    let session_manager = session_manager();
    let Ok(sessions) = session_manager.GetSessions() else {
        return None;
    };

    Some(sessions.into_iter())
}
