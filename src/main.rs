use std::time::Duration;

use gtk::cairo::{RectangleInt, Region};
use gtk::gdk::Display as GdkDisplay;
use gtk::{glib, Application, Label};
use gtk::{prelude::*, CssProvider};

use tokio::runtime::Handle;
use waylyrics::config::Config;
use waylyrics::lyric::{self, LyricProvider};
use window::Window;

pub const APP_ID: &str = "io.poly000.waylyrics";

const WINDOW_HEIGHT: i32 = 120;

mod window;

#[tokio::main]
async fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    let app_ = app.downgrade();

    glib::timeout_add_local(Duration::from_secs(3), move || {
        if let Some(app) = app_.upgrade() {
            for window in app.windows() {
                println!("{window:?}");
            }
        }
        Continue(true)
    });

    let ncmlyric = lyric::netease::NeteaseLyricProvider::new().unwrap();

    let handle = Handle::current();
    ncmlyric.query_lyric(handle, 1968702735).unwrap();

    app.run();
}

fn load_css() {
    let css = r#"
        window {
            background-color: rgba(0, 0, 0, 0)
        }
        label {
            font-size: 50px
        }
    "#;

    add_css(css);
}

fn add_css(css: &str) {
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
        text: waylyrics::config::Text { color },
    } = read_config("config.toml").unwrap();

    add_css(&format!("label {{color: Rgba{:?}}}", color));

    let window = Window::new(app);

    window.set_size_request(500, WINDOW_HEIGHT);

    window.set_title(Some("Waylyrics"));

    window.set_decorated(false);
    window.present();

    let label = Label::builder().label("Hello world!").build();

    window.set_child(Some(&label));

    let native = window.native().unwrap();
    let surface = native.surface();
    surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(0, 0, 0, 0)));
}

fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let conf = std::fs::read(path)?;
    let conf = String::from_utf8(conf)?;
    Ok(toml::from_str(&conf)?)
}
