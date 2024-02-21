use std::cell::Cell;
use std::time::SystemTime;

use gettextrs::gettext;
use gio::Settings;
use gtk::gio::MenuItem;
use gtk::glib::Propagation;
use gtk::prelude::{ObjectExt, ToVariant};
use gtk::subclass::prelude::*;
use gtk::{gio, glib, ApplicationWindow, PopoverMenu};
use std::sync::OnceLock;

use crate::app::utils::set_click_pass_through;
use crate::config::{Align, LyricDisplay};
use crate::sync::list_player_names;

#[derive(Default)]
pub struct Window {
    pub settings: OnceLock<Settings>,

    pub cache_lyrics: Cell<bool>,
    pub lyric_display_mode: Cell<LyricDisplay>,
    pub lyric_align: Cell<Align>,
    pub lyric_start: Cell<Option<SystemTime>>,
    pub lyric_offset_ms: Cell<i64>,
    pub length_toleration_ms: Cell<u128>,
    pub show_default_text_on_idle: Cell<bool>,

    pub headerbar: gtk::HeaderBar,
    pub menubutton: gtk::MenuButton,
    pub menu: gio::Menu,
    pub player_menu: gio::Menu,
    pub display_mode_menu: gio::Menu,
    pub align_mode_menu: gio::Menu,
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
        self.menu.append_submenu(
            Some(&gettext("Select Player")), //
            &self.player_menu,
        );
        self.menu.append_submenu(
            Some(&gettext("Lyric Display Mode")),
            &self.display_mode_menu,
        );
        self.menu.append_submenu(
            Some(&gettext("Lyric Align")), //
            &self.align_mode_menu,
        );

        self.menu.append_item(&passthrough);
        self.menu.append_item(&hide_decoration);
        self.menu.append_item(&reload_theme);
        self.menu.append_item(&search_lyric);
        self.menu.append_item(&remove_lyric);
        self.menu.append_item(&refetch_lyric);
        popover.set_menu_model(Some(&self.menu));

        let player_menu_weak = self.player_menu.downgrade();
        popover.connect_visible_submenu_notify(move |sub| {
            if Some(&*gettext("Select Player")) != sub.visible_submenu().as_deref() {
                return;
            }
            let Some(menu) = player_menu_weak.upgrade() else {
                return;
            };
            menu.remove_all();

            let section = gio::Menu::new();
            let players = list_player_names();
            if !players.is_empty() {
                let disconnect =
                    MenuItem::new(Some(&gettext("Disconnect")), Some("app.disconnect"));
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

        for display_mode in <LyricDisplay as strum::IntoEnumIterator>::iter() {
            let display_mode_str = display_mode.to_string();
            let item = MenuItem::new(Some(&gettext(&display_mode_str)), None);
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
