use crate::EXCLUDED_REGEXES;

use gtk::{prelude::*, Label};

use super::window;

#[cfg(target_os = "windows")]
pub(super) fn set_click_pass_through(window: &window::Window, enabled: bool) {
    fn set_window_click_through(hwnd: isize, enabled: bool) {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE,
        };
        let hwnd = HWND(hwnd);

        const WS_EX_TRANSPARENT: isize = 0x00000020;
        const WS_EX_LAYERED: isize = 0x00080000;
        unsafe {
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            if enabled {
                SetWindowLongPtrW(
                    hwnd,
                    GWL_EXSTYLE,
                    ex_style | WS_EX_TRANSPARENT | WS_EX_LAYERED,
                );
            } else {
                SetWindowLongPtrW(
                    hwnd,
                    GWL_EXSTYLE,
                    ex_style & !WS_EX_TRANSPARENT & !WS_EX_LAYERED,
                );
            }
        }
    }

    let Some(surface) = window.surface().and_downcast::<gdk4_win32::Win32Surface>() else {
        return;
    };

    let handle = surface.handle().0;

    set_window_click_through(handle, enabled);
}

#[cfg(not(target_os = "windows"))]
pub(super) fn set_click_pass_through(window: &window::Window, enabled: bool) {
    use gtk::{
        cairo::{RectangleInt, Region},
        subclass::prelude::*,
    };

    let obj = window;
    let Some(surface) = obj.surface() else {
        return;
    };

    if enabled {
        if !window.is_decorated() {
            surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(0, 0, 0, 0)));
        } else {
            let headerbar = &window.imp().headerbar;
            let allocation = headerbar.allocation();

            surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(
                allocation.x(),
                allocation.y(),
                allocation.width(),
                allocation.height(),
            )));
        }
    } else {
        surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(
            0,
            0,
            i32::MAX,
            i32::MAX,
        )));
    }
}

/// set css style for waylyrics
/// As said in [GTK+ doc], gtk constructs style from the lower priority ones to the upper ones,
/// We set priority as `STYLE_PROVIDER_PRIORITY + 1` to override user theme
///
/// [GTK+ doc]: https://docs.gtk.org/gtk4/type_func.StyleContext.add_provider_for_display.html#parameters
pub fn merge_css(css: &str) {
    use gtk::gdk::Display as GdkDisplay;

    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_data(css);

    gtk::style_context_add_provider_for_display(
        &GdkDisplay::default().expect("Could not connect to a display."),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER + 1,
    );
}

fn has_filtered_word(text: &str) -> bool {
    EXCLUDED_REGEXES.with_borrow(|regex_set| regex_set.is_match(text))
}

pub fn setup_label(label: &Label, hide_filtered_words: bool) {
    if hide_filtered_words {
        label.connect_label_notify(|label| {
            let text = label.label();
            let visible = !has_filtered_word(&text) && !text.is_empty();
            label.set_visible(visible);
        });
    } else {
        label.connect_label_notify(|label| {
            let visible = !label.label().is_empty();
            label.set_visible(visible);
        });
    }
}
