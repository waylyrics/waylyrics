use std::path::Path;

use crate::EXCLUDED_REGEXES;

use gtk::prelude::*;
use gtk::Label;

use super::window;

#[cfg(target_os = "windows")]
pub(super) fn set_click_pass_through(window: &window::Window, enabled: bool) {
    use std::ffi::c_void;

    fn set_window_click_through(hwnd: *mut c_void, enabled: bool) {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE,
        };
        let hwnd = HWND(hwnd as _);

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
    use gtk::cairo::{RectangleInt, Region};
    use gtk::subclass::prelude::*;

    let obj = window;
    let Some(surface) = obj.surface() else {
        return;
    };

    if enabled {
        if !window.is_decorated() {
            // https://github.com/lianchengwu/wmplayer-lyric/blob/158d66e1800ce535414f2758c5f8290ea3e87f2d/osdlyric/osd_lyrics_lib.c#L943-L959
            // Empty input region will imply that we don't handle any mouse event.
            surface.set_input_region(Some(&Region::create_rectangle(&RectangleInt::new(
                0, 0, 1, 1,
            ))));
        } else {
            let headerbar = &window.imp().headerbar;
            let allocation = headerbar.allocation();

            surface.set_input_region(Some(&Region::create_rectangle(&RectangleInt::new(
                allocation.x(),
                allocation.y(),
                allocation.width(),
                allocation.height(),
            ))));
        }
    } else {
        surface.set_input_region(Some(&Region::create_rectangle(&RectangleInt::new(
            0,
            0,
            i32::MAX,
            i32::MAX,
        ))));
    }
}

/// Keep the main window out of the taskbar.
///
/// The hint has to be applied after the widget is realized (so a
/// `GdkSurface` exists) but before the window is presented, otherwise the
/// window manager has already added its taskbar entry.
#[cfg(target_os = "windows")]
pub(super) fn set_skip_taskbar(window: &window::Window, skip: bool) {
    use std::ffi::c_void;

    fn set_window_skip_taskbar(hwnd: *mut c_void, skip: bool) {
        use windows::Win32::Foundation::HWND;
        use windows::Win32::UI::WindowsAndMessaging::{
            GetWindowLongPtrW, SetWindowLongPtrW, GWL_EXSTYLE,
        };
        let hwnd = HWND(hwnd as _);

        // a window carrying `WS_EX_TOOLWINDOW` and no `WS_EX_APPWINDOW`
        // is left out of the taskbar
        const WS_EX_TOOLWINDOW: isize = 0x0000_0080;
        const WS_EX_APPWINDOW: isize = 0x0004_0000;
        unsafe {
            let ex_style = GetWindowLongPtrW(hwnd, GWL_EXSTYLE);
            let new_style = if skip {
                (ex_style | WS_EX_TOOLWINDOW) & !WS_EX_APPWINDOW
            } else {
                ex_style & !WS_EX_TOOLWINDOW
            };
            if new_style != ex_style {
                SetWindowLongPtrW(hwnd, GWL_EXSTYLE, new_style);
            }
        }
    }

    let Some(surface) = window.surface().and_downcast::<gdk4_win32::Win32Surface>() else {
        return;
    };

    let handle = surface.handle().0;

    set_window_skip_taskbar(handle, skip);
}

/// macOS has neither the EWMH hint nor the win32 extended style.
#[cfg(target_os = "macos")]
pub(super) fn set_skip_taskbar(_window: &window::Window, _skip: bool) {}

#[cfg(all(unix, not(target_os = "macos")))]
pub(super) fn set_skip_taskbar(window: &window::Window, skip: bool) {
    if !skip {
        return;
    }

    let Some(surface) = window.surface() else {
        tracing::warn!("skip-taskbar: the window has no surface yet");
        return;
    };

    if let Some(surface) = surface.downcast_ref::<gdk4_x11::X11Surface>() {
        // deprecated since GTK 4.18, but still the only way to ask an X11
        // window manager to keep the window out of the taskbar
        #[allow(deprecated)]
        {
            surface.set_skip_taskbar_hint(true);
            surface.set_skip_pager_hint(true);
        }
        tracing::info!("skip-taskbar: _NET_WM_STATE_SKIP_TASKBAR was set (X11)");
        return;
    }

    // Wayland: xdg-shell has no "keep me out of the taskbar" request, the
    // compositor decides on its own. Only a layer shell surface is exempt,
    // and that has to be asked for explicitly through `layer_shell`, because
    // a layer surface can no longer be moved or resized by the user.
    tracing::warn!(
        "skip-taskbar is enabled, but Wayland gives a client no way to ask for \
         this. Either set `layer-shell = true` (needs the `layer-shell` feature \
         and `LD_PRELOAD=/usr/lib/libgtk4-layer-shell.so`; note that a layer \
         surface can no longer be dragged or resized), or hide the taskbar \
         entry with a window rule in your compositor."
    );
}

/// set css style for waylyrics
/// As said in [GTK+ doc], gtk constructs style from the lower priority ones to the upper ones,
/// We set priority as `STYLE_PROVIDER_PRIORITY + 1` to override user theme
///
/// [GTK+ doc]: https://docs.gtk.org/gtk4/type_func.StyleContext.add_provider_for_display.html#parameters
pub fn merge_css(css: &Path) {
    use gtk::gdk::Display as GdkDisplay;
    use gtk::CssProvider;
    use std::cell::RefCell;

    thread_local! {
        static LATEST_PROVIDER: RefCell<Option<CssProvider>> = const { RefCell::new(None) };
    }
    let css_provider = CssProvider::new();
    css_provider.load_from_path(css);
    let display = GdkDisplay::default().expect("Could not connect to a display.");
    LATEST_PROVIDER.with_borrow_mut(|provider| {
        if let Some(provider) = provider.take() {
            gtk::style_context_remove_provider_for_display(&display, &provider);
        }
    });

    gtk::style_context_add_provider_for_display(
        &display,
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_USER + 1,
    );
    LATEST_PROVIDER.with_borrow_mut(|provider| {
        *provider = Some(css_provider);
    });
}

fn has_filtered_word(text: &str) -> bool {
    EXCLUDED_REGEXES.with_borrow(|regex_set| regex_set.is_match(text))
}

pub fn setup_label(label: &Label, hide_filtered_words: bool) {
    label.set_wrap(true);
    label.set_wrap_mode(gtk::pango::WrapMode::Word);

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
