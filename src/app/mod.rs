pub mod search_window;
mod window;

use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{Application, Label};
pub use window::Window;

use crate::app::utils::set_click_pass_through;
use crate::{config, DEFAULT_TEXT};

const WINDOW_MIN_HEIGHT: i32 = 120;

pub mod actions;
pub mod dialog;
pub mod utils;

pub fn build_main_window(
    app: &Application,
    enable_filter_regex: bool,
    cache_lyrics: bool,
    length_toleration_ms: u128,
    show_default_text_on_idle: bool,
    show_lyric_on_pause: bool,
) -> Window {
    let window = Window::new(
        app,
        cache_lyrics,
        length_toleration_ms,
        show_default_text_on_idle,
        show_lyric_on_pause,
    );

    window.set_size_request(500, WINDOW_MIN_HEIGHT);
    window.set_title(Some(DEFAULT_TEXT));
    window.set_icon_name(Some(crate::APP_ID_FIXED));
    window.present();

    let above_label = Label::builder()
        .label("Waylyrics")
        .name("above")
        .vexpand(true)
        .hexpand(false)
        .build();
    let below_label = Label::builder()
        .label("")
        .name("below")
        .vexpand(true)
        .hexpand(false)
        .visible(false)
        .build();

    for label in [&above_label, &below_label] {
        utils::setup_label(label, enable_filter_regex);
    }

    let verical_box = gtk::Box::builder()
        .name("lyrics-box")
        .baseline_position(gtk::BaselinePosition::Center)
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .vexpand(true)
        .hexpand(false)
        .build();

    verical_box.insert_child_after(&above_label, gtk::Box::NONE);
    verical_box.insert_child_after(&below_label, Some(&above_label));

    window.set_child(Some(&verical_box));

    let align = window.imp().lyric_align.get();
    set_lyric_align(&window, align);

    window.connect_decorated_notify(|window| {
        crate::log::debug!("triggered decorated signal");
        let clickthrough = window.imp().clickthrough.get();
        set_click_pass_through(window, clickthrough)
    });

    window.set_icon_name(Some(crate::APP_ID_FIXED));

    window
}

pub fn set_lyric_align(window: &Window, align: config::Align) -> Option<()> {
    let labels = get_labels(window)?;
    for label in labels {
        label.set_halign(align.into());
    }
    window.imp().lyric_align.set(align);
    Some(())
}

fn get_labels(window: &Window) -> Option<[Label; 2]> {
    let vbox: gtk::Box = window.child()?.downcast().ok()?;
    let above_label: Label = vbox.first_child()?.downcast().ok()?;
    let below_label: Label = vbox.last_child()?.downcast().ok()?;
    Some([above_label, below_label])
}

pub fn get_label(window: &Window, position: &str) -> Label {
    get_labels(window)
        .expect("cannot find labels")
        .into_iter()
        .find(|label| label.widget_name() == position)
        .unwrap()
}
