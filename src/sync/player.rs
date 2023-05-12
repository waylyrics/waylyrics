use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use gtk::glib::WeakRef;
use gtk::prelude::*;
use gtk::{glib, Application};
use mpris::{PlaybackStatus, ProgressTracker};
use tracing::{debug, error, info};

use crate::app::get_label;
use crate::lyric::netease::NeteaseLyricProvider;
use crate::lyric::{LyricOwned, LyricProvider, LyricStore, SongInfo};
use crate::CACHE_DIR;

use super::{
    utils, CACHE_LYRICS, DEFAULT_TEXT, LYRIC, LYRIC_START, PLAYER, PLAYER_FINDER,
    TOKIO_RUNTIME_HANDLE, TRACK_PLAYING_PAUSED,
};

enum PlayerStatus {
    Missing,
    Paused,
    Unsupported(&'static str),
}

fn try_sync_player(window: &gtk::Window) -> Result<(), PlayerStatus> {
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
        let need_update_lyric =
            TRACK_PLAYING_PAUSED.with_borrow_mut(|(track_id_playing, paused)| {
                if let Some(track_id) = track_meta.track_id() {
                    let need = track_id_playing.is_none()
                        || track_id_playing.as_ref().is_some_and(|p| p != &track_id)
                            && !(*paused
                                && track_id_playing.as_ref().is_some_and(|p| p == &track_id));

                    *track_id_playing = Some(track_id);
                    *paused = false;
                    need
                } else {
                    *track_id_playing = None;
                    *paused = false;
                    false
                }
            });

        if need_update_lyric {
            utils::clear_lyric();

            let title = track_meta
                .title()
                .ok_or(PlayerStatus::Unsupported("cannot get song title"))?;
            let artist = track_meta.artists().map(|arts| arts.join(","));

            let length = track_meta.length();

            let cache = CACHE_LYRICS.with_borrow(|cache| *cache);
            let fetch_result = if cache {
                fetch_lyric_cached(title, artist, length, window)
            } else {
                fetch_lyric(title, artist, length, window)
            };

            if let Err(e) = fetch_result {
                error!("lyric fetch error: {e}");
            }

            get_label(window, false).set_label(DEFAULT_TEXT);
            get_label(window, true).set_label("");
        }

        // sync play position
        let position = player
            .get_position()
            .map_err(|_| PlayerStatus::Unsupported("cannot get playback position"))?;
        let start = SystemTime::now()
            .checked_sub(position)
            .ok_or(PlayerStatus::Unsupported(
                "Position is greater than SystemTime",
            ))?;

        LYRIC_START.set(start);

        Ok(())
    })
}

pub fn register_mpris_sync(app: WeakRef<Application>, interval: Duration) {
    glib::timeout_add_local(interval, move || {
        if let Some(app) = app.upgrade() {
            let windows = app.windows();
            if windows.is_empty() {
                return Continue(true);
            }

            let sync_status = try_sync_player(&windows[0]);

            match sync_status {
                Err(PlayerStatus::Missing) => {
                    PLAYER_FINDER.with_borrow(|player_finder| {
                        if let Ok(player) = player_finder.find_active() {
                            info!("connected to player: {}", player.identity());
                            PLAYER.set(Some(player));
                        } else {
                            PLAYER.set(None);
                        }
                    });
                    get_label(&windows[0], true).set_label(DEFAULT_TEXT);
                    get_label(&windows[0], false).set_label("");
                    TRACK_PLAYING_PAUSED.set((None, false));
                }
                Err(PlayerStatus::Unsupported(kind)) => {
                    get_label(&windows[0], true).set_label("Unsupported Player");
                    get_label(&windows[0], false).set_label("");

                    utils::clear_lyric();
                    error!(kind);
                }
                Err(PlayerStatus::Paused) => {
                    TRACK_PLAYING_PAUSED.with_borrow_mut(|(_, paused)| *paused = true)
                }
                _ => (),
            }
        }
        Continue(true)
    });
}

fn fetch_lyric_cached(
    title: &str,
    artist: Option<String>,
    length: Option<Duration>,
    window: &gtk::Window,
) -> Result<(), Box<dyn std::error::Error>> {
    let digest = md5::compute(format!("{title}-{artist:?}-{length:?}"));
    let cache_dir = CACHE_DIR
        .with_borrow(|cache_home| PathBuf::from(cache_home).join(utils::md5_cache_dir(digest)));
    let cache_path = cache_dir.join(format!("{digest:x}.json"));
    debug!(
        "cache_path for {} - {title} - {length:?}: {cache_path:?}",
        artist.as_deref().unwrap_or("Unknown")
    );

    if let Err(e) = std::fs::create_dir_all(&cache_dir) {
        error!("cannot create cache dir {cache_dir:?}: {e}");
    }

    match std::fs::read_to_string(&cache_path) {
        Ok(lyric) => {
            let cached_lyric: Result<(LyricOwned, LyricOwned), _> = serde_json::from_str(&lyric);
            match cached_lyric {
                Ok(lyrics) => {
                    LYRIC.set(lyrics);
                    return Ok(());
                }
                Err(e) => error!("cache parse error: {e} from {cache_path:?}"),
            }
        }
        Err(e) => info!("cache missed: {e}"),
    }
    let result = fetch_lyric(title, artist, length, window);
    if result.is_ok() {
        LYRIC.with_borrow(|lyric| {
            if let Err(e) = std::fs::write(
                &cache_path,
                serde_json::to_string(lyric).expect("cannot serialize lyrics!"),
            ) {
                error!("cannot write cache {cache_path:?}: {e}");
            } else {
                info!("cached to {cache_path:?}");
            }
        });
    }
    result
}

fn fetch_lyric(
    title: &str,
    artist: Option<String>,
    length: Option<Duration>,
    window: &gtk::Window,
) -> Result<(), Box<dyn std::error::Error>> {
    let provider = NeteaseLyricProvider::new().unwrap();
    let search_result = TOKIO_RUNTIME_HANDLE.with_borrow(|handle| {
        provider.search_song(handle, artist.as_deref().unwrap_or(""), title)
    })?;

    if let Some(song_id) = length
        .and_then(|leng| {
            search_result
                .iter()
                .find(|SongInfo { length, .. }| length == &leng)
        })
        .or(search_result.get(0))
        .map(|song| song.id)
    {
        let lyric =
            TOKIO_RUNTIME_HANDLE.with_borrow(|handle| provider.query_lyric(handle, song_id))?;
        let olyric = lyric.get_lyric().into_owned();
        let tlyric = lyric.get_translated_lyric().into_owned();
        debug!("original lyric: {olyric:?}");
        debug!("translated lyric: {tlyric:?}");

        // show info to user if original lyric is empty or no timestamp
        match &olyric {
            LyricOwned::LineTimestamp(_) => (),
            _ => {
                info!(
                    "No lyric for {} - {title}",
                    artist.as_deref().unwrap_or("Unknown"),
                );
            }
        }

        if let LyricOwned::LineTimestamp(_) = &tlyric {
        } else {
            info!(
                "No translated lyric for {} - {title}",
                artist.as_deref().unwrap_or("Unknown"),
            );
            get_label(window, true).set_visible(false);
        }
        LYRIC.set((olyric, tlyric));
        Ok(())
    } else {
        info!(
            "Failed searching for {} - {title}",
            artist.as_deref().unwrap_or("Unknown"),
        );
        utils::clear_lyric();
        Err("No lyric found".into())
    }
}
