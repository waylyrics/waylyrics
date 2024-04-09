use anyhow::Result;
use async_channel::Sender;
use tray_icon::{
    menu::{Menu, MenuEvent, MenuId, MenuItemBuilder},
    Icon, TrayIconBuilder,
};

use crate::{
    app::actions::{UIAction, UI_ACTION},
    sync::{PlayAction, PLAY_ACTION},
};

const EXIT: &str = "exit";
const SEARCH_LYRIC: &str = "search-lyric";
const SWITCH_PASSTHROUGH: &str = "switch-passthrough";
const SWITCH_DECORATION: &str = "switch-decoration";

pub fn start_tray_service() -> Result<()> {
    let search_lyric = MenuItemBuilder::new()
        .text("Search lyric")
        .id(MenuId::new(SEARCH_LYRIC))
        .enabled(true)
        .build();
    let switch_decoration = MenuItemBuilder::new()
        .text("Toggle decoration")
        .id(MenuId::new(SWITCH_DECORATION))
        .enabled(true)
        .build();
    let switch_passthrough = MenuItemBuilder::new()
        .text("Toggle passthrough")
        .id(MenuId::new(SWITCH_PASSTHROUGH))
        .enabled(true)
        .build();
    let exit = MenuItemBuilder::new()
        .text("Exit")
        .id(MenuId::new(EXIT))
        .enabled(true)
        .build();

    let menu = Menu::with_items(&[
        &search_lyric,
        &switch_decoration,
        &switch_passthrough,
        &exit,
    ])?;
    let tray_icon = TrayIconBuilder::new()
        .with_tooltip(env!("CARGO_PKG_DESCRIPTION"))
        .with_icon(Icon::from_resource_name("icon0", None)?)
        .with_menu(Box::new(menu))
        .build()?;

    std::mem::forget(tray_icon);
    std::thread::spawn(menu_event_handler);
    Ok(())
}

fn menu_event_handler() {
    while let Ok(event) = MenuEvent::receiver().recv() {
        match event.id().0.as_str() {
            SEARCH_LYRIC => {
                let _ = play_action().send_blocking(PlayAction::SearchLyric);
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
