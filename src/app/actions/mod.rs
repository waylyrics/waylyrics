#[cfg(feature = "action-event")]
mod event;

#[cfg(feature = "action-event")]
pub use event::{init_ui_action_channel, UIAction, UI_ACTION};

use crate::app::{utils::set_click_pass_through, Window};

use crate::config::Align;
use crate::log::error;

use glib_macros::clone;
use gtk::gio::SimpleAction;
use gtk::glib::{self, VariantTy};
use gtk::{
    prelude::*, subclass::prelude::*, Application, NamedAction, Shortcut, ShortcutController,
    ShortcutTrigger,
};

use super::{set_lyric_display_mode, set_lyric_align};

pub fn register_switch_decoration(wind: &Window, switch_decoration_trigger: &str) {
    let action = SimpleAction::new("switch-decoration", None);
    action.connect_activate(clone!(@weak wind => move |_, _| {
        wind.set_decorated(!wind.is_decorated());
    }));
    wind.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("win.switch-decoration"))
        .trigger(&ShortcutTrigger::parse_string(switch_decoration_trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn register_reload_theme(app: &Application, wind: &Window, trigger: &str) {
    let action = SimpleAction::new("reload-theme", None);
    action.connect_activate(move |_, _| {
        crate::THEME_PATH.with_borrow(|theme_path| {
            if let Ok(style) = std::fs::read_to_string(theme_path) {
                crate::app::utils::merge_css(&style);
            }
        });
    });
    app.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("app.reload-theme"))
        .trigger(&ShortcutTrigger::parse_string(trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn register_switch_passthrough(wind: &Window, trigger: &str) {
    let action = SimpleAction::new("switch-passthrough", None);
    action.connect_activate(clone!(@weak wind => move |_, _| {
        let clickthrough = !wind.imp().clickthrough.get();
        wind.imp().clickthrough.set(clickthrough);
        set_click_pass_through(&wind, clickthrough);
        wind.present();
    }));
    wind.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("win.switch-passthrough"))
        .trigger(&ShortcutTrigger::parse_string(trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn register_set_display_mode(wind: &Window) {
    let action = SimpleAction::new("set-display-mode", Some(VariantTy::STRING));
    action.connect_activate(clone!(@weak wind => move |_, display_mode| {
        let Some(display_mode) = display_mode.and_then(|d| d.str()) else {
            return;
        };
        let Ok(display_mode) = display_mode.parse() else {
            error!("unknown display_mode: {display_mode}");
            return;
        };
        set_lyric_display_mode(&wind, display_mode);
    }));
    wind.add_action(&action);
}

pub fn register_set_lyric_align(wind: &Window) {
    let action = SimpleAction::new("set-lyric-align", Some(VariantTy::STRING));
    action.connect_activate(clone!(@weak wind => move |_, lyric_align| {
        let Some(align) = lyric_align.and_then(|d| d.str()) else {
            return;
        };
        let Ok(align): Result<Align, _> = align.parse() else {
            error!("unknown lyric align: {align}");
            return;
        };
        set_lyric_align(&wind, align);
    }));
    wind.add_action(&action);
}
