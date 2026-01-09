use std::env::args;
use std::mem;
use std::process::Command;

use anyhow::Result;
use async_channel::Sender;

use tray_item::{IconSource, TrayItem};

use crate::app::actions::{UIAction, UI_ACTION};
use crate::sync::{PlayAction, PLAY_ACTION};
use crate::utils::gettext;

pub fn start_tray_service() -> Result<()> {
    let icon = IconSource::Resource("icon0");
    let mut tray = TrayItem::new(crate::DEFAULT_TEXT, icon)?;

    tray.add_menu_item(&gettext("Search lyric"), || {
        let _ = play_action().send_blocking(PlayAction::SearchLyric);
    })?;
    tray.add_menu_item(&gettext("Refetch lyric"), || {
        let _ = play_action().send_blocking(PlayAction::RefetchLyric);
    })?;
    tray.add_menu_item(&gettext("Toggle Decoration"), || {
        let _ = ui_action().send_blocking(UIAction::SwitchDecoration);
    })?;
    tray.add_menu_item(&gettext("Toggle Passthrough"), || {
        let _ = ui_action().send_blocking(UIAction::SwitchPassthrough);
    })?;
    tray.add_menu_item(&gettext("Restart"), || {
        let _ = Command::new("powershell")
            .arg("-WindowStyle")
            .arg("1")
            .arg("-Command")
            .arg(format!("sleep 5; start {}", args().next().unwrap()))
            .spawn();
        let _ = ui_action().send_blocking(UIAction::Quit);
    })?;
    tray.add_menu_item(&gettext("Quit"), || {
        let _ = ui_action().send_blocking(UIAction::Quit);
    })?;

    mem::forget(tray);

    Ok(())
}

fn ui_action() -> &'static Sender<UIAction> {
    UI_ACTION.get().unwrap()
}
fn play_action() -> &'static Sender<PlayAction> {
    PLAY_ACTION.get().unwrap()
}
