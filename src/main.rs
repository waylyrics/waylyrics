use std::fs;
use std::path::PathBuf;

use gettextrs::TextDomain;
use gtk::prelude::*;
use gtk::{glib, Application};

use anyhow::Result;

use regex::RegexSet;
use waylyrics::app::{self, build_main_window};
use waylyrics::config::append_comments;
use waylyrics::config::{Config, Triggers};
use waylyrics::lyric_providers::qqmusic::QQMusic;
use waylyrics::lyric_providers::utils::get_provider;
use waylyrics::lyric_providers::LyricProvider;
use waylyrics::utils::init_dirs;
use waylyrics::{utils, EXCLUDED_REGEXES, LYRIC_PROVIDERS, MAIN_WINDOW, PACKAGE_NAME, THEME_PATH};

use waylyrics::log;
use waylyrics::sync::*;

#[cfg(feature = "action-event")]
use waylyrics::app::actions::init_ui_action_channel;
#[cfg(feature = "tray-icon")]
use waylyrics::tray_icon::start_tray_service;

use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, Registry};

use app::actions::{
    register_reload_theme, register_set_display_mode, register_set_lyric_align,
    register_switch_decoration, register_switch_passthrough,
};

pub const THEME_PRESETS_DIR: Option<&str> = option_env!("WAYLYRICS_THEME_PRESETS_DIR");

fn main() -> Result<glib::ExitCode> {
    let _ = TextDomain::new(PACKAGE_NAME).init();

    Registry::default()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()?,
        )
        .with(fmt::Layer::new())
        .with(tracing_journald::layer()?)
        .init();

    log::info!("process id: {}", std::process::id());

    let app = Application::builder()
        .application_id(waylyrics::APP_ID)
        .build();

    app.connect_activate(|app| {
        if let Err(e) = build_ui(app) {
            log::error!("failed to start: {e}");
        }
    });

    Ok(app.run())
}

fn build_ui(app: &Application) -> Result<()> {
    use utils::parse_time;

    let (config_path, theme_dir) = init_dirs()?;

    log::debug!("config path: {:?}", config_path);
    let config = std::fs::read_to_string(&config_path)?;
    let config: Config = toml_edit::de::from_str(&config)?;
    let config_with_docs = append_comments(&toml::to_string(&config)?)?;
    fs::write(config_path, config_with_docs)?;

    let Config {
        player_sync_interval,
        lyric_update_interval,
        theme,
        cache_lyrics,
        enable_filter_regex,
        filter_regexies,
        ref length_toleration,
        triggers,
        qqmusic_api_base_url,
        lyric_search_source,
        show_default_text_on_idle,
        #[cfg(feature = "tray-icon")]
        show_tray_icon,
    } = config;

    #[cfg(feature = "tray-icon")]
    if show_tray_icon {
        start_tray_service();
    }

    let player_sync_interval = parse_time(&player_sync_interval)?;
    let lyric_update_interval = parse_time(&lyric_update_interval)?;

    let theme_file_name = format!("{theme}.css");
    let user_theme = theme_dir.join(&theme_file_name);
    let global_theme = THEME_PRESETS_DIR.map(|d| PathBuf::from(d).join(&theme_file_name));

    let theme_path = if user_theme.exists() {
        user_theme
    } else {
        let Some(global_theme) = global_theme else {
            anyhow::bail!("theme {theme_file_name} not found");
        };
        global_theme
    };

    log::debug!("theme path: {:?}", theme_path);
    let css_style = fs::read_to_string(&theme_path)?;
    app::utils::merge_css(&css_style);
    THEME_PATH.set(theme_path);

    let wind = build_main_window(
        app,
        enable_filter_regex && !filter_regexies.is_empty(),
        cache_lyrics,
        parse_time(length_toleration)?.as_millis(),
        show_default_text_on_idle,
    );

    register_sync_task(ObjectExt::downgrade(app), player_sync_interval);
    register_lyric_display(ObjectExt::downgrade(app), lyric_update_interval);
    register_actions(app, &wind, triggers);

    #[cfg(feature = "action-event")]
    init_play_action_channel(ObjectExt::downgrade(app));
    #[cfg(feature = "action-event")]
    init_ui_action_channel(ObjectExt::downgrade(app), ObjectExt::downgrade(&wind));

    if enable_filter_regex {
        EXCLUDED_REGEXES.set(RegexSet::new(&filter_regexies)?);
    }

    if let Some(base_url) = qqmusic_api_base_url {
        QQMusic.init(&base_url)?;
    }

    setup_providers(lyric_search_source);

    MAIN_WINDOW.set(Some(wind));

    Ok(())
}

fn register_actions(
    app: &Application,
    wind: &app::Window,
    Triggers {
        switch_decoration,
        switch_passthrough,
        reload_theme,
        search_lyric,
        refetch_lyric,
    }: Triggers,
) {
    register_connect(app);
    register_disconnect(app);
    register_set_lyric_align(wind);
    register_set_display_mode(wind);
    register_switch_decoration(wind, &switch_decoration);
    register_switch_passthrough(wind, &switch_passthrough);
    register_reload_theme(app, wind, &reload_theme);
    register_search_lyric(app, wind, &search_lyric);
    register_remove_lyric(app, wind);
    register_refetch_lyric(app, wind, &refetch_lyric);
}

fn setup_providers(providers_enabled: Vec<String>) {
    let mut providers = vec![];
    for source in providers_enabled {
        if let Some(provider) = get_provider(&source) {
            providers.push(provider);
        }
    }
    let _ = LYRIC_PROVIDERS.set(providers);
}

#[cfg(feature = "mimalloc")]
mod _alloc {
    use mimalloc::MiMalloc;

    #[global_allocator]
    static GLOBAL: MiMalloc = MiMalloc;
}
