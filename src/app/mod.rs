mod window;
use gtk::{prelude::*, Application, Label};
use window::Window;

const WINDOW_MIN_HEIGHT: i32 = 120;

pub mod utils;

pub fn build_main_window(
    app: &Application,
    full_width_label_bg: bool,
    hide_label_on_empty_text: bool,
    allow_click_through_me: bool,
    origin_lyric_in_above: bool,
    enable_filter_regex: bool,
) -> Window {
    let window = Window::new(app);

    window.set_size_request(500, WINDOW_MIN_HEIGHT);
    window.set_title(Some("Waylyrics"));
    window.set_decorated(false);
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

    if allow_click_through_me {
        utils::set_click_through(&window.surface())
    }

    window.set_icon_name(Some(crate::APP_ID));
    window
}

pub fn get_label(window: &gtk::Window, translation: bool) -> Label {
    let vbox: gtk::Box = window.child().unwrap().downcast().unwrap();
    let first: Label = vbox.first_child().unwrap().downcast().unwrap();
    let last: Label = vbox.last_child().unwrap().downcast().unwrap();

    let name = if translation { "translated" } else { "origin" };

    [first, last]
        .into_iter()
        .find(|label| label.widget_name() == name)
        .unwrap()
}
