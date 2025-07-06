#[cfg(feature = "action-event")]
mod event;

#[cfg(feature = "action-event")]
pub use event::{init_ui_action_channel, UIAction, UI_ACTION};

use crate::app::{utils::set_click_pass_through, Window};

use crate::config::Align;
use crate::log::error;
use crate::utils::bind_shortcut;

use glib_macros::clone;
use gtk::gio::SimpleAction;
use gtk::glib::{self, Variant, VariantTy};
use gtk::{prelude::*, subclass::prelude::*, Application};

use super::{get_label, set_lyric_align};

pub fn register_switch_decoration(wind: &Window, trigger: &str) {
    let action = SimpleAction::new("switch-decoration", None);
    action.connect_activate(clone!(
        #[weak]
        wind,
        move |_, _| {
            wind.set_decorated(!wind.is_decorated());
        }
    ));
    wind.add_action(&action);

    bind_shortcut("win.switch-decoration", wind, trigger);
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
    bind_shortcut("app.reload-theme", wind, trigger);
}

pub fn register_switch_passthrough(wind: &Window, trigger: &str) {
    let action = SimpleAction::new("switch-passthrough", None);
    action.connect_activate(clone!(
        #[weak]
        wind,
        move |_, _| {
            let clickthrough = !wind.imp().clickthrough.get();
            wind.imp().clickthrough.set(clickthrough);
            set_click_pass_through(&wind, clickthrough);
            wind.present();
        }
    ));
    wind.add_action(&action);

    bind_shortcut("win.switch-passthrough", wind, trigger);
}

pub fn register_set_display_mode(wind: &Window) {
    let action = SimpleAction::new("set-display-mode", Some(VariantTy::STRING));
    action.connect_activate(clone!(
        #[weak]
        wind,
        move |_, display_mode| {
            let Some(display_mode) = display_mode.and_then(|d| d.str()) else {
                return;
            };
            let Ok(display_mode) = display_mode.parse() else {
                error!("unknown display_mode: {display_mode}");
                return;
            };
            wind.imp().lyric_display_mode.set(display_mode);
        }
    ));
    wind.add_action(&action);
}

pub fn register_set_lyric_align(wind: &Window) {
    let action = SimpleAction::new("set-lyric-align", Some(VariantTy::STRING));
    action.connect_activate(clone!(
        #[weak]
        wind,
        move |_, lyric_align| {
            let Some(align) = lyric_align.and_then(|d| d.str()) else {
                return;
            };
            let Ok(align): Result<Align, _> = align.parse() else {
                error!("unknown lyric alignment: {align}");
                return;
            };
            set_lyric_align(&wind, align);
        }
    ));
    wind.add_action(&action);
}

pub fn register_set_above_label(wind: &Window) {
    let action = SimpleAction::new("set-label", Some(VariantTy::STRING_ARRAY));
    action.connect_activate(clone!(
        #[weak]
        wind,
        move |_, args| {
            let Some((position, text)) = args.and_then(extract_str_array) else {
                return;
            };
            get_label(&wind, position).set_label(text);
        }
    ));
    wind.add_action(&action);
}

fn extract_str_array(variant: &Variant) -> Option<(&str, &str)> {
    let mut iter = variant.array_iter_str().ok()?;
    let position = iter.next()?;

    if !["above", "below"].contains(&position) {
        return None;
    }

    let text = iter.next()?;

    Some((position, text))
}
