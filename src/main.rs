#![feature(local_key_cell_methods)]
#![feature(is_some_and)]

use std::cell::RefCell;
use std::time::{Duration, SystemTime};

use gtk::glib::WeakRef;
use gtk::prelude::*;
use gtk::{glib, Application};

use mpris::{PlaybackStatus, ProgressTracker, TrackID};
use mpris::{Player, PlayerFinder};

use tokio::runtime::Handle;

use tracing::{debug, error, info};
use waylyrics::app::{build_main_window, get_label};
use waylyrics::config::Config;
use waylyrics::lyric::netease::NeteaseLyricProvider;
use waylyrics::lyric::{LyricOwned, LyricProvider, LyricStore, SongInfo};
use waylyrics::utils;

const DEFAULT_TEXT: &str = "Waylyrics";

thread_local! {
    static PLAYER: RefCell<Option<Player>> = RefCell::new(None);
    static PLAYER_FINDER: RefCell<PlayerFinder> = RefCell::new(PlayerFinder::new().unwrap());

    static LYRIC: RefCell<(LyricOwned, LyricOwned)> = RefCell::new((LyricOwned::None, LyricOwned::None));
    static LYRIC_START: RefCell<SystemTime> = RefCell::new(SystemTime::now());

    static TRACK_PLAYING_PAUSED: RefCell<(Option<TrackID>, bool)> = RefCell::new((None, false));

    static TOKIO_RUNTIME_HANDLE: RefCell<Handle> = RefCell::new(Handle::current());
}

#[tokio::main]
async fn main() -> Result<glib::ExitCode, Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app = Application::builder()
        .application_id(waylyrics::APP_ID)
        .build();

    app.connect_activate(build_ui);

    Ok(app.run())
}

enum PlayerStatus {
    Missing,
    Paused,
    Unsupported(&'static str),
}

fn register_mpris_sync(app: WeakRef<Application>, interval: Duration) {
    glib::timeout_add_local(interval, move || {
        if let Some(app) = app.upgrade() {
            let windows = app.windows();
            if windows.is_empty() {
                return Continue(true);
            }
            match PLAYER.with_borrow(|player| {
                let player = player.as_ref();
                if let Some(player) = player {
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
                    let track_meta = player.get_metadata().map_err(|_| {
                        PlayerStatus::Unsupported("cannot get metadata of track playing")
                    })?;
                    let need_update_lyric =
                        TRACK_PLAYING_PAUSED.with_borrow_mut(|(track_id_playing, paused)| {
                            if let Some(track_id) = track_meta.track_id() {
                                let need = track_id_playing.is_none()
                                    || track_id_playing.as_ref().is_some_and(|p| p != &track_id)
                                        && !(*paused
                                            && track_id_playing
                                                .as_ref()
                                                .is_some_and(|p| p == &track_id));

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
                        LYRIC.set((LyricOwned::None, LyricOwned::None));

                        let title = track_meta
                            .title()
                            .ok_or(PlayerStatus::Unsupported("cannot get song title"))?;
                        let artist = track_meta.artists().map(|arts| arts.join(","));

                        let length = track_meta.length();
                        if let Err(e) = fetch_lyric(title, artist, length, &windows[0]) {
                            error!("lyric fetch error: {e}");
                        }

                        get_label(&windows[0], false).set_label(DEFAULT_TEXT);
                        get_label(&windows[0], true).set_label("");
                    }

                    // sync play position
                    let position = player
                        .get_position()
                        .map_err(|_| PlayerStatus::Unsupported("cannot get playback position"))?;
                    let start = SystemTime::now().checked_sub(position).ok_or(
                        PlayerStatus::Unsupported("Position is greater than SystemTime"),
                    )?;
                    LYRIC_START.set(start);
                    Ok(())
                } else {
                    Err(PlayerStatus::Missing)
                }
            }) {
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

                    LYRIC.set((LyricOwned::None, LyricOwned::None));
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
                    artist.as_ref().map(|s| &**s).unwrap_or("Unknown"),
                );
            }
        }

        if let LyricOwned::LineTimestamp(_) = &tlyric {
            get_label(window, true).set_visible(true);
        } else {
            info!(
                "No translated lyric for {} - {title}",
                artist.as_ref().map(|s| &**s).unwrap_or("Unknown"),
            );
            get_label(window, true).set_visible(false);
        }
        LYRIC.set((olyric, tlyric));
        Ok(())
    } else {
        info!(
            "Failed searching for {} - {title}",
            artist.as_ref().map(|s| &**s).unwrap_or("Unknown"),
        );
        LYRIC.set((LyricOwned::None, LyricOwned::None));
        Err("No lyric found".into())
    }
}

fn register_lyric_display(app: WeakRef<Application>, interval: Duration) {
    glib::timeout_add_local(interval, move || {
        if let Some(app) = app.upgrade() {
            let windows = app.windows();
            if windows.is_empty() {
                return Continue(true);
            }
            if TRACK_PLAYING_PAUSED.with_borrow(|(play, paused)| *paused || play.is_none()) {
                // no music is playing
                return Continue(true); // skip lyric scrolling
            }

            LYRIC.with_borrow(|(origin, translation)| {
                let elapsed = LYRIC_START.with_borrow(|start| start.elapsed().ok());
                if let Some(elapsed) = elapsed {
                    if let LyricOwned::LineTimestamp(lyric) = origin {
                        let new_text = lyric.iter().take_while(|(_, off)| off < &elapsed).last();
                        if let Some((text, _time)) = new_text {
                            get_label(&windows[0], false).set_label(text);
                        }
                    }
                    if let LyricOwned::LineTimestamp(lyric) = translation {
                        let new_text = lyric.iter().take_while(|(_, off)| off < &elapsed).last();
                        if let Some((text, _time)) = new_text {
                            get_label(&windows[0], true).set_label(text);
                        }
                    }
                }
            });

            return Continue(true);
        }

        Continue(false)
    });
}

fn build_ui(app: &Application) {
    use utils::parse_time;

    let config = std::fs::read_to_string("config.toml").unwrap();
    let Config {
        mpris_sync_interval,
        lyric_update_interval,
        allow_click_through_me,
        full_width_lyric_bg,
        hide_label_on_empty_text,
        origin_lyric_in_above,
        theme,
    } = toml::from_str(&config).unwrap();

    let mpris_sync_interval = parse_time(&mpris_sync_interval);
    let lyric_update_interval = parse_time(&lyric_update_interval);
    let css_style =
        std::fs::read_to_string(std::path::PathBuf::from("themes").join(&format!("{theme}.css")))
            .unwrap();

    utils::merge_css(&css_style);

    register_mpris_sync(ObjectExt::downgrade(app), mpris_sync_interval);
    register_lyric_display(ObjectExt::downgrade(app), lyric_update_interval);

    build_main_window(
        app,
        full_width_lyric_bg,
        hide_label_on_empty_text,
        allow_click_through_me,
        origin_lyric_in_above,
    );
}
