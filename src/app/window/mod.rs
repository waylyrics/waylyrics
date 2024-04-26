mod imp;

use gio::Settings;
use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, Application};

use crate::config::{Align, LyricDisplayMode};
use crate::APP_ID;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(
        app: &Application,
        cache_lyrics: bool,
        length_toleration_ms: u128,
        show_default_text_on_idle: bool,
        show_lyric_on_pause: bool,
    ) -> Self {
        let window: Self = Object::builder().property("application", app).build();
        let imp = window.imp();

        window.set_widget_name("main-window");

        imp.cache_lyrics.set(cache_lyrics);
        imp.length_toleration_ms.set(length_toleration_ms);
        imp.show_default_text_on_idle.set(show_default_text_on_idle);
        imp.show_lyric_on_pause.set(show_lyric_on_pause);

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
        let click_through = self.imp().clickthrough.get();
        let align_mode = self.imp().lyric_align.get().to_string();
        let display_mode = self.imp().lyric_display_mode.get().to_string();

        self.settings().set_int("window-width", width)?;
        self.settings().set_int("window-height", height)?;
        self.settings().set_boolean("window-decorated", decorated)?;
        self.settings()
            .set_boolean("window-click-through", click_through)?;
        self.settings()
            .set_string("lyric-align-mode", &align_mode)?;
        self.settings()
            .set_string("lyric-display-mode", &display_mode)?;

        Ok(())
    }

    fn load_window_state(&self) {
        let height = self.settings().int("window-height");
        let width = self.settings().int("window-width");
        let decorated = self.settings().boolean("window-decorated");
        let click_through = self.settings().boolean("window-click-through");
        let align_mode: Align = self.settings().string("lyric-align-mode").parse().unwrap();
        let display_mode: LyricDisplayMode = self
            .settings()
            .string("lyric-display-mode")
            .parse()
            .unwrap();

        self.set_default_size(width, height);
        self.set_decorated(decorated);
        self.imp().clickthrough.set(click_through);
        self.imp().lyric_align.set(align_mode);
        self.imp().lyric_display_mode.set(display_mode)
    }
}
