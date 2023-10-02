use gtk::{prelude::*, NamedAction, Shortcut, ShortcutController, ShortcutTrigger};

use crate::app;

pub fn bind_shortcut(action_name: impl AsRef<str>, wind: &app::Window, trigger: impl AsRef<str>) {
    let shortcut = Shortcut::builder()
        .action(&NamedAction::new(action_name.as_ref()))
        .trigger(&ShortcutTrigger::parse_string(trigger.as_ref()).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}
