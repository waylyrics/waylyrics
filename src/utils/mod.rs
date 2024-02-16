use anyhow::Result;
use std::path::PathBuf;
use std::time::Duration;

use crate::app::{get_label, Window};
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

pub fn init_dirs() -> Result<(PathBuf, PathBuf)> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("waylyrics")?;
    let config_home = xdg_dirs.get_config_home();
    let cache_dir = xdg_dirs.get_cache_home();
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
