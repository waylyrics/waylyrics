use crate::log::{info, warn};
use glib_macros::clone;
use gtk::{
    gio::SimpleAction,
    glib::{self, VariantTy},
    prelude::*,
    subclass::prelude::ObjectSubclassIsExt,
    Application,
};
mod utils;

use crate::{
    app::{self, dialog::show_dialog},
    glib_spawn,
    lyric_providers::{default_search_query, LyricOwned},
    sync::{
        interop::clean_player, interop::common::update_lyric, lyric::cache::update_lyric_cache,
        search_window, TrackState, LYRIC, TRACK_PLAYING_STATE,
    },
    utils::reset_lyric_labels,
    MAIN_WINDOW,
};

use crate::sync::interop::connect_player_with_id;

pub fn register_action_disconnect(app: &Application) {
    let action = SimpleAction::new("disconnect", None);
    action.connect_activate(|_, _| {
        clean_player();
    });
    app.add_action(&action);
}

pub fn register_sigusr1_disconnect() {
    glib::unix_signal_add_local(libc::SIGUSR1, move || {
        clean_player();
        glib::ControlFlow::Continue
    });
}

// TODO: code cleanup
pub fn register_action_search_lyric(app: &Application, wind: &app::Window, trigger: &str) {
    let action = SimpleAction::new("search-lyric", None);
    let cache_lyrics = wind.imp().cache_lyrics.get();
    action.connect_activate(move |_, _| {
        // Get current playing track
        let query_default = TRACK_PLAYING_STATE.with_borrow(|TrackState { metainfo, .. }| {
            metainfo.as_ref().map(|track| {
                let artists = if let Some(artists) = track.artists.as_ref() {
                    artists.iter().map(|s| s.as_str()).collect()
                } else {
                    vec![]
                };
                default_search_query(
                    track.album.as_deref().unwrap_or_default(),
                    &artists,
                    track.title.as_deref().unwrap_or("Unknown"),
                )
            })
        });

        let window = search_window::Window::new(query_default.as_deref(), cache_lyrics);
        window.present();
    });
    app.add_action(&action);

    utils::bind_shortcut("app.search-lyric", wind, trigger);
}

pub fn register_action_refetch_lyric(app: &Application, window: &app::Window, trigger: &str) {
    let action = SimpleAction::new("refetch-lyric", None);
    action.connect_activate(move |_, _| {
        let metainfo = TRACK_PLAYING_STATE
            .with_borrow(|TrackState { metainfo, .. }| metainfo.as_ref().cloned());
        let Some(metainfo) = metainfo else {
            return;
        };

        crate::log::debug!("spawned update_lyric from refetch-lyric action");
        glib_spawn!(async move {
            let Some(wind) = MAIN_WINDOW.with_borrow(|wind| wind.as_ref().cloned()) else {
                return;
            };
            reset_lyric_labels(&wind);
            if let Err(err) = update_lyric(&metainfo, &wind, true).await {
                show_dialog(
                    Some(&wind),
                    &format!("cannot refetch lyric: {err:?}"),
                    gtk::MessageType::Error,
                )
            }
        });
    });
    app.add_action(&action);

    utils::bind_shortcut("app.refetch-lyric", window, trigger);
}

pub fn register_action_remove_lyric(app: &Application, wind: &app::Window) {
    let action = SimpleAction::new("remove-lyric", None);
    let cache_lyrics = wind.imp().cache_lyrics.get();
    action.connect_activate(clone!(@weak wind as window => move |_, _| {
        // Clear current lyric
        LYRIC.with_borrow_mut(|(origin, translation)| {
            *origin = LyricOwned::LineTimestamp(vec![]);
            *translation = LyricOwned::None;
        });
        // Update cache
        if cache_lyrics {
            TRACK_PLAYING_STATE.with_borrow(|TrackState{ cache_path, ..}| {
                if let Some(cache_path) = cache_path {
                    update_lyric_cache(cache_path);
                }
            });
        }
        // Remove current lyric inside window
        reset_lyric_labels(&window);
        info!("removed lyric");
    }));
    app.add_action(&action);
}

pub fn register_action_connect(app: &Application) {
    let connect = SimpleAction::new("connect", Some(VariantTy::STRING));
    connect.connect_activate(|_, player_id| {
        let Some(player_id) = player_id.and_then(|p| p.str()) else {
            warn!("did not received string paramter for action \'app.connect\'");
            return;
        };

        connect_player_with_id(player_id)
    });
    app.add_action(&connect);
}
