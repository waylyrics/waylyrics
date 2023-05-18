use std::cell::Cell;
use std::time::{Duration, SystemTime};

use gio::Settings;
use glib::once_cell::sync::OnceCell;
use glib::signal::Inhibit;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, ApplicationWindow};

#[derive(Default)]
pub struct Window {
    pub settings: OnceCell<Settings>,
    pub cache_lyrics: Cell<bool>,
    pub lyric_start: Cell<Option<SystemTime>>,
    pub lyric_playing: Cell<Option<Duration>>,
    pub lyric_playing_translation: Cell<Option<Duration>>,
    pub lyric_offset_ms: Cell<i64>,
    pub length_toleration_ms: Cell<u128>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "GtkAppWindowSaveState";
    type Type = super::Window;
    type ParentType = ApplicationWindow;
}
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        // Load latest window state
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_size();
    }
}
impl WidgetImpl for Window {}
impl WindowImpl for Window {
    // Save window state right before the window will be closed
    fn close_request(&self) -> Inhibit {
        // Save window size
        self.obj()
            .save_window_size()
            .expect("Failed to save window state");

        // Don't inhibit the default handler
        Inhibit(false)
    }
}
impl ApplicationWindowImpl for Window {}
