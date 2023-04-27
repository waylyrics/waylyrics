#![feature(local_key_cell_methods)]

use std::cell::RefCell;
use std::time::{Duration, SystemTime};

use gtk::{glib, Application, Label};
use gtk::{prelude::*, CssProvider};

use mpris::{Player, PlayerFinder};

use tokio::runtime::Handle;

use waylyrics::config::Config;
use waylyrics::lyric::{self, LyricProvider};

use window::Window;
mod window;

pub const APP_ID: &str = "io.poly000.waylyrics";

const WINDOW_HEIGHT: i32 = 120;
const UPDATE_INTERVAL_SEC: u64 = 3;

thread_local! {
    static PLAYER: RefCell<Option<Player>> = RefCell::new(None);
}

#[tokio::main]
async fn main() -> Result<glib::ExitCode, Box<dyn std::error::Error>> {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_font_size());
    app.connect_activate(build_ui);
    let app_ = ObjectExt::downgrade(&app);

    let player_finder = PlayerFinder::new()?;
    let player = player_finder.find_active()?;
    PLAYER.set(Some(player));

    glib::timeout_add_local(Duration::from_secs(UPDATE_INTERVAL_SEC), move || {
        if let Some(app) = app_.upgrade() {
            if let Some(window) = app.windows().get(0) {
                let label: Label = window.first_child().unwrap().downcast().unwrap();
                label.set_label(&format!("{:?}", SystemTime::now()));

                return Continue(true);
            }
        }

        Continue(false)
    });

    let ncmlyric = lyric::netease::NeteaseLyricProvider::new()?;

    let handle = Handle::current();
    ncmlyric.query_lyric(handle, 1968702735)?;

    Ok(app.run())
}

fn load_font_size() {
    let css = r#"
        label {
            font-size: 50px;
        }
    "#;

    add_css(css);
}

fn add_css(css: &str) {
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
    } = read_config("config.toml").unwrap();

    add_css(&format!(
        r#"label {{
            color: Rgba{:?};
        }}"#,
        text_color
    ));
    add_css(&format!(
        "window {{ background-color: Rgba{:?}; }}",
        background_color
    ));

    let window = Window::new(app);

    window.set_size_request(500, WINDOW_HEIGHT);

    window.set_title(Some("Waylyrics"));

    window.set_decorated(false);
    window.present();

    let label = Label::builder().label("Waylyrics").build();

    window.set_child(Some(&label));

    use gtk::cairo::{RectangleInt, Region};
    let native = window.native().unwrap();
    let surface = native.surface();
    surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(0, 0, 0, 0)));
}

fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let conf = std::fs::read(path)?;
    let conf = String::from_utf8(conf)?;
    Ok(toml::from_str(&conf)?)
}
