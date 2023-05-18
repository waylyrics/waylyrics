use crate::EXLUDED_REGEXIES;

use gtk::{
    cairo::{RectangleInt, Region},
    gdk::Surface,
    prelude::*,
    Label,
};

pub fn set_click_through(surface: &Surface, switch: bool) {
    if switch {
        surface.set_input_region(&Region::create());
    } else {
        surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(
            0,
            0,
            i32::MAX,
            i32::MAX,
        )))
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

pub fn has_filtered_word(text: &str) -> bool {
    EXLUDED_REGEXIES.with_borrow(|regex_set| regex_set.is_match(text))
}

pub fn setup_label(label: &Label, hide_empty_label: bool, hide_filtered_words: bool) {
    match (hide_empty_label, hide_filtered_words) {
        (true, false) => {
            label.connect_label_notify(|label| {
                label.set_visible(!label.label().is_empty());
            });
        }
        (false, true) => {
            label.connect_label_notify(|label| {
                let label_text = label.label();
                let label_text = label_text.as_str();

                label.set_visible(!has_filtered_word(label_text));
            });
        }
        (true, true) => {
            label.connect_label_notify(|label| {
                let label_text = label.label();
                let label_text = label_text.as_str();

                label.set_visible(!has_filtered_word(label_text) && !label.label().is_empty());
            });
        }
        (false, false) => (),
    };
}
