use std::cell::Cell;
use std::time::{Duration, SystemTime};

use gio::Settings;
use glib::once_cell::sync::OnceCell;
use gtk::gio::MenuItem;
use gtk::glib::Propagation;
use gtk::prelude::{ObjectExt, ToVariant};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, ApplicationWindow, PopoverMenu};

use crate::app::utils::set_click_pass_through;
use crate::app::REMOVE_LYRICS;
use crate::config::LyricDisplay;
use crate::sync::list_player_names;

#[derive(Default)]
pub struct Window {
    pub settings: OnceCell<Settings>,
    pub cache_lyrics: Cell<bool>,
    pub lyric_display_mode: Cell<LyricDisplay>,
    pub lyric_start: Cell<Option<SystemTime>>,
    pub lyric_playing: [Cell<Option<Duration>>; 2],
    pub lyric_offset_ms: Cell<i64>,
    pub length_toleration_ms: Cell<u128>,

    pub headerbar: gtk::HeaderBar,
    pub menubutton: gtk::MenuButton,
    pub menu: gio::Menu,
    pub submenu: gio::Menu,
    pub clickthrough: Cell<bool>,
}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "GtkAppWindowSaveState";
    type Type = super::Window;
    type ParentType = ApplicationWindow;
}
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        // Load latest window state
        let obj = self.obj();
        obj.setup_settings();
        obj.load_window_state();

        self.headerbar.set_decoration_layout(Some("menu:close"));
        self.menubutton.set_icon_name("open-menu-symbolic");

        let hide_decoration = MenuItem::new(Some("Hide Decoration"), Some("win.switch-decoration"));
        let passthrough = MenuItem::new(Some("Switch Passthrough"), Some("win.switch-passthrough"));
        let reload_theme = MenuItem::new(Some("Reload theme"), Some("app.reload-theme"));
        let search_lyric = MenuItem::new(Some("Search lyric"), Some("app.search-lyric"));
        let refetch_lyric = MenuItem::new(Some("Refetch lyric"), Some("app.refetch-lyric"));
        let remove_lyric = MenuItem::new(Some(&REMOVE_LYRICS.take()), Some("app.remove-lyric"));

        let popover = PopoverMenu::builder()
            .accessible_role(gtk::AccessibleRole::MenuItemRadio)
            .build();
        self.menu
            .append_submenu(Some("Switch players"), &self.submenu);
        self.menu.append_item(&passthrough);
        self.menu.append_item(&hide_decoration);
        self.menu.append_item(&reload_theme);
        self.menu.append_item(&search_lyric);
        self.menu.append_item(&remove_lyric);
        self.menu.append_item(&refetch_lyric);
        popover.set_menu_model(Some(&self.menu));

        let _submenu = self.submenu.downgrade();
        popover.connect_visible_submenu_notify(move |sub| {
            if Some("Switch players") != sub.visible_submenu().as_deref() {
                return;
            }
            let Some(menu) = _submenu.upgrade() else {
                return;
            };
            menu.remove_all();

            let section = gio::Menu::new();
            let players = list_player_names();
            if !players.is_empty() {
                let disconnect = MenuItem::new(Some("Disconnect"), Some("app.disconnect"));
                menu.append_item(&disconnect);
            }

            for player in players {
                let item = MenuItem::new(Some(&player.player_name), None);
                item.set_action_and_target_value(
                    Some("app.connect"),
                    Some(&ToVariant::to_variant(&player.inner_id)),
                );
                section.append_item(&item);
            }
            menu.append_section(None, &section);
        });

        self.menubutton.set_popover(Some(&popover));
        self.headerbar.pack_end(&self.menubutton)
    }
}
impl WidgetImpl for Window {
    fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
        self.parent_size_allocate(width, height, baseline);
        let clickthrough = self.clickthrough.get();
        set_click_pass_through(&self.obj(), clickthrough);
    }
}
impl WindowImpl for Window {
    // Save window state right before the window will be closed
    fn close_request(&self) -> Propagation {
        // Save window size
        self.obj()
            .save_window_state()
            .expect("Failed to save window state");

        // Don't invoke the default handler
        Propagation::Proceed
    }
}
impl ApplicationWindowImpl for Window {}
