use crate::EXLUDED_REGEXIES;

use super::window::Window;
use gtk::prelude::*;

pub fn allow_click_through(window: &Window) {
    use gtk::cairo::{RectangleInt, Region};
    let native = window.native().unwrap();
    let surface = native.surface();
    surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(0, 0, 0, 0)));
}

pub fn hide_on_empty(label: &gtk::Label) {
    if label.label().is_empty() {
        label.set_visible(false);
    } else {
        label.set_visible(true);
    }
}

pub fn hide_exluded_words(label: &gtk::Label) {
    let label_text = label.label();
    let label_text = label_text.as_str();

    if EXLUDED_REGEXIES.with_borrow(|regex_set| regex_set.is_match(label_text)) {
        label.set_visible(false);
    } else {
        label.set_visible(true);
    }
}

pub fn merge_css(css: &str) {
    use gtk::gdk::Display as GdkDisplay;

    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_data(css);

    gtk::style_context_add_provider_for_display(
        &GdkDisplay::default().expect("Could not connect to a display."),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
