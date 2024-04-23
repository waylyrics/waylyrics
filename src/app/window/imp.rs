use std::cell::Cell;
use std::time::SystemTime;

use crate::utils::gettext;
use gio::Settings;
use glib_macros::clone;
use gtk::gio::MenuItem;
use gtk::glib::Propagation;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib, ApplicationWindow, PopoverMenu};
use std::sync::OnceLock;

use crate::app::utils::set_click_pass_through;
use crate::config::{Align, LyricDisplayMode};
use crate::sync::list_players;

#[derive(Default)]
pub struct Window {
    pub settings: OnceLock<Settings>,

    pub clickthrough: Cell<bool>,
    pub cache_lyrics: Cell<bool>,

    pub lyric_align: Cell<Align>,
    pub lyric_display_mode: Cell<LyricDisplayMode>,
    pub show_default_text_on_idle: Cell<bool>,
    pub show_lyric_on_pause: Cell<bool>,

    pub lyric_start: Cell<Option<SystemTime>>,
    pub lyric_offset_ms: Cell<i64>,
    pub length_toleration_ms: Cell<u128>,

    // widgets
    pub headerbar: gtk::HeaderBar,
    pub menubutton: gtk::MenuButton,
    pub menu: gio::Menu,
    pub player_menu: gio::Menu,
    pub display_mode_menu: gio::Menu,
    pub align_mode_menu: gio::Menu,
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
        // set titlebar before loading state: whether to show it is a state
        obj.set_titlebar(Some(&self.headerbar));
        obj.load_window_state();

        self.headerbar.set_decoration_layout(Some("menu:close"));
        self.menubutton.set_icon_name("open-menu-symbolic");

        let hide_decoration = MenuItem::new(
            Some(&gettext("Hide Decoration")),
            Some("win.switch-decoration"),
        );
        let passthrough = MenuItem::new(
            Some(&gettext("Toggle Passthrough")),
            Some("win.switch-passthrough"),
        );
        let reload_theme = MenuItem::new(Some(&gettext("Reload theme")), Some("app.reload-theme"));
        let search_lyric = MenuItem::new(Some(&gettext("Search lyric")), Some("app.search-lyric"));
        let refetch_lyric =
            MenuItem::new(Some(&gettext("Refetch lyric")), Some("app.refetch-lyric"));
        let remove_lyric = MenuItem::new(
            Some(&if self.cache_lyrics.get() {
                gettext("Remove lyric")
            } else {
                gettext("Remove lyric forever")
            }),
            Some("app.remove-lyric"),
        );

        let popover = PopoverMenu::builder()
            .accessible_role(gtk::AccessibleRole::MenuItemRadio)
            .build();

        let ui_section = gio::Menu::default();
        ui_section.append_submenu(
            Some(&gettext("Lyric Display Mode")),
            &self.display_mode_menu,
        );
        ui_section.append_submenu(
            Some(&gettext("Lyric Alignment")), //
            &self.align_mode_menu,
        );
        ui_section.append_item(&passthrough);
        ui_section.append_item(&hide_decoration);
        ui_section.append_item(&reload_theme);
        self.menu.append_section(None, &ui_section);

        let play_section = gio::Menu::default();
        play_section.append_submenu(
            Some(&gettext("Select Player")), //
            &self.player_menu,
        );
        play_section.append_item(&search_lyric);
        play_section.append_item(&remove_lyric);
        play_section.append_item(&refetch_lyric);
        self.menu.append_section(None, &play_section);

        popover.set_menu_model(Some(&self.menu));

        let player_menu = &self.player_menu;
        popover.connect_visible_submenu_notify(clone!(@weak player_menu=> move |sub| {
            if Some(&*gettext("Select Player")) != sub.visible_submenu().as_deref() {
                return;
            }
            player_menu.remove_all();

            let section = gio::Menu::new();
            let players = list_players();
            if !players.is_empty() {
                let disconnect =
                    MenuItem::new(Some(&gettext("Disconnect")), Some("app.disconnect"));
                player_menu.append_item(&disconnect);
            }

            for player in players {
                let item = MenuItem::new(Some(&player.player_name), None);
                item.set_action_and_target_value(
                    Some("app.connect"),
                    Some(&ToVariant::to_variant(&player.inner_id)),
                );
                section.append_item(&item);
            }
            player_menu.append_section(None, &section);
        }));
        self.menubutton.set_popover(Some(&popover));

        for display_mode in <LyricDisplayMode as strum::IntoEnumIterator>::iter() {
            let display_mode_str = display_mode.to_string();
            let item = MenuItem::new(Some(&gettext(&display_mode_str).replace("_", "__")), None);
            item.set_action_and_target_value(
                Some("win.set-display-mode"),
                Some(&display_mode_str.to_variant()),
            );
            self.display_mode_menu.append_item(&item);
        }

        for lyric_align in <Align as strum::IntoEnumIterator>::iter() {
            let lyric_align_str = lyric_align.to_string();
            let item = MenuItem::new(Some(&gettext(&lyric_align_str)), None);
            item.set_action_and_target_value(
                Some("win.set-lyric-align"),
                Some(&lyric_align_str.to_variant()),
            );
            self.align_mode_menu.append_item(&item);
        }

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
