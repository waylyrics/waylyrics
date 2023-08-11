#![feature(local_key_cell_methods)]

use std::path::PathBuf;

use gtk::prelude::*;
use gtk::{glib, Application};

use anyhow::Result;

use regex::RegexSet;
use waylyrics::app::{self, build_main_window};
use waylyrics::config::{Config, Triggers};
use waylyrics::{utils, EXCLUDED_REGEXES, THEME_PATH};

use waylyrics::sync::*;

pub const THEME_PRESETS_DIR: &str = env!("WAYLYRICS_THEME_PRESETS_DIR");

fn main() -> Result<glib::ExitCode> {
    tracing_subscriber::fmt::init();
    tracing::info!("process id: {}", std::process::id());

    let app = Application::builder()
        .application_id(waylyrics::APP_ID)
        .build();

    app.connect_activate(|app| build_ui(app).unwrap());
    register_sigusr1_disconnect();

    Ok(app.run())
}

fn build_ui(app: &Application) -> Result<()> {
    use utils::parse_time;

    let (config_path, theme_dir) = init_dirs()?;

    let config = std::fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&config).unwrap();
    std::fs::write(&config_path, toml::to_string(&config)?)?;
    let Config {
        mpris_sync_interval,
        lyric_update_interval,
        click_pass_through,
        hide_label_on_empty_text,
        theme,
        cache_lyrics,
        enable_filter_regex,
        filter_regexies,
        ref length_toleration,
        lyric_align,
        triggers:
            Triggers {
                switch_decoration,
                reload_theme,
                reload_lyric,
                switch_passthrough,
            },
    } = config;

    let mpris_sync_interval = parse_time(&mpris_sync_interval)?;
    let lyric_update_interval = parse_time(&lyric_update_interval)?;

    let theme_file_name = format!("{theme}.css");
    let user_theme = theme_dir.join(&theme_file_name);
    let global_theme = PathBuf::from(THEME_PRESETS_DIR).join(&theme_file_name);

    let theme_path = if user_theme.exists() {
        user_theme
    } else {
        global_theme
    };

    let css_style = std::fs::read_to_string(&theme_path)?;
    app::utils::merge_css(&css_style);
    THEME_PATH.set(theme_path);

    register_mpris_sync(ObjectExt::downgrade(app), mpris_sync_interval);
    register_lyric_display(ObjectExt::downgrade(app), lyric_update_interval);
    utils::register_sigusr2_decoration(ObjectExt::downgrade(app));
    register_action_disconnect(app);
    register_action_connect(app);

    let wind = build_main_window(
        app,
        hide_label_on_empty_text,
        click_pass_through,
        enable_filter_regex && !filter_regexies.is_empty(),
        cache_lyrics,
        parse_time(&length_toleration)?.as_millis(),
        lyric_align,
    );

    utils::register_action_switch_decoration(&wind, &switch_decoration);
    utils::register_action_switch_passthrough(&wind, &switch_passthrough);
    utils::register_action_reload_theme(app, &wind, &reload_theme);
    register_action_reload_lyric(app, &wind, &reload_lyric);

    if enable_filter_regex {
        EXCLUDED_REGEXES.set(RegexSet::new(&filter_regexies)?);
    }

    Ok(())
}

fn init_dirs() -> Result<(PathBuf, PathBuf)> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("waylyrics")?;
    let config_home = xdg_dirs.get_config_home();
    let cache_dir = xdg_dirs.get_cache_home();
    waylyrics::CONFIG_HOME.set(
        config_home
            .to_str()
            .expect("xdg config home is not valid UTF-8")
            .into(),
    );
    waylyrics::CACHE_DIR.set(
        cache_dir
            .to_str()
            .expect("xdg config home is not valid UTF-8")
            .into(),
    );

    std::fs::create_dir_all(&config_home)?;
    std::fs::create_dir_all(&cache_dir)?;
    let config_path = config_home.join("config.toml");
    let user_theme_dir = xdg_dirs.get_data_home().join("_themes");

    if !config_path.exists() {
        std::fs::write(&config_path, toml::to_string(&Config::default())?)?;
    }
    if !user_theme_dir.exists() {
        std::fs::create_dir_all(&user_theme_dir)?;
    }

    Ok((config_path, user_theme_dir))
}
