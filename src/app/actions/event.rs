use std::sync::OnceLock;

use async_channel::Sender;
use gtk::prelude::*;
use gtk::{
    glib::{self, WeakRef},
    Application,
};

use crate::app::Window;
use crate::log::debug;

#[derive(Clone, Debug)]
pub enum UIAction {
    ReloadTheme,
    /// toggles mouse click passthrough
    SwitchPassthrough,
    /// toggles GTK+ CSD
    SwitchDecoration,
    /// set new lyric display mode
    SetDisplayMode(String),
    /// set new lyric alignment mode
    SetLyricAlign(String),
    /// Quit Waylyrics
    Quit,
}

fn register_ui_action(app: WeakRef<Application>, wind: WeakRef<Window>) -> Sender<UIAction> {
    let (tx, rx) = async_channel::unbounded();

    glib::spawn_future_local(async move {
        while let Ok(action) = rx.recv().await {
            debug!("Received UI Action {action:?}");

            let Some(app) = app.upgrade() else {
                continue;
            };
            let Some(wind) = wind.upgrade() else {
                continue;
            };

            match action {
                UIAction::ReloadTheme => app.activate_action("reload-theme", None),
                UIAction::SwitchPassthrough => {
                    ActionGroupExt::activate_action(&wind, "switch-passthrough", None)
                }
                UIAction::SwitchDecoration => {
                    ActionGroupExt::activate_action(&wind, "switch-decoration", None)
                }
                UIAction::SetDisplayMode(display_mode) => ActionGroupExt::activate_action(
                    &wind,
                    "set-display-mode",
                    Some(&display_mode.to_variant()),
                ),
                UIAction::SetLyricAlign(lyric_align) => ActionGroupExt::activate_action(
                    &wind,
                    "set-lyric-align",
                    Some(&lyric_align.to_variant()),
                ),
                UIAction::Quit => {
                    wind.close();
                }
            }
        }
    });

    tx
}

pub fn init_ui_action_channel(app: WeakRef<Application>, wind: WeakRef<Window>) {
    let tx = register_ui_action(app, wind);
    UI_ACTION.set(tx).expect("must only initialize once");
}

pub static UI_ACTION: OnceLock<Sender<UIAction>> = OnceLock::new();
