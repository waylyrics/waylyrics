use anyhow::Result;
use gtk::glib::subclass::types::ObjectSubclassIsExt;
use std::path::PathBuf;
use std::time::Duration;

use crate::app::{get_label, Window};
use crate::config::Config;
use crate::DEFAULT_TEXT;

pub fn gettext(msg: impl Into<String>) -> String {
    #[cfg(feature = "i18n")]
    return gettextrs::gettext(msg.into());
    #[cfg(not(feature = "i18n"))]
    return msg.into();
}

pub fn reset_lyric_labels(window: &Window, tip: Option<&str>) {
    let tip = tip.unwrap_or(if window.imp().show_default_text_on_idle.get() {
        DEFAULT_TEXT
    } else {
        ""
    });

    get_label(window, "above").set_label(tip);
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

pub fn init_dirs() -> Result<(PathBuf, PathBuf)> {
    let proj_dirs =
        directories::ProjectDirs::from("io", "poly000", "waylyrics").expect("can't get proj_dirs!");
    let config_home = proj_dirs.config_dir();
    let cache_dir = proj_dirs.cache_dir();

    crate::CACHE_DIR.set(
        cache_dir
            .to_str()
            .expect("xdg config home is not valid UTF-8")
            .into(),
    );

    std::fs::create_dir_all(config_home)?;
    std::fs::create_dir_all(cache_dir)?;
    let config_path = config_home.join("config.toml");
    let user_theme_dir = proj_dirs.data_dir().join("_themes");

    if !config_path.exists() {
        std::fs::write(&config_path, toml::to_string(&Config::default())?)?;
    }
    if !user_theme_dir.exists() {
        std::fs::create_dir_all(&user_theme_dir)?;
    }

    Ok((config_path, user_theme_dir))
}

mod shortcut;
pub use shortcut::bind_shortcut;
mod theme;
pub use theme::auto_theme_change;
