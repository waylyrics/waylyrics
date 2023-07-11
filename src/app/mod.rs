mod window;
use gtk::{prelude::*, subclass::prelude::ObjectSubclassIsExt, Application, Label};
pub use window::Window;

const WINDOW_MIN_HEIGHT: i32 = 120;

pub mod utils;

pub fn build_main_window(
    app: &Application,
    full_width_label_bg: bool,
    hide_label_on_empty_text: bool,
    click_pass_through: bool,
    origin_lyric_in_above: bool,
    enable_filter_regex: bool,
    cache_lyrics: bool,
    length_toleration_ms: u128,
    window_decoration: bool,
) -> Window {
    let window = Window::new(app);

    window.set_size_request(500, WINDOW_MIN_HEIGHT);
    window.set_title(Some("Waylyrics"));
    window.set_decorated(window_decoration);
    window.present();

    let olabel = Label::builder().label("Waylyrics").name("origin").build();
    let tlabel = Label::builder()
        .label("")
        .name("translated")
        .visible(false)
        .build();

    for label in [&olabel, &tlabel] {
        utils::setup_label(label, hide_label_on_empty_text, enable_filter_regex);
    }
    olabel.set_vexpand(true);
    tlabel.set_vexpand(true);

    if !full_width_label_bg {
        olabel.set_halign(gtk::Align::Center);
        tlabel.set_halign(gtk::Align::Center);
    }

    let verical_box = gtk::Box::builder()
        .baseline_position(gtk::BaselinePosition::Center)
        .orientation(gtk::Orientation::Vertical)
        .build();
    verical_box.set_vexpand(true);
    verical_box.set_valign(gtk::Align::Center);

    let slibing: Option<&gtk::Box> = None;
    verical_box.insert_child_after(&olabel, slibing);
    verical_box.insert_child_after(&tlabel, Some(&olabel));

    if !origin_lyric_in_above {
        verical_box.reorder_child_after(&olabel, Some(&tlabel));
    }

    window.set_child(Some(&verical_box));

    if click_pass_through {
        utils::set_click_pass_through(&window.surface(), true)
    }

    window.set_icon_name(Some(crate::APP_ID));
    window.imp().cache_lyrics.set(cache_lyrics);
    window.imp().length_toleration_ms.set(length_toleration_ms);
    window
}

pub fn get_label(window: &Window, translation: bool) -> Label {
    let vbox: gtk::Box = window.child().unwrap().downcast().unwrap();
    let first: Label = vbox.first_child().unwrap().downcast().unwrap();
    let last: Label = vbox.last_child().unwrap().downcast().unwrap();

    let name = if translation { "translated" } else { "origin" };

    [first, last]
        .into_iter()
        .find(|label| label.widget_name() == name)
        .unwrap()
}
