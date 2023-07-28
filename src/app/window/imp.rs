use std::cell::Cell;
use std::time::{Duration, SystemTime};

use gio::Settings;
use glib::once_cell::sync::OnceCell;
use glib::signal::Inhibit;
use gtk::gio::MenuItem;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, ApplicationWindow};

use crate::app::utils::set_click_pass_through;

#[derive(Default)]
pub struct Window {
    pub settings: OnceCell<Settings>,
    pub cache_lyrics: Cell<bool>,
    pub lyric_start: Cell<Option<SystemTime>>,
    pub lyric_playing: Cell<Option<Duration>>,
    pub lyric_playing_translation: Cell<Option<Duration>>,
    pub lyric_offset_ms: Cell<i64>,
    pub length_toleration_ms: Cell<u128>,

    pub headerbar: gtk::HeaderBar,
    pub menubutton: gtk::MenuButton,
    pub menu: gio::Menu,
    pub clickthrough: Cell<bool>,
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

        self.headerbar.set_decoration_layout(Some("menu:close"));
        self.menubutton.set_icon_name("open-menu-symbolic");

        let disconnect = MenuItem::new(Some("Disconnect"), Some("app.disconnect"));
        let hide_decoration = MenuItem::new(Some("Hide Decoration"), Some("win.hide_decoration"));

        self.menu.append_item(&disconnect);
        self.menu.append_item(&hide_decoration);

        self.menubutton.set_menu_model(Some(&self.menu));
        self.headerbar.pack_start(&self.menubutton)
    }
}
impl WidgetImpl for Window {
    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline);
        let clickthrough = self.clickthrough.get();
        set_click_pass_through(&self.obj(), clickthrough);
    }
}
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
