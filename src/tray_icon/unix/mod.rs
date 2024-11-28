use std::thread::sleep;
use std::time::Duration;
use std::{env, process};

use async_channel::Sender;
use gtk::glib;

use crate::utils::gettext;
use ksni::{Tray, TrayService};
use rust_decimal::prelude::Zero;
use strum::IntoEnumIterator;

use crate::app::actions::{UIAction, UI_ACTION};
use crate::sync::{PlayAction, PLAY_ACTION};

use crate::config::{Align, LyricDisplayMode};
use crate::sync::{OsImp, PlayerId, OS};

use crate::log::error;
use crate::{INSTANCE_NAME, PACKAGE_NAME};

#[derive(Debug)]
struct TrayIcon {
    // For calling list_players() inside main thread.
    req_tx: async_channel::Sender<()>,
    resp_rx: async_channel::Receiver<Vec<PlayerId>>,
}

impl TrayIcon {
    pub fn new(
        req_tx: async_channel::Sender<()>,
        resp_rx: async_channel::Receiver<Vec<PlayerId>>,
    ) -> Self {
        Self { req_tx, resp_rx }
    }
}

impl Tray for TrayIcon {
    fn icon_name(&self) -> String {
        INSTANCE_NAME.get().unwrap().to_string()
    }
    fn id(&self) -> String {
        PACKAGE_NAME.into()
    }
    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        use ksni::menu::*;

        self.req_tx.send_blocking(()).unwrap();
        let players = self.resp_rx.recv_blocking().unwrap();

        vec![
            SubMenu {
                label: gettext("Lyric Display Mode"),
                icon_name: "quickview".into(),
                submenu: LyricDisplayMode::iter()
                    .map(|display_mode| {
                        let label = gettext(display_mode.to_string());
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
            SubMenu {
                label: gettext("Lyric Alignment"),
                icon_name: "format-justify-left".into(),
                submenu: Align::iter()
                    .map(|lyric_align| {
                        let label = gettext(lyric_align.to_string());
                        StandardItem {
                            label,
                            activate: Box::new(move |_| {
                                let action = UIAction::SetLyricAlign(lyric_align.to_string());
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
                label: gettext("Toggle Decoration"),
                icon_name: "window-new".into(),
                activate: Box::new(|_| {
                    let _ = ui_action().send_blocking(UIAction::SwitchDecoration);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: gettext("Toggle Passthrough"),
                icon_name: "input-mouse".into(),
                activate: Box::new(|_| {
                    let _ = ui_action().send_blocking(UIAction::SwitchPassthrough);
                    let dur = Duration::from_millis(200);
                    sleep(dur);
                    let _ = ui_action().send_blocking(UIAction::SwitchDecoration);
                    sleep(dur);
                    let _ = ui_action().send_blocking(UIAction::SwitchDecoration);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: gettext("Reload theme"),
                icon_name: "color-management".into(),
                activate: Box::new(|_| {
                    let _ = ui_action().send_blocking(UIAction::ReloadTheme);
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            SubMenu {
                label: gettext("Select Player"),
                icon_name: "format-indent-more".into(),
                enabled: !players.len().is_zero(),
                submenu: players
                    .into_iter()
                    .map(
                        |PlayerId {
                             player_name,
                             inner_id,
                         }| {
                            StandardItem {
                                label: player_name,
                                activate: Box::new(move |_| {
                                    let _ = play_action()
                                        .send_blocking(PlayAction::Connect(inner_id.clone()));
                                }),
                                ..Default::default()
                            }
                            .into()
                        },
                    )
                    .collect(),
                ..Default::default()
            }
            .into(),
            #[cfg(feature = "import-lyric")]
            SubMenu {
                label: gettext("Import Lyric"),
                icon_name: "document-import".into(),
                submenu: vec![
                    StandardItem {
                        label: gettext("Original Lyric"),
                        activate: Box::new(|_| {
                            let _ = play_action().send_blocking(PlayAction::ImportOriginalLyric);
                        }),
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: gettext("Translated Lyric"),
                        activate: Box::new(|_| {
                            let _ = play_action().send_blocking(PlayAction::ImportTranslatedLyric);
                        }),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into(),
            #[cfg(feature = "export-lyric")]
            SubMenu {
                label: gettext("Export Lyric"),
                icon_name: "document-export".into(),
                submenu: vec![
                    StandardItem {
                        label: gettext("Original Lyric"),
                        activate: Box::new(|_| {
                            let _ = play_action().send_blocking(PlayAction::ExportOriginalLyric);
                        }),
                        ..Default::default()
                    }
                    .into(),
                    StandardItem {
                        label: gettext("Translated Lyric"),
                        activate: Box::new(|_| {
                            let _ = play_action().send_blocking(PlayAction::ExportTranslatedLyric);
                        }),
                        ..Default::default()
                    }
                    .into(),
                ],
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: gettext("Search Lyric"),
                icon_name: "system-search".into(),
                activate: Box::new(|_| {
                    let _ = play_action().send_blocking(PlayAction::SearchLyric);
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: gettext("Refetch Lyric"),
                icon_name: "folder-download".into(),
                activate: Box::new(|_| {
                    let _ = play_action().send_blocking(PlayAction::RefetchLyric);
                }),
                ..Default::default()
            }
            .into(),
            MenuItem::Separator,
            StandardItem {
                label: gettext("Restart"),
                icon_name: "system-reboot".into(),
                activate: Box::new(|_| {
                    restart_myself();
                }),
                ..Default::default()
            }
            .into(),
            StandardItem {
                label: gettext("Quit"),
                icon_name: "application-exit".into(),
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
    // mpris::PlayerFinder would create a new DBus connection which has 4 matches attached,
    // but in TrayService, the messages matching could never be handled.
    // This would make messages stalling within dbus broker, and might finally make dbus-broker
    // terminates some connections inside user session as it would exceed its per-UID bytes quota.
    // This could be fatal as modern desktop environments would expect dbus service always working,
    // and the quota exceeding issue could bring complete user session down.
    // To resolve this issue, here we just enforce that we shall call OS::list_players() only in
    // main thread (by glib async runtime), with 2 channels.
    let (req_tx, req_rx) = async_channel::unbounded();
    let (resp_tx, resp_rx) = async_channel::unbounded();
    let service = TrayService::new(TrayIcon::new(req_tx, resp_rx));
    glib::spawn_future_local(async move {
        while req_rx.recv().await.is_ok() {
            let players = OS::list_players();
            resp_tx.send(players).await.unwrap();
        }
    });
    service.spawn_without_dbus_name();
    Some(())
}

fn ui_action() -> &'static Sender<UIAction> {
    UI_ACTION.get().unwrap()
}
fn play_action() -> &'static Sender<PlayAction> {
    PLAY_ACTION.get().unwrap()
}

fn restart_myself() {
    let my_path = env::args().next().unwrap();
    let Ok(_) = process::Command::new("sh")
        .arg("-c")
        .arg(format!("sleep 1 && {my_path}"))
        .spawn()
    else {
        error!("failed to run waylyrics");
        return;
    };
    let _ = ui_action().send_blocking(UIAction::Quit);
}
