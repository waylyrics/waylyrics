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

const WINDOW_MIN_HEIGHT: i32 = 120;

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
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    Ok(app.run())
}

enum PlayerStatus {
    Missing,
    Paused,
    Playing,
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
                        let title = track_meta.title().expect("cannot get song title");
                        let artist = track_meta.artists().map(|arts| arts.join(","));

                        let length = track_meta.length();
                        let _ = fetch_lyric(title, artist, length, &windows[0]);

                        get_label(&windows[0], false).set_label(DEFAULT_TEXT);
                        get_label(&windows[0], true).set_label("");
                    }

                    // sync play position
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
                    // 之前的似了，找新欢
                    PLAYER_FINDER.with_borrow(|player_finder| {
                        if let Ok(player) = player_finder.find_active() {
                            PLAYER.set(Some(player));
                        } else {
                            PLAYER.set(None); // 还没新欢可找…… 再等等8
                        }
                    });
                    get_label(&windows[0], true).set_label(DEFAULT_TEXT);
                    get_label(&windows[0], false).set_label("");
                    TRACK_PLAYING_PAUSED.set((None, false));
                }
                PlayerStatus::Paused => {
                    TRACK_PLAYING_PAUSED.with_borrow_mut(|(_, paused)| *paused = true)
                }
                PlayerStatus::Playing => (),
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
        if let LyricOwned::LineTimestamp(_) = &tlyric {
            get_label(window, true).set_visible(true);
        } else {
            get_label(window, true).set_visible(false);
        }
        LYRIC.set((olyric, tlyric));
        Ok(())
    } else {
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
    fn parse_time(time: &str) -> Duration {
        use rust_decimal::prelude::*;
        use rust_decimal_macros::dec;

        let time_ms = if time.ends_with("ms") {
            let sec = time.trim_end_matches("ms");
            Decimal::from_str_exact(sec).unwrap()
        } else if time.ends_with('s') {
            let milli_sec = time.trim_end_matches('s');
            Decimal::from_str_exact(milli_sec).unwrap() * dec!(1000)
        } else {
            panic!("unsupported time format! should be ended with 's' or 'ms'.")
        };
        Duration::from_millis(
            time_ms
                .to_u64()
                .expect("could not represent duration more accurate than ms"),
        )
    }
    let config = std::fs::read_to_string("config.toml").unwrap();
    let Config {
        mpris_sync_interval,
        lyric_update_interval,
        allow_click_through_me,
        full_width_lyric_bg,
    } = toml::from_str(&config).unwrap();

    let mpris_sync_interval = parse_time(&mpris_sync_interval);
    let lyric_update_interval = parse_time(&lyric_update_interval);
    let css_style = std::fs::read_to_string("style.css").unwrap();

    merge_css(&css_style);

    register_mpris_sync(ObjectExt::downgrade(app), mpris_sync_interval);
    register_lyric_display(ObjectExt::downgrade(app), lyric_update_interval);

    let window = build_main_window(app, full_width_lyric_bg);
    if allow_click_through_me {
        allow_click_through(&window);
    }
}

fn allow_click_through(window: &Window) {
    use gtk::cairo::{RectangleInt, Region};
    let native = window.native().unwrap();
    let surface = native.surface();
    surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(0, 0, 0, 0)));
}

fn build_main_window(app: &Application, full_width_label_bg: bool) -> Window {
    let window = Window::new(app);

    window.set_size_request(500, WINDOW_MIN_HEIGHT);
    window.set_title(Some("Waylyrics"));
    window.set_decorated(false);
    window.present();

    let olabel = Label::builder().label("Waylyrics").build();
    let tlabel = Label::builder()
        .label("")
        .name("translated")
        .visible(false)
        .build();

    olabel.set_vexpand(true);
    tlabel.set_vexpand(true);

    if !full_width_label_bg {
        olabel.set_halign(gtk::Align::Center);
        tlabel.set_halign(gtk::Align::Center);
    }

    let verical_box = gtk::Box::builder()
        .baseline_position(gtk::BaselinePosition::Center)
        .orientation(gtk::Orientation::Vertical)
        .build();
    verical_box.set_vexpand(true);
    verical_box.set_valign(gtk::Align::Center);

    let slibing: Option<&gtk::Box> = None;
    verical_box.insert_child_after(&olabel, slibing);
    verical_box.insert_child_after(&tlabel, Some(&olabel));

    window.set_child(Some(&verical_box));

    window
}

fn get_label(window: &gtk::Window, translated: bool) -> Label {
    let vbox: gtk::Box = window.child().unwrap().downcast().unwrap();
    if !translated {
        vbox.first_child().unwrap().downcast().unwrap()
    } else {
        vbox.last_child().unwrap().downcast().unwrap()
    }
}
