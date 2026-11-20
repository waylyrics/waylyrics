pub mod search_window;
mod window;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{Application, Label};
use glib_macros::clone;
pub use window::Window;

use crate::app::utils::set_click_pass_through;
use crate::app::utils::set_skip_taskbar;
use crate::{config, DEFAULT_TEXT};

const WINDOW_MIN_HEIGHT: i32 = 120;

pub mod actions;
pub mod dialog;
pub mod utils;

mod label;
pub use label::VerticalLabel;
#[cfg(test)]
pub(crate) use label::{
    column_offset, compute_fallback_vertical_layout, compute_vertical_layout, group_offset,
    group_width, setup_vertical_context,
};

#[derive(Debug)]
pub struct LyricWidgets {
    pub above: Label,
    pub below: Label,
    pub vertical_above: VerticalLabel,
    pub vertical_below: VerticalLabel,
}

const WINDOW_MIN_WIDTH: i32 = 500;
const VERTICAL_MIN_WIDTH: i32 = 120;
const VERTICAL_MIN_HEIGHT: i32 = 240;

pub fn window_min_size(align: config::Align) -> (i32, i32) {
    if align.is_vertical() {
        (VERTICAL_MIN_WIDTH, VERTICAL_MIN_HEIGHT)
    } else {
        (WINDOW_MIN_WIDTH, WINDOW_MIN_HEIGHT)
    }
}

pub fn build_main_window(
    app: &Application,
    enable_filter_regex: bool,
    cache_lyrics: bool,
    length_toleration_ms: u128,
    show_default_text_on_idle: bool,
    show_lyric_on_pause: bool,
    respect_empty_line_as_gap: bool,
    skip_auto_search: bool,
    skip_taskbar: bool,
    #[cfg(feature = "layer-shell")] layer_shell: bool,
    #[cfg(feature = "layer-shell")] layer_shell_anchor: crate::config::LayerShellAnchor,
) -> Window {
    let window = Window::new(
        app,
        cache_lyrics,
        length_toleration_ms,
        show_default_text_on_idle,
        show_lyric_on_pause,
        respect_empty_line_as_gap,
        skip_auto_search,
    );

    #[cfg(feature = "layer-shell")]
    if layer_shell {
        use gtk4_layer_shell::{is_supported, KeyboardMode, Layer, LayerShell};

        if is_supported() {
            LayerShell::init_layer_shell(&window);
            // wlr-layer-shell-unstable-v1 version 4 needed, see
            // https://wayland.app/protocols/wlr-layer-shell-unstable-v1#compositor-support
            // and https://wayland.app/protocols/wlr-layer-shell-unstable-v1#zwlr_layer_surface_v1:enum:keyboard_interactivity
            LayerShell::set_keyboard_mode(&window, KeyboardMode::OnDemand);
            LayerShell::set_layer(&window, Layer::Overlay);
        } else {
            tracing::warn!("layer-shell was enabled but unsupported by the compositor!");
        }

        layer_shell_anchor.setup(&window);
    }

    let (min_width, min_height) = window_min_size(window.imp().lyric_align.get());
    window.set_size_request(min_width, min_height);
    window.set_title(Some(DEFAULT_TEXT));
    window.set_icon_name(Some(crate::APP_ID_FIXED));

    if skip_taskbar {
        // the hint needs a `GdkSurface` to exist, and it has to be applied
        // before the window is mapped, otherwise the window manager has
        // already registered its taskbar entry
        gtk::prelude::WidgetExt::realize(&window);
        set_skip_taskbar(&window, true);
    }

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

    let vertical_above = VerticalLabel::new("above");
    let vertical_below = VerticalLabel::new("below");
    for label in [&vertical_above, &vertical_below] {
        label.set_visible(false);
        label.set_vexpand(true);
        label.set_hexpand(false);
    }
    mirror_label(&above_label, &vertical_above);
    mirror_label(&below_label, &vertical_below);

    window
        .imp()
        .lyric_widgets
        .set(LyricWidgets {
            above: above_label.clone(),
            below: below_label.clone(),
            vertical_above,
            vertical_below,
        })
        .expect("lyric widgets must only be initialized once");

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

fn sync_vertical_box_empty(label: &VerticalLabel) {
    let Some(parent) = label.parent().and_downcast::<gtk::Box>() else {
        return;
    };
    if !parent.has_css_class("vertical-lyrics") {
        return;
    }
    let mut has_visible = false;
    let mut child = parent.first_child();
    while let Some(widget) = child {
        if widget.is_visible() {
            has_visible = true;
            break;
        }
        child = widget.next_sibling();
    }
    if has_visible {
        parent.remove_css_class("empty");
    } else {
        parent.add_css_class("empty");
    }
}

fn mirror_label(source: &Label, target: &VerticalLabel) {
    target.set_text(&source.label());
    target.set_visible(source.is_visible());

    source.connect_label_notify(clone!(
        #[weak]
        target,
        move |source| {
            target.set_text(&source.label());
            target.set_visible(source.is_visible());
            sync_vertical_box_empty(&target);
        }
    ));
    source.connect_visible_notify(clone!(
        #[weak]
        target,
        move |source| {
            target.set_visible(source.is_visible());
            sync_vertical_box_empty(&target);
        }
    ));
}

pub fn set_lyric_align(window: &Window, align: config::Align) -> Option<()> {
    let vbox: gtk::Box = window.child()?.downcast().ok()?;
    let LyricWidgets {
        above,
        below,
        vertical_above,
        vertical_below,
    } = window.imp().lyric_widgets.get()?;

    crate::log::debug!(
        "set_lyric_align: {align:?}, default={:?}, current={}x{}",
        window.default_size(),
        window.width(),
        window.height()
    );

    for widget in [
        above.clone().upcast::<gtk::Widget>(),
        below.clone().upcast::<gtk::Widget>(),
        vertical_above.clone().upcast::<gtk::Widget>(),
        vertical_below.clone().upcast::<gtk::Widget>(),
    ] {
        if widget.parent().is_some() {
            widget.unparent();
        }
    }

    if align.is_vertical() {
        let rtl = align.is_rtl();

        vbox.set_orientation(gtk::Orientation::Horizontal);
        vbox.add_css_class("vertical-lyrics");
        vbox.set_halign(if rtl {
            gtk::Align::End
        } else {
            gtk::Align::Start
        });
        vbox.set_valign(gtk::Align::Center);
        vbox.set_spacing(4);

        for label in [vertical_above, vertical_below] {
            label.set_rtl(rtl);
            label.set_vexpand(false);
            label.set_valign(gtk::Align::Center);
            label.set_hexpand(false);
        }

        vertical_above.set_text(&above.label());
        vertical_above.set_visible(above.is_visible());
        vertical_below.set_text(&below.label());
        vertical_below.set_visible(below.is_visible());

        if rtl {
            vbox.append(vertical_below);
            vbox.append(vertical_above);
        } else {
            vbox.append(vertical_above);
            vbox.append(vertical_below);
        }
        if vertical_above.is_visible() || vertical_below.is_visible() {
            vbox.remove_css_class("empty");
        } else {
            vbox.add_css_class("empty");
        }
    } else {
        vbox.remove_css_class("vertical-lyrics");
        vbox.remove_css_class("empty");
        vbox.set_orientation(gtk::Orientation::Vertical);
        vbox.set_halign(align.into());
        vbox.set_valign(gtk::Align::Center);
        vbox.set_spacing(0);

        for label in [above, below] {
            label.set_halign(align.into());
            label.set_justify(align.into());
            label.set_vexpand(true);
            label.set_hexpand(false);
        }

        vbox.append(above);
        vbox.append(below);
        vertical_above.set_visible(false);
        vertical_below.set_visible(false);
    }

    let (min_width, min_height) = window_min_size(align);
    window.set_size_request(min_width, min_height);

    window.imp().lyric_align.set(align);
    window.queue_resize();
    Some(())
}

pub fn get_label(window: &Window, position: &str) -> Label {
    let widgets = window
        .imp()
        .lyric_widgets
        .get()
        .expect("lyric widgets must be initialized");
    match position {
        "above" => widgets.above.clone(),
        "below" => widgets.below.clone(),
        _ => panic!("unknown lyric position: {position}"),
    }
}
