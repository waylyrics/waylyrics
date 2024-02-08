use anyhow::Result;
use gtk::gio::SimpleAction;
use gtk::{
    prelude::*, subclass::prelude::*, Application, NamedAction, Shortcut, ShortcutController,
    ShortcutTrigger,
};
use std::path::PathBuf;
use std::time::Duration;

use crate::app::{get_label, utils::set_click_pass_through, Window};
use crate::config::Config;
use crate::DEFAULT_TEXT;

pub fn reset_lyric_labels(window: &Window) {
    get_label(window, "above").set_label(DEFAULT_TEXT);
    get_label(window, "below").set_label("");
}

pub fn parse_time(time: &str) -> Result<Duration, ParseError> {
    use rust_decimal::prelude::*;
    use rust_decimal_macros::dec;

    let time_ms = if time.ends_with("ms") {
        let sec = time.trim_end_matches("ms");
        Decimal::from_str_exact(sec)?
    } else if time.ends_with('s') {
        let milli_sec = time.trim_end_matches('s');
        Decimal::from_str_exact(milli_sec)? * dec!(1000)
    } else {
        return Err(ParseError::IllFormed);
    };

    Ok(Duration::from_millis(
        time_ms.to_u64().ok_or(ParseError::ExceedsLimits)?,
    ))
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("{0}")]
    InvalidDecimal(#[from] rust_decimal::Error),

    #[error("could not represent duration more accurate than ms")]
    ExceedsLimits,

    #[error("unsupported time format! should be ended with 's' or 'ms'.")]
    IllFormed,
}

pub fn register_action_switch_decoration(wind: &Window, switch_decoration_trigger: &str) {
    let action = SimpleAction::new("switch-decoration", None);
    let _wind = Window::downgrade(wind);
    action.connect_activate(move |_, _| {
        if let Some(wind) = _wind.upgrade() {
            wind.set_decorated(!wind.is_decorated());
        }
    });
    wind.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("win.switch-decoration"))
        .trigger(&ShortcutTrigger::parse_string(switch_decoration_trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn register_action_reload_theme(app: &Application, wind: &Window, trigger: &str) {
    let action = SimpleAction::new("reload-theme", None);
    action.connect_activate(move |_, _| {
        crate::THEME_PATH.with_borrow(|theme_path| {
            if let Ok(style) = std::fs::read_to_string(theme_path) {
                crate::app::utils::merge_css(&style);
            }
        });
    });
    app.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("app.reload-theme"))
        .trigger(&ShortcutTrigger::parse_string(trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn register_action_switch_passthrough(wind: &Window, trigger: &str) {
    let action = SimpleAction::new("switch-passthrough", None);
    let _wind = Window::downgrade(wind);
    action.connect_activate(move |_, _| {
        if let Some(wind) = _wind.upgrade() {
            let clickthrough = !wind.imp().clickthrough.get();
            wind.imp().clickthrough.set(clickthrough);
            set_click_pass_through(&wind, clickthrough);
            wind.present();
        }
    });
    wind.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("win.switch-passthrough"))
        .trigger(&ShortcutTrigger::parse_string(trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn init_dirs() -> Result<(PathBuf, PathBuf)> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("waylyrics")?;
    let config_home = xdg_dirs.get_config_home();
    let cache_dir = xdg_dirs.get_cache_home();
    let _ = crate::CONFIG_HOME.set(
        config_home
            .to_str()
            .expect("xdg config home is not valid UTF-8")
            .into(),
    );
    crate::CACHE_DIR.set(
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
