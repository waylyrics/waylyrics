use async_channel::Sender;
use ksni::{Tray, TrayService};
use strum::IntoEnumIterator;

use crate::app::actions::{UIAction, UI_ACTION};
use crate::config::LyricDisplay;
use crate::sync::{PlayAction, PLAY_ACTION};

struct TrayIcon {}

impl Tray for TrayIcon {
    fn icon_name(&self) -> String {
        crate::APP_ID.to_string()
    }
    fn id(&self) -> String {
        env!("CARGO_PKG_NAME").into()
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;
        vec![
            SubMenu {
                label: "Lyric Display Mode".into(),
                submenu: LyricDisplay::iter()
                    .map(|display_mode| {
                        let label = display_mode.to_string().replace("_", "__");
                        StandardItem {
                            label,
                            activate: Box::new(move |_| {
                                let action = UIAction::SetDisplayMode(display_mode.to_string());
                                let _ = ui_action().send_blocking(action);
                            }),
                            ..Default::default()
                        }
                        .into()
                    })
                    .collect(),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Toggle Decoration".into(),
                activate: Box::new(|_| {
                    let _ = ui_action().send_blocking(UIAction::SwitchDecoration);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Toggle Passthrough".into(),
                activate: Box::new(|_| {
                    let _ = ui_action().send_blocking(UIAction::SwitchPassthrough);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Reload theme".into(),
                activate: Box::new(|_| {
                    let _ = ui_action().send_blocking(UIAction::ReloadTheme);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: "Quit".into(),
                activate: Box::new(|_| {
                    let _ = ui_action().send_blocking(UIAction::Quit);
                }),
                ..Default::default()
            }
            .into(),
        ]
    }
}

pub fn start_tray_service() -> Option<()> {
    let service = TrayService::new(TrayIcon {});

    Some(service.spawn())
}

fn ui_action() -> Sender<UIAction> {
    let ui_action = UI_ACTION.get().unwrap().clone();
    ui_action
}
fn play_action() -> Sender<PlayAction> {
    let play_action = PLAY_ACTION.get().unwrap().clone();
    play_action
}
