use std::time::Duration;

use gtk::cairo::{RectangleInt, Region};
use gtk::prelude::*;
use gtk::{glib, Application};

use tokio::runtime::Handle;
use waylyrics::config::Config;
use waylyrics::lyric::{self, LyricProvider};
use window::Window;

pub const APP_ID: &str = "io.poly000.waylyrics";

mod window;

#[tokio::main]
async fn main() {
    let app = Application::builder().application_id(APP_ID).build();
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

fn build_ui(app: &Application) {
    let Config {
        text: waylyrics::config::Text { .. },
    } = read_config("config.toml").unwrap();

    let window = Window::new(app);

    window.set_size_request(250, 100);
    window.set_title(Some("Waylyrics"));

    // Present window
    window.set_decorated(false);
    window.present();

    let native = window.native().unwrap();
    let surface = native.surface();
    surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(0, 0, 0, 0)));
}

fn read_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let conf = std::fs::read(path)?;
    let conf = String::from_utf8(conf)?;
    Ok(toml::from_str(&conf)?)
}
