use std::sync::{mpsc, OnceLock};

use crate::app::{utils::set_click_pass_through, Window};

use gtk::gio::SimpleAction;
use gtk::glib::{self, WeakRef};
use gtk::{
    prelude::*, subclass::prelude::*, Application, NamedAction, Shortcut, ShortcutController,
    ShortcutTrigger,
};

pub fn register_action_switch_decoration(wind: &Window, switch_decoration_trigger: &str) {
    let action = SimpleAction::new("switch-decoration", None);
    let _wind = Window::downgrade(wind);
    action.connect_activate(move |_, _| {
        if let Some(wind) = _wind.upgrade() {
            wind.set_decorated(!wind.is_decorated());
        }
    });
    wind.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("win.switch-decoration"))
        .trigger(&ShortcutTrigger::parse_string(switch_decoration_trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn register_action_reload_theme(app: &Application, wind: &Window, trigger: &str) {
    let action = SimpleAction::new("reload-theme", None);
    action.connect_activate(move |_, _| {
        crate::THEME_PATH.with_borrow(|theme_path| {
            if let Ok(style) = std::fs::read_to_string(theme_path) {
                crate::app::utils::merge_css(&style);
            }
        });
    });
    app.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("app.reload-theme"))
        .trigger(&ShortcutTrigger::parse_string(trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn register_action_switch_passthrough(wind: &Window, trigger: &str) {
    let action = SimpleAction::new("switch-passthrough", None);
    let _wind = Window::downgrade(wind);
    action.connect_activate(move |_, _| {
        if let Some(wind) = _wind.upgrade() {
            let clickthrough = !wind.imp().clickthrough.get();
            wind.imp().clickthrough.set(clickthrough);
            set_click_pass_through(&wind, clickthrough);
            wind.present();
        }
    });
    wind.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("win.switch-passthrough"))
        .trigger(&ShortcutTrigger::parse_string(trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub enum UIAction {
    ReloadTheme,
    /// toggles mouse click passthrough
    SwitchPassthrough,
    /// toggles GTK+ CSD
    SwitchDecoration,
}

fn register_ui_action(
    app: WeakRef<Application>,
    wind: WeakRef<Window>,
) -> mpsc::Sender<UIAction> {
    let (tx, rx) = mpsc::channel();

    glib::idle_add_local(move || {
        let Some(app) = app.upgrade() else {
            return glib::ControlFlow::Continue;
        };
        let Some(wind) = wind.upgrade() else {
            return glib::ControlFlow::Continue;
        };

        match rx.try_recv() {
            Err(_) => return glib::ControlFlow::Continue,
            Ok(action) => match action {
                UIAction::ReloadTheme => app.activate_action("reload-theme", None),
                UIAction::SwitchPassthrough => {
                    ActionGroupExt::activate_action(&wind, "switch-passthrough", None)
                }
                UIAction::SwitchDecoration => {
                    ActionGroupExt::activate_action(&wind, "switch-decoration", None)
                }
            },
        };

        glib::ControlFlow::Continue
    });

    tx
}

pub fn init_ui_action_channel(app: WeakRef<Application>, wind: WeakRef<Window>) {
    let tx = register_ui_action(app, wind);
    UI_ACTION.set(tx).expect("must only initialize once");
}

pub static UI_ACTION: OnceLock<mpsc::Sender<UIAction>> = OnceLock::new();
