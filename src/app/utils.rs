use crate::window::Window;
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