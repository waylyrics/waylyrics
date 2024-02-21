pub mod search_window;
mod window;

use gtk::{prelude::*, subclass::prelude::ObjectSubclassIsExt, Application, Label};
pub use window::Window;

use crate::{
    app::utils::set_click_pass_through,
    config::{self, LyricDisplayMode},
    DEFAULT_TEXT,
};

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
) -> Window {
    let window = Window::new(
        app,
        cache_lyrics,
        length_toleration_ms,
        show_default_text_on_idle,
    );

    window.set_size_request(500, WINDOW_MIN_HEIGHT);
    window.set_title(Some(DEFAULT_TEXT));
    window.present();

    let above_label = Label::builder().label("Waylyrics").name("above").build();
    let below_label = Label::builder()
        .label("")
        .name("below")
        .visible(false)
        .build();

    for label in [&above_label, &below_label] {
        utils::setup_label(label, enable_filter_regex);
    }
    above_label.set_vexpand(true);
    below_label.set_vexpand(true);

    let verical_box = gtk::Box::builder()
        .baseline_position(gtk::BaselinePosition::Center)
        .orientation(gtk::Orientation::Vertical)
        .build();
    verical_box.set_vexpand(true);
    verical_box.set_valign(gtk::Align::Center);

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

    window.set_icon_name(Some(crate::APP_ID));

    let display_mode = window.imp().lyric_display_mode.get();
    set_label_classes(&window, display_mode);
  
    window
}

pub fn set_lyric_display_mode(window: &Window, display_mode: LyricDisplayMode) -> Option<()> {
    window.imp().lyric_display_mode.set(display_mode);
    set_label_classes(window, display_mode);
    Some(())
}

pub fn set_label_classes(window: &Window, display_mode: LyricDisplayMode) -> Option<()> {
    let [above, below] = get_labels(window)?;

    match display_mode {
        LyricDisplayMode::ShowBoth => {
            above.set_css_classes(&["origin"]);
            below.set_css_classes(&["translation"]);
        }
        LyricDisplayMode::ShowBothRev => {
            above.set_css_classes(&["translation"]);
            below.set_css_classes(&["origin"]);
        }
        LyricDisplayMode::Origin => {
            above.set_css_classes(&["origin"]);
            below.set_css_classes(&[]);
        }
        LyricDisplayMode::PreferTranslation => {
            // how could we handle this?
            above.set_css_classes(&[]);
            below.set_css_classes(&[]);
        }
    }

    Some(())
}

pub fn set_lyric_align(window: &Window, align: config::Align) -> Option<()> {
    let labels: [Label; 2] = get_labels(window)?;
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
