#![feature(local_key_cell_methods)]

use std::cell::RefCell;
use std::time::{Duration, SystemTime};

use gtk::glib::WeakRef;
use gtk::{glib, Application, Label};
use gtk::{prelude::*, CssProvider};

use mpris::{Player, PlayerFinder};

use tokio::runtime::Handle;

use waylyrics::config::Config;
use waylyrics::lyric::{self, LyricOwned, LyricProvider, LyricStore};

use window::Window;
mod window;

pub const APP_ID: &str = "io.poly000.waylyrics";

const WINDOW_HEIGHT: i32 = 120;

const TRACK_PLAT_SYNC_INTERVAL_SEC: u64 = 10;
const LYRIC_UPDATE_INTERVAL_MS: u64 = 100;

thread_local! {
    static PLAYER: RefCell<Option<Player>> = RefCell::new(None);
    static LYRIC: RefCell<LyricOwned> = RefCell::new(LyricOwned::None);
    static LYRIC_START: RefCell<SystemTime> = RefCell::new(SystemTime::now());
}

#[tokio::main]
async fn main() -> Result<glib::ExitCode, Box<dyn std::error::Error>> {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    init_player()?;
    register_lyric_updater(ObjectExt::downgrade(&app));

    let ncmlyric = lyric::netease::NeteaseLyricProvider::new()?;

    let handle = Handle::current();
    let lyric = ncmlyric.query_lyric(&handle, 1968702735)?;
    let (lyric, tlyric) = (
        lyric.get_lyric().into_owned(),
        lyric.get_translated_lyric().into_owned(),
    );

    let merged = merge_lyric(lyric, tlyric).unwrap();
    LYRIC.set(merged);

    LYRIC_START.set(SystemTime::now());

    Ok(app.run())
}

fn init_player() -> Result<(), Box<dyn std::error::Error>> {
    let player_finder = PlayerFinder::new()?;
    let player = player_finder.find_active()?;
    PLAYER.set(Some(player));
    Ok(())
}

fn register_lyric_updater(app: WeakRef<Application>) {
    glib::timeout_add_local(Duration::from_millis(LYRIC_UPDATE_INTERVAL_MS), move || {
        if let Some(app) = app.upgrade() {
            if let Some(window) = app.windows().get(0) {
                let label: Label = window.first_child().unwrap().downcast().unwrap();

                LYRIC.with_borrow(|lyric| {
                    if let LyricOwned::LineTimestamp(lyric) = lyric {
                        let elapsed = LYRIC_START.with_borrow(|start| start.elapsed().ok());
                        if let Some(elapsed) = elapsed {
                            if let Some((text, _)) =
                                lyric.iter().take_while(|(_, off)| off < &elapsed).last()
                            {
                                label.set_label(text);
                            }
                        }
                    }
                });

                return Continue(true);
            }
        }

        Continue(false)
    });
}

fn merge_lyric(lyric1: LyricOwned, lyric2: LyricOwned) -> Option<LyricOwned> {
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
        left.into_iter()
            .skip_while(|(_, off)| off != &right_start)
            .zip(right.into_iter())
            .filter(|((_, t1), (_, t2))| format!("{t1:?}") == format!("{t2:?}"))
            .map(|((mut text, off), (text_, _))| {
                text.push('\n');
                text += &text_;
                (text, off)
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
        merge_css(&format!(r#"
            label {{
                font-family: {font_family};
            }} 
        "#))
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
