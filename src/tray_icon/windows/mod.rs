use std::{env::args, process::Command};

use anyhow::Result;
use async_channel::Sender;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuItemBuilder},
    Icon, TrayIconBuilder,
};

use crate::{
    app::actions::{UIAction, UI_ACTION},
    sync::{PlayAction, PLAY_ACTION},
    utils::gettext,
};

const EXIT: &str = "exit";
const RESTART: &str = "restart";
const SEARCH_LYRIC: &str = "search-lyric";
const SWITCH_PASSTHROUGH: &str = "switch-passthrough";
const SWITCH_DECORATION: &str = "switch-decoration";

pub fn start_tray_service() -> Result<()> {
    let icon = Icon::from_resource_name("icon0", None)?;
    let menu = build_tray_menu()?;
    let tray_icon = TrayIconBuilder::new()
        .with_tooltip(env!("CARGO_PKG_DESCRIPTION"))
        .with_icon(icon)
        .with_menu(Box::new(menu))
        .build()?;

    std::thread::spawn(menu_event_handler);
    std::mem::forget(tray_icon);
    Ok(())
}

fn build_tray_menu() -> Result<Menu> {
    let search_lyric = MenuItemBuilder::new()
        .text(gettext("Search lyric"))
        .id(SEARCH_LYRIC.into())
        .enabled(true)
        .build();
    let switch_decoration = MenuItemBuilder::new()
        .text(gettext("Toggle Decoration"))
        .id(SWITCH_DECORATION.into())
        .enabled(true)
        .build();
    let switch_passthrough = MenuItemBuilder::new()
        .text(gettext("Toggle Passthrough"))
        .id(SWITCH_PASSTHROUGH.into())
        .enabled(true)
        .build();
    let restart = MenuItemBuilder::new()
        .text(gettext("Restart"))
        .id(RESTART.into())
        .enabled(true)
        .build();
    let exit = MenuItemBuilder::new()
        .text(gettext("Quit"))
        .id(EXIT.into())
        .enabled(true)
        .build();

    Ok(Menu::with_items(&[
        &search_lyric,
        &switch_decoration,
        &switch_passthrough,
        &restart,
        &exit,
    ])?)
}

fn menu_event_handler() {
    while let Ok(event) = MenuEvent::receiver().recv() {
        match event.id().0.as_str() {
            SEARCH_LYRIC => {
                let _ = play_action().send_blocking(PlayAction::SearchLyric);
            }
            RESTART => {
                let _ = Command::new("powershell")
                    .arg("-WindowStyle")
                    .arg("1")
                    .arg("-Command")
                    .arg(&format!("sleep 5; start {}", args().next().unwrap()))
                    .spawn();
                let _ = ui_action().send_blocking(UIAction::Quit);
            }
            EXIT => {
                let _ = ui_action().send_blocking(UIAction::Quit);
            }
            SWITCH_DECORATION => {
                let _ = ui_action().send_blocking(UIAction::SwitchDecoration);
            }
            SWITCH_PASSTHROUGH => {
                let _ = ui_action().send_blocking(UIAction::SwitchPassthrough);
            }
            _ => unreachable!(),
        }
    }
}

fn ui_action() -> &'static Sender<UIAction> {
    UI_ACTION.get().unwrap()
}
fn play_action() -> &'static Sender<PlayAction> {
    PLAY_ACTION.get().unwrap()
}
