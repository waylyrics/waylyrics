use std::{cell::RefCell, path::PathBuf, sync::OnceLock};

use app::Window;
use lyric_providers::LyricProvider;
use once_cell::sync::Lazy;
use regex::RegexSet;

pub mod app;
pub mod config;
pub mod log;
pub mod lyric_providers;
pub mod sync;
pub mod utils;

pub const DEFAULT_TEXT: &str = "Waylyrics";
pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_ID: &str = "io.poly000.waylyrics";

thread_local! {
    pub static CACHE_DIR: RefCell<String> = const { RefCell::new(String::new()) };
    pub static THEME_PATH: RefCell<PathBuf> = RefCell::new(PathBuf::new());
    pub static EXCLUDED_REGEXES: RefCell<RegexSet> = RefCell::new(RegexSet::empty());
    pub static MAIN_WINDOW: RefCell<Option<Window>> = const { RefCell::new(None) };
}

pub static LYRIC_PROVIDERS: OnceLock<Vec<&'static dyn LyricProvider>> = OnceLock::new();

pub static TOKIO_RUNTIME: Lazy<tokio::runtime::Runtime> =
    Lazy::new(|| tokio::runtime::Runtime::new().unwrap());

#[macro_export]
macro_rules! glib_spawn {
    ($future: expr) => {
        gtk::glib::MainContext::default().spawn_local($future)
    };
}

/// Used with functions requiring reqwest.
#[macro_export]
macro_rules! tokio_spawn {
    ($future: expr) => {
        $crate::TOKIO_RUNTIME.spawn($future)
    };
}

#[cfg(test)]
mod tests;

#[cfg(feature = "tray-icon")]
pub mod tray_icon;
