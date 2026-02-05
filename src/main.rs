use std::fs;
use std::ops::ControlFlow;
use std::path::PathBuf;
use std::sync::atomic::Ordering;

use gtk::gio::DBusSignalFlags;
use gtk::glib::{OptionArg, OptionFlags};
use gtk::prelude::*;
use gtk::{glib, Application};

use anyhow::Result;

use regex::RegexSet;
use tracing::{error, warn};

use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, Registry};

use waylyrics::app::actions::{
    register_reload_theme, register_set_display_mode, register_set_label, register_set_lyric_align,
    register_switch_decoration, register_switch_passthrough, UIAction, UI_ACTION,
};
use waylyrics::app::{self, build_main_window};

use waylyrics::config::{append_comments, Config, Triggers};
use waylyrics::lyric_providers::qqmusic::QQMusic;
use waylyrics::lyric_providers::utils::get_provider;
use waylyrics::lyric_providers::LyricProvider;

use waylyrics::sync::lyric::fetch::tricks::EXTRACT_TRANSLATED_LYRIC;
use waylyrics::utils::{self, acquire_instance_name, gettext, init_dirs, CUSTOM_CONFIG_PATH};
use waylyrics::{
    EXCLUDED_REGEXES, GTK_DBUS_CONNECTION, LYRIC_PROVIDERS, MAIN_WINDOW, PLAYER_IDENTITY_BLACKLIST,
    PLAYER_NAME_BLACKLIST, THEME_PATH,
};

use waylyrics::sync::*;
use waylyrics::{glib_spawn, log, LYRIC_SEARCH_SKIP};

#[cfg(feature = "action-event")]
use waylyrics::app::actions::init_ui_action_channel;
#[cfg(feature = "tray-icon")]
use waylyrics::tray_icon::start_tray_service;

pub const THEME_PRESETS_DIR: Option<&str> = option_env!("WAYLYRICS_THEME_PRESETS_DIR");

fn main() -> Result<glib::ExitCode> {
    #[cfg(feature = "i18n")]
    let i18n_result = {
        let textdomain = gettextrs::TextDomain::new(waylyrics::PACKAGE_NAME);
        eprintln!("textdomain: {textdomain:#?}");

        #[cfg(target_os = "windows")]
        let result = textdomain.push("../share").init();
        #[cfg(not(target_os = "windows"))]
        let result = textdomain.init();

        result
    };

    let registry = Registry::default()
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env()?,
        )
        .with(fmt::Layer::new());

    #[cfg(feature = "journald")]
    registry.with(tracing_journald::layer()?).init();
    #[cfg(not(feature = "journald"))]
    registry.init();

    std::panic::set_hook(Box::new(tracing_panic::panic_hook));

    #[cfg(feature = "i18n")]
    match i18n_result {
        Err(e) => log::error!("failed to bind textdomain: {e}"),
        Ok(domain) => log::info!(
            "bind to textdomain: {:?}",
            domain.as_ref().map(|s| String::from_utf8_lossy(s))
        ),
    }

    log::info!("process id: {}", std::process::id());

    acquire_instance_name()?;

    let app = Application::builder()
        .application_id(
            waylyrics::INSTANCE_NAME
                .get()
                .expect("Instance name must be set"),
        )
        .build();

    app.connect_handle_local_options(|_app, option| {
        if let Ok(Some(path)) = option.lookup::<PathBuf>("config-path") {
            _ = CUSTOM_CONFIG_PATH.set(path);
        }
        ControlFlow::Continue(())
    });
    app.add_main_option(
        "config-path",
        b'c'.into(),
        OptionFlags::empty(),
        OptionArg::Filename,
        &gettext("[optional] configuration path"),
        None,
    );

    glib::set_prgname(Some(waylyrics::APP_ID_FIXED));

    log::info!("successfully created application!");

    app.connect_activate(|app| {
        if let Err(e) = build_ui(app) {
            log::error!("failed to start: {e}");
        }
    });

    app.connect_startup(|a| {
        let dbus_conn = a.dbus_connection();
        GTK_DBUS_CONNECTION.set(dbus_conn);

        let conn = GTK_DBUS_CONNECTION.with_borrow(|conn| conn.as_ref().cloned());

        let Some(conn) = conn else {
            error!("D-Bus signal initialization failed");
            return;
        };
        glib_spawn!(async move {
            #[allow(deprecated, reason = "upstream bug")]
            conn.signal_subscribe(
                None,
                Some("io.github.waylyrics.Waylyrics"),
                None,
                Some("/io/github/waylyrics/Waylyrics"),
                None,
                DBusSignalFlags::NONE,
                |_conn, _sender, _obj_path, _interface, signal_name, params| {
                    let Some(sender) = UI_ACTION.get() else {
                        return;
                    };
                    match signal_name {
                        "SetAboveLabel" => {
                            let child = params.child_value(0);
                            let Some(text) = child.str() else {
                                warn!("Invalud arguments from dbus signal!");
                                return;
                            };
                            _ = sender.send_blocking(UIAction::SetAboveLabel(text.to_string()));
                        }
                        "SetBelowLabel" => {
                            let child = params.child_value(0);
                            let Some(text) = child.str() else {
                                warn!("Invalud arguments from dbus signal!");
                                return;
                            };
                            _ = sender.send_blocking(UIAction::SetAboveLabel(text.to_string()));
                        }
                        _ => warn!("unknown signal: {signal_name}"),
                    }
                },
            );
        });
    });

    Ok(app.run())
}

fn build_ui(app: &Application) -> Result<()> {
    use utils::parse_time;

    let (config_path, theme_dir) = init_dirs()?;

    log::info!("config path: {:?}", config_path);
    let config = std::fs::read_to_string(&config_path)?;
    let config: Config = toml_edit::de::from_str(&config)?;
    let config_with_docs = append_comments(&toml::to_string(&config)?)?;
    fs::write(config_path, config_with_docs)?;

    #[cfg_attr(windows, allow(unused))]
    let Config {
        auto_connect,
        player_sync_interval,
        lyric_update_interval,
        theme,
        cache_lyrics,
        enable_filter_regex,
        filter_regexies,
        ref length_toleration,
        triggers,
        lyric_search_source,
        skip_auto_search,
        show_default_text_on_idle,
        show_lyric_on_pause,
        #[cfg(feature = "tray-icon")]
        show_tray_icon,
        player_name_blacklist,
        player_identity_blacklist,
        enable_local_lyric,
        extract_translated_lyric,
        qqmusic,
        color_scheme,
        theme_dark_switch,
    } = config;

    LYRIC_SEARCH_SKIP.store(skip_auto_search, Ordering::Release);

    #[cfg(feature = "tray-icon")]
    if show_tray_icon {
        let result = start_tray_service();
        log::info!("tray-icon status: {result:?}");
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

    #[cfg(not(windows))]
    utils::auto_theme_change(color_scheme, theme_dark_switch);

    let enable_filter_regex = enable_filter_regex && !filter_regexies.is_empty();
    let length_toleration_ms = parse_time(length_toleration)?.as_millis();
    let wind = build_main_window(
        app,
        enable_filter_regex,
        cache_lyrics,
        length_toleration_ms,
        show_default_text_on_idle,
        show_lyric_on_pause,
    );

    register_sync_task(
        ObjectExt::downgrade(&wind),
        player_sync_interval,
        auto_connect,
    );
    register_lyric_display(ObjectExt::downgrade(&wind), lyric_update_interval);
    register_actions(app, &wind, triggers);

    #[cfg(feature = "action-event")]
    init_play_action_channel(ObjectExt::downgrade(app));
    #[cfg(feature = "action-event")]
    init_ui_action_channel(ObjectExt::downgrade(app), ObjectExt::downgrade(&wind));

    if enable_filter_regex {
        EXCLUDED_REGEXES.set(RegexSet::new(&filter_regexies)?);
    }

    QQMusic.init(&serde_json::to_string(&qqmusic)?)?;

    setup_providers(lyric_search_source);

    #[cfg(target_os = "windows")]
    // * workaround for a GTK4 bug:
    // GTK4 will freeze on close request on windows
    // so we just exit without actually call gtk_window_close
    wind.connect_close_request(|wind| {
        let save_state = wind.save_window_state();
        log::info!("window state save status: {save_state:?}");
        std::process::exit(0);
    });

    let _ = ENABLE_LOCAL_LYRIC.set(enable_local_lyric);
    let _ = EXTRACT_TRANSLATED_LYRIC.set(extract_translated_lyric);

    MAIN_WINDOW.set(Some(wind));
    PLAYER_IDENTITY_BLACKLIST.set(player_identity_blacklist);
    PLAYER_NAME_BLACKLIST.set(player_name_blacklist);

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
    register_set_label(wind);
    register_reload_theme(app, wind, &reload_theme);
    register_search_lyric(app, wind, &search_lyric);
    register_remove_lyric(app, wind);
    register_reload_lyric(app);
    register_refetch_lyric(app, wind, &refetch_lyric);
    #[cfg(feature = "import-lyric")]
    register_import_lyric(app, wind);
    #[cfg(feature = "export-lyric")]
    register_export_lyric(app, wind);
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
