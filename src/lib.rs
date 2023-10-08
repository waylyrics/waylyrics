#![allow(stable_features)]
#![feature(local_key_cell_methods)]
#![feature(is_some_and)]

use std::{cell::RefCell, path::PathBuf, rc::Rc};

use lyric_providers::LyricProvider;
use qqmusic_rs::QQMusicApi;
use regex::RegexSet;

pub mod app;
pub mod config;
pub mod lyric_providers;
pub mod sync;
pub mod utils;

pub const APP_ID: &str = "io.poly000.waylyrics";

thread_local! {
    pub static CONFIG_HOME: RefCell<String> = RefCell::new(String::new());
    pub static CACHE_DIR: RefCell<String> = RefCell::new(String::new());
    pub static THEME_PATH: RefCell<PathBuf> = RefCell::new(PathBuf::new());
    pub static EXCLUDED_REGEXES: RefCell<RegexSet> = RefCell::new(RegexSet::empty());
    pub static LYRIC_PROVIDERS: RefCell<Vec<Rc<dyn LyricProvider>>> = RefCell::new(vec![]);
    pub static QQMUSIC_API_CLIENT: RefCell<Option<QQMusicApi>> = RefCell::new(None);
}

pub const DEFAULT_TEXT: &str = "Waylyrics";
