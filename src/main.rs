#![feature(local_key_cell_methods)]

use std::cell::RefCell;
use std::time::{Duration, SystemTime};

use gtk::glib::WeakRef;
use gtk::{glib, Application, Label};
use gtk::{prelude::*, CssProvider};

use mpris::{PlaybackStatus, ProgressTracker, TrackID};
use mpris::{Player, PlayerFinder};

use tokio::runtime::Handle;

use waylyrics::config::Config;
use waylyrics::lyric::netease::NeteaseLyricProvider;
use waylyrics::lyric::{LyricOwned, LyricProvider, LyricStore, SongInfo};

use window::Window;
mod window;

pub const APP_ID: &str = "io.poly000.waylyrics";

const WINDOW_HEIGHT: i32 = 120;

const TRACK_PLAT_SYNC_INTERVAL_SEC: u64 = 3;
const LYRIC_UPDATE_INTERVAL_MS: u64 = 100;

const DEFAULT_TEXT: &str = "Waylyrics";

thread_local! {
    static PLAYER: RefCell<Option<Player>> = RefCell::new(None);
    static PLAYER_FINDER: RefCell<PlayerFinder> = RefCell::new(PlayerFinder::new().unwrap());

    static LYRIC: RefCell<LyricOwned> = RefCell::new(LyricOwned::None);
    static LYRIC_START: RefCell<SystemTime> = RefCell::new(SystemTime::now());

    static TRACK_PLAYING_PAUSED: RefCell<(Option<TrackID>, bool)> = RefCell::new((None, false));

    static TOKIO_RUNTIME_HANDLE: RefCell<Handle> = RefCell::new(Handle::current());
}

#[tokio::main]
async fn main() -> Result<glib::ExitCode, Box<dyn std::error::Error>> {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    register_mpris_sync(ObjectExt::downgrade(&app));
    register_lyric_display(ObjectExt::downgrade(&app));

    Ok(app.run())
}

enum PlayerStatus {
    Missing,
    Paused,
    Playing,
}

fn register_mpris_sync(app: WeakRef<Application>) {
    glib::timeout_add_local(
        Duration::from_secs(TRACK_PLAT_SYNC_INTERVAL_SEC),
        move || {
            if let Some(app) = app.upgrade() {
                let windows = app.windows();
                if windows.len() < 1 {
                    return Continue(true);
                }
                match PLAYER.with_borrow(|player| {
                    let player = player.as_ref();
                    if let Some(player) = player {
                        if !player.is_running() {
                            return PlayerStatus::Missing;
                        }

                        let mut progress_tracker =
                            ProgressTracker::new(player, 0).expect("cannot fetch progress");

                        let progress_tick = progress_tracker.tick();
                        if progress_tick.progress.playback_status() != PlaybackStatus::Playing {
                            return PlayerStatus::Paused;
                        }
                        let track_meta = player
                            .get_metadata()
                            .expect("cannot get metadata of track playing");
                        let need_update_lyric =
                            TRACK_PLAYING_PAUSED.with_borrow_mut(|(track_id_playing, paused)| {
                                if let Some(track_id) = track_meta.track_id() {
                                    let need = track_id_playing.is_none()
                                        || track_id_playing
                                            .as_ref()
                                            .is_some_and(|p| p != &track_id)
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
                            let title = track_meta.title().expect("cannot get song title");
                            let artist = track_meta.artists().map(|arts| arts.join(","));

                            let length = track_meta.length();
                            if fetch_lyric(title, artist, length).is_err() {
                                let label: Label = windows[0].child().unwrap().downcast().unwrap();
                                label.set_label(DEFAULT_TEXT);
                            }
                        }

                        let position = player.get_position().expect("cannot get playback position");
                        let start = SystemTime::now()
                            .checked_sub(position)
                            .expect("Position is greater than SystemTime");
                        LYRIC_START.set(start);
                        PlayerStatus::Playing
                    } else {
                        PlayerStatus::Missing
                    }
                }) {
                    PlayerStatus::Missing => {
                        PLAYER_FINDER.with_borrow(|player_finder| {
                            if let Ok(player) = player_finder.find_active() {
                                PLAYER.set(Some(player));
                            } else {
                                PLAYER.set(None);
                            }
                        });
                        let label: Label = windows[0].child().unwrap().downcast().unwrap();
                        label.set_label(DEFAULT_TEXT);
                        TRACK_PLAYING_PAUSED.set((None, false));
                    }
                    PlayerStatus::Paused => {
                        TRACK_PLAYING_PAUSED.with_borrow_mut(|(_, paused)| *paused = true)
                    }
                    PlayerStatus::Playing => (),
                }
            }
            Continue(true)
        },
    );
}

fn fetch_lyric(
    title: &str,
    artist: Option<String>,
    length: Option<Duration>,
) -> Result<(), Box<dyn std::error::Error>> {
    let provider = NeteaseLyricProvider::new().unwrap();
    let search_result = TOKIO_RUNTIME_HANDLE.with_borrow(|handle| {
        provider.search_song(handle, artist.as_ref().map(|s| &**s).unwrap_or(""), title)
    })?;

    if let Some(song_id) = length
        .map(|leng| {
            search_result
                .iter()
                .find(|SongInfo { length, .. }| length == &leng)
        })
        .flatten()
        .or(search_result.get(0))
        .map(|song| song.id)
    {
        let lyric =
            TOKIO_RUNTIME_HANDLE.with_borrow(|handle| provider.query_lyric(handle, song_id))?;
        let olyric = lyric.get_lyric().into_owned();
        let tlyric = lyric.get_translated_lyric().into_owned();
        let lyric = merge_lyric(&olyric, &tlyric).unwrap_or(olyric);
        LYRIC.set(lyric);
        Ok(())
    } else {
        LYRIC.set(LyricOwned::None);
        Err("No lyric found".into())
    }
}

fn register_lyric_display(app: WeakRef<Application>) {
    glib::timeout_add_local(Duration::from_millis(LYRIC_UPDATE_INTERVAL_MS), move || {
        if let Some(app) = app.upgrade() {
            let windows = app.windows();
            if windows.len() < 1 {
                return Continue(true);
            }
            if TRACK_PLAYING_PAUSED.with_borrow(|(play, paused)| *paused || play.is_none()) {
                // no music is playing
                return Continue(true); // skip lyric scrolling
            }

            LYRIC.with_borrow(|lyric| {
                if let LyricOwned::LineTimestamp(lyric) = lyric {
                    let elapsed = LYRIC_START.with_borrow(|start| start.elapsed().ok());
                    if let Some(elapsed) = elapsed {
                        let new_text = lyric.iter().take_while(|(_, off)| off < &elapsed).last();
                        if let Some((text, _time)) = new_text {
                            let label: Label = windows[0].child().unwrap().downcast().unwrap();
                            label.set_label(text);
                        }
                    }
                }
            });

            return Continue(true);
        }

        Continue(false)
    });
}

fn merge_lyric(lyric1: &LyricOwned, lyric2: &LyricOwned) -> Option<LyricOwned> {
    let left = match lyric1 {
        LyricOwned::LineTimestamp(v) => v,
        _ => return None,
    };
    let right = match lyric2 {
        LyricOwned::LineTimestamp(v) => v,
        _ => return None,
    };

    // 翻译歌词可能会少作曲信息，因此不能直接成对。为了避免为此存储两份歌词...
    let right_start = (&right[0].1).clone();

    Some(LyricOwned::LineTimestamp(
        left.iter()
            .skip_while(|(_, off)| off != &right_start)
            .zip(right.iter())
            .map(|((text, off), (text_, _))| {
                let mut text = text.clone();
                text.push('\n');
                text += &text_;
                (text, *off)
            })
            .collect(),
    ))
}

fn merge_css(css: &str) {
    use gtk::gdk::Display as GdkDisplay;

    let css_provider = CssProvider::new();
    css_provider.load_from_data(css);
    gtk::style_context_add_provider_for_display(
        &GdkDisplay::default().expect("Could not connect to a display."),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let Config {
        text_color,
        background_color,
        font_size,
        font_family,
    } = read_config("config.toml").unwrap();

    merge_css(&format!(
        r#"
        label {{
            font-size: {font_size}px;
            color: Rgba{text_color:?};
        }}
        window {{
            background-color: Rgba{background_color:?};
        }}
        "#,
    ));
    if let Some(font_family) = font_family {
        merge_css(&format!(
            r#"
            label {{
                font-family: {font_family};
            }} 
        "#
        ))
    }

    let window = build_main_window(app);
    allow_click_through(&window);
}

fn allow_click_through(window: &Window) {
    use gtk::cairo::{RectangleInt, Region};
    let native = window.native().unwrap();
    let surface = native.surface();
    surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(0, 0, 0, 0)));
}

fn build_main_window(app: &Application) -> Window {
    let window = Window::new(app);

    window.set_size_request(500, WINDOW_HEIGHT);
    window.set_title(Some("Waylyrics"));
    window.set_decorated(false);
    window.present();

    let label = Label::builder()
        .label("Waylyrics")
        .justify(gtk::Justification::Center)
        .build();

    window.set_child(Some(&label));

    window
}

fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let conf = std::fs::read(path)?;
    let conf = String::from_utf8(conf)?;
    Ok(toml::from_str(&conf)?)
}
