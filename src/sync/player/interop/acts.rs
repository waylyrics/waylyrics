use gtk::{
    gio::SimpleAction,
    glib::{self, VariantTy},
    prelude::*,
    Application, NamedAction, Shortcut, ShortcutController, ShortcutTrigger,
};
use tracing::{error, info, warn};

use crate::{
    app,
    sync::{PLAYER, PLAYER_FINDER, TRACK_PLAYING_STATE, search_window},
};

pub fn register_action_disconnect(app: &Application) {
    let action = SimpleAction::new("disconnect", None);
    action.connect_activate(|_, _| {
        PLAYER.set(None);
    });
    app.add_action(&action);
}

pub fn register_sigusr1_disconnect() {
    glib::unix_signal_add_local(libc::SIGUSR1, move || {
        PLAYER.set(None);
        Continue(true)
    });
}

// TODO: code cleanup
pub fn register_action_search_lyric(app: &Application, wind: &app::Window, trigger: &str) {
    let action = SimpleAction::new("search-lyric", None);
    action.connect_activate(move |_, _| {
        let window = search_window::Window::new();
        window.present();
    });
    app.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("app.search-lyric"))
        .trigger(&ShortcutTrigger::parse_string(trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn register_action_reload_lyric(app: &Application, wind: &app::Window, trigger: &str) {
    let action = SimpleAction::new("reload-lyric", None);
    action.connect_activate(move |_, _| {
        TRACK_PLAYING_STATE.set((None, false, None));
        info!("cleaned lyric");
    });
    app.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("app.reload-lyric"))
        .trigger(&ShortcutTrigger::parse_string(trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn register_action_remove_lyric(app: &Application) {
    let action = SimpleAction::new("remove-lyric", None);
    action.connect_activate(move |_, _| {
        // TODO: which func?
        info!("removed lyric");
    });
    app.add_action(&action);
}

pub fn register_action_connect(app: &Application) {
    let connect = SimpleAction::new("connect", Some(VariantTy::STRING));
    connect.connect_activate(|_, player_id| {
        let Some(player_id) = player_id.and_then(|p| p.str()) else {
            warn!("did not received string paramter for action \'app.connect\'");
            return;
        };
        PLAYER_FINDER.with_borrow(|player_finder| {
            if let Ok(player) = player_finder.find_by_name(player_id) {
                PLAYER.set(Some(player));
            } else {
                error!("cannot connect to: {player_id}");
            }
        });
    });
    app.add_action(&connect);
}