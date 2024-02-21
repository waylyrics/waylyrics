mod imp;

use gio::Settings;
use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application};

use crate::config::Align;
use crate::APP_ID;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application, clickthrough: bool, cache_lyrics: bool) -> Self {
        let window: Self = Object::builder().property("application", app).build();
        let imp = window.imp();
        window.set_titlebar(Some(&imp.headerbar));
        imp.clickthrough.set(clickthrough);
        imp.cache_lyrics.set(cache_lyrics);
        window.set_widget_name("main-window");
        window
    }

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    pub fn save_window_state(&self) -> Result<(), glib::BoolError> {
        let (width, height) = self.default_size();
        let decorated = self.is_decorated();
        let align_mode = self.imp().lyric_align.get().to_string();

        self.settings().set_int("window-width", width)?;
        self.settings().set_int("window-height", height)?;
        self.settings().set_boolean("window-decorated", decorated)?;
        self.settings()
            .set_string("lyric-align-mode", &align_mode)?;

        Ok(())
    }

    fn load_window_state(&self) {
        let height = self.settings().int("window-height");
        let width = self.settings().int("window-width");
        let decorated = self.settings().boolean("window-decorated");
        let align_mode: Align = self.settings().string("lyric-align-mode").parse().unwrap();

        self.set_default_size(width, height);
        self.set_decorated(decorated);
        self.imp().lyric_align.set(align_mode);
    }
}
