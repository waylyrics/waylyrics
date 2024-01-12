mod window;
use std::cell::Cell;

use gtk::{prelude::*, subclass::prelude::ObjectSubclassIsExt, Align, Application, Label};
pub use window::Window;

use crate::{app::utils::set_click_pass_through, config::LyricDisplay, DEFAULT_TEXT};

const WINDOW_MIN_HEIGHT: i32 = 120;

pub mod dialog;
pub mod utils;

thread_local! {
    static REMOVE_LYRICS: Cell<String> = const { Cell::new(String::new()) };
}

pub fn build_main_window(
    app: &Application,
    hide_label_on_empty_text: bool,
    click_pass_through: bool,
    enable_filter_regex: bool,
    cache_lyrics: bool,
    length_toleration_ms: u128,
    lyric_align: impl Into<Align> + Copy,
    lyric_display_mode: LyricDisplay,
) -> Window {
    REMOVE_LYRICS.set(
        if cache_lyrics {
            "Remove lyric permanently"
        } else {
            "Remove lyric"
        }
        .to_string(),
    );

    let window = Window::new(app, click_pass_through, cache_lyrics);

    window.set_size_request(500, WINDOW_MIN_HEIGHT);
    window.set_title(Some(DEFAULT_TEXT));
    window.present();

    let above_label = Label::builder().label("Waylyrics").name("above").build();
    let below_label = Label::builder()
        .label("")
        .name("below")
        .visible(false)
        .build();

    for label in [&above_label, &below_label] {
        utils::setup_label(label, hide_label_on_empty_text, enable_filter_regex);
    }
    above_label.set_vexpand(true);
    below_label.set_vexpand(true);

    above_label.set_halign(lyric_align.into());
    below_label.set_halign(lyric_align.into());

    let verical_box = gtk::Box::builder()
        .baseline_position(gtk::BaselinePosition::Center)
        .orientation(gtk::Orientation::Vertical)
        .build();
    verical_box.set_vexpand(true);
    verical_box.set_valign(gtk::Align::Center);

    verical_box.insert_child_after(&above_label, gtk::Box::NONE);
    verical_box.insert_child_after(&below_label, Some(&above_label));

    window.set_child(Some(&verical_box));

    window.connect_decorated_notify(|window| {
        crate::log::debug!("triggered decorated signal");
        let clickthrough = window.imp().clickthrough.get();
        set_click_pass_through(window, clickthrough)
    });

    window.set_icon_name(Some(crate::APP_ID));
    window.imp().length_toleration_ms.set(length_toleration_ms);
    window.imp().lyric_display_mode.set(lyric_display_mode);
    window
}

pub fn get_label(window: &Window, position: &str) -> Label {
    let vbox: gtk::Box = window.child().unwrap().downcast().unwrap();
    let first: Label = vbox.first_child().unwrap().downcast().unwrap();
    let last: Label = vbox.last_child().unwrap().downcast().unwrap();

    [first, last]
        .into_iter()
        .find(|label| label.widget_name() == position)
        .unwrap()
}
