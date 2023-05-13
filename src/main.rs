#![feature(local_key_cell_methods)]

use std::error::Error;
use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{glib, Application};

use regex::RegexSet;
use waylyrics::app::{self, build_main_window};
use waylyrics::config::Config;
use waylyrics::{utils, EXLUDED_REGEXIES};

use waylyrics::sync::*;

// waylyrics will copy from there if no config file was found in XDG_CONFIG_HOME/waylyrics/
pub const DEFAULT_CONFIG_PATH: &str = env!("WAYLYRICS_DEFAULT_CONFIG");
pub const THEME_PRESETS_DIR: &str = env!("WAYLYRICS_THEME_PRESETS_DIR");

#[tokio::main]
async fn main() -> Result<glib::ExitCode, Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let app = Application::builder()
        .application_id(waylyrics::APP_ID)
        .build();

    app.connect_activate(|app| build_ui(app).unwrap());

    Ok(app.run())
}

fn build_ui(app: &Application) -> Result<(), Box<dyn Error>> {
    use utils::parse_time;

    let xdg_dirs = xdg::BaseDirectories::with_prefix("waylyrics")?;
    let config_home = xdg_dirs.get_config_home();
    let cache_dir = xdg_dirs.get_cache_home();
    waylyrics::CONFIG_HOME.set(
        config_home
            .to_str()
            .ok_or("xdg config home is not valid UTF-8")?
            .into(),
    );
    waylyrics::CACHE_DIR.set(
        cache_dir
            .to_str()
            .ok_or("xdg config home is not valid UTF-8")?
            .into(),
    );

    std::fs::create_dir_all(&config_home)?;
    std::fs::create_dir_all(&cache_dir)?;
    let config_path = config_home.join("config.toml");
    let theme_dir = xdg_dirs.get_data_home().join("themes");

    if !config_path.exists() {
        std::fs::copy(DEFAULT_CONFIG_PATH, &config_path)?;
    }
    if !theme_dir.exists() {
        std::fs::create_dir_all(&theme_dir)?;
        let mut theme_dir_iter =
            std::fs::read_dir(THEME_PRESETS_DIR).expect("cannot access theme presets dir");
        while let Some(Ok(entry)) = theme_dir_iter.next() {
            if entry.file_type().unwrap().is_file()
                && entry.file_name().to_str().unwrap().ends_with(".css")
            {
                let source_file = PathBuf::from(THEME_PRESETS_DIR).join(entry.file_name());
                let dest_file = theme_dir.join(entry.file_name());
                std::fs::copy(source_file, dest_file).unwrap();
            }
        }
    }

    let config = std::fs::read_to_string(config_path)?;
    let Config {
        mpris_sync_interval,
        lyric_update_interval,
        allow_click_through_me,
        full_width_lyric_bg,
        hide_label_on_empty_text,
        origin_lyric_in_above,
        theme,
        cache_lyrics,
        enable_filter_regex,
        filter_regexies,
    } = toml::from_str(&config).unwrap();

    let mpris_sync_interval = parse_time(&mpris_sync_interval)?;
    let lyric_update_interval = parse_time(&lyric_update_interval)?;
    let css_style =
        std::fs::read_to_string(std::path::PathBuf::from("themes").join(format!("{theme}.css")))?;

    app::utils::merge_css(&css_style);

    register_mpris_sync(ObjectExt::downgrade(app), mpris_sync_interval);
    register_lyric_display(ObjectExt::downgrade(app), lyric_update_interval);

    build_main_window(
        app,
        full_width_lyric_bg,
        hide_label_on_empty_text,
        allow_click_through_me,
        origin_lyric_in_above,
        enable_filter_regex && !filter_regexies.is_empty(),
    );

    if enable_filter_regex {
        EXLUDED_REGEXIES.set(RegexSet::new(&filter_regexies)?);
    }

    CACHE_LYRICS.set(cache_lyrics);

    Ok(())
}
