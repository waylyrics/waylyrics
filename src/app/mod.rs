use crate::window::Window;
use gtk::{prelude::*, Application, Label};

const WINDOW_MIN_HEIGHT: i32 = 120;

pub mod utils;

pub fn build_main_window(
    app: &Application,
    full_width_label_bg: bool,
    hide_label_on_empty_text: bool,
    allow_click_through_me: bool,
    origin_lyric_in_above: bool,
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

    if hide_label_on_empty_text {
        olabel.connect_label_notify(utils::hide_on_empty);
        tlabel.connect_label_notify(utils::hide_on_empty);
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
        utils::allow_click_through(&window);
    }

    window.set_icon_name(Some(crate::APP_ID));

    window
}

pub fn get_label(window: &gtk::Window, translated: bool) -> Label {
    let vbox: gtk::Box = window.child().unwrap().downcast().unwrap();
    let first: Label = vbox.first_child().unwrap().downcast().unwrap();
    let last: Label = vbox.last_child().unwrap().downcast().unwrap();

    let name = if translated { "translated" } else { "origin" };

    [first, last]
        .into_iter()
        .find(|label| label.widget_name() == name)
        .unwrap()
}
