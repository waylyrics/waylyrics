#![feature(local_key_cell_methods)]
#![feature(is_some_and)]

use gtk::prelude::*;
use gtk::{glib, Application};

use waylyrics::app::{self, build_main_window};
use waylyrics::config::Config;
use waylyrics::utils::{self};

use waylyrics::sync::*;

#[tokio::main]
async fn main() -> Result<glib::ExitCode, Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app = Application::builder()
        .application_id(waylyrics::APP_ID)
        .build();

    app.connect_activate(build_ui);

    Ok(app.run())
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
        cache_lyrics,
    } = toml::from_str(&config).unwrap();

    let mpris_sync_interval = parse_time(&mpris_sync_interval);
    let lyric_update_interval = parse_time(&lyric_update_interval);
    let css_style =
        std::fs::read_to_string(std::path::PathBuf::from("themes").join(format!("{theme}.css")))
            .unwrap();

    app::utils::merge_css(&css_style);

    register_mpris_sync(ObjectExt::downgrade(app), mpris_sync_interval);
    register_lyric_display(ObjectExt::downgrade(app), lyric_update_interval);

    build_main_window(
        app,
        full_width_lyric_bg,
        hide_label_on_empty_text,
        allow_click_through_me,
        origin_lyric_in_above,
    );

    CACHE_LYRICS.set(cache_lyrics);
}
