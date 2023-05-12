#![feature(local_key_cell_methods)]

use std::{cell::RefCell};

pub mod app;
pub mod config;
pub mod lyric;
pub mod sync;
pub mod utils;

pub const APP_ID: &str = "io.poly000.waylyrics";

thread_local! {
    pub static CONFIG_HOME: RefCell<String> = RefCell::new(String::new());
    pub static CACHE_DIR: RefCell<String> = RefCell::new(String::new());
}
