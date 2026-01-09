use crate::app::search_window;
use crate::log::{info, warn};
use crate::sync::lyric::fetch::tricks::LYRIC_TAG_CACHE;
use crate::sync::LyricState;
use crate::utils::bind_shortcut;
use glib_macros::clone;
use gtk::gio::SimpleAction;
use gtk::glib::{self, VariantTy};
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::Application;

use crate::app::dialog::show_dialog;
use crate::lyric_providers::LyricOwned;
use crate::sync::interop::common::update_lyric;
use crate::sync::{TrackState, LYRIC, TRACK_PLAYING_STATE};
use crate::utils::reset_lyric_labels;
use crate::{app, glib_spawn, MAIN_WINDOW};

use crate::sync::interop::{OsImp, OS};
pub mod utils;

pub fn register_disconnect(app: &Application) {
    let action = SimpleAction::new("disconnect", None);
    action.connect_activate(|_, _| {
        OS::clean_player();
    });
    app.add_action(&action);
}

pub fn register_search_lyric(app: &Application, wind: &app::Window, trigger: &str) {
    let action = SimpleAction::new("search-lyric", None);
    let cache_lyrics = wind.imp().cache_lyrics.get();
    action.connect_activate(move |_, _| {
        // Get current playing track
        let (title, album, artists) =
            TRACK_PLAYING_STATE.with_borrow(|TrackState { metainfo, .. }| {
                let Some(track) = metainfo.as_ref() else {
                    return <(String, String, String)>::default();
                };
                let artists = track
                    .artists
                    .as_ref()
                    .map(|artists| {
                        artists
                            .iter()
                            .map(String::as_str)
                            .collect::<Vec<&str>>()
                            .join("/")
                    })
                    .unwrap_or_default();
                let title = track.title.as_deref().unwrap_or_default().to_string();
                let album = track.album.as_deref().unwrap_or_default().to_string();
                (title, album, artists)
            });

        let window = search_window::Window::new(title, album, artists, cache_lyrics);
        window.present();
    });
    app.add_action(&action);

    bind_shortcut("app.search-lyric", wind, trigger);
}

/// update lyric, but do not ignore cache
pub fn register_reload_lyric(app: &Application) {
    let action = SimpleAction::new("reload-lyric", None);
    action.connect_activate(move |_, _| {
        let metainfo = TRACK_PLAYING_STATE
            .with_borrow(|TrackState { metainfo, .. }| metainfo.as_ref().cloned());
        let Some(metainfo) = metainfo else {
            return;
        };

        crate::log::debug!("spawned update_lyric from reload-lyric action");
        glib_spawn!(async move {
            let Some(wind) = MAIN_WINDOW.with_borrow(|wind| wind.as_ref().cloned()) else {
                return;
            };
            reset_lyric_labels(&wind, None);
            LYRIC_TAG_CACHE.clear(); // 手动刷新歌词时清空对歌曲文件中歌词标签存在性的缓存
            if let Err(err) = update_lyric(&metainfo, &wind, false).await {
                show_dialog(
                    Some(&wind),
                    &format!("cannot refetch lyric: {err:?}"),
                    gtk::MessageType::Error,
                );
            }
        });
    });
    app.add_action(&action);
}

pub fn register_refetch_lyric(app: &Application, window: &app::Window, trigger: &str) {
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
            reset_lyric_labels(&wind, None);
            LYRIC_TAG_CACHE.clear(); // 手动刷新歌词时清空对歌曲文件中歌词标签存在性的缓存
            if let Err(err) = update_lyric(&metainfo, &wind, true).await {
                show_dialog(
                    Some(&wind),
                    &format!("cannot refetch lyric: {err:?}"),
                    gtk::MessageType::Error,
                );
            }
        });
    });
    app.add_action(&action);

    bind_shortcut("app.refetch-lyric", window, trigger);
}

pub fn register_remove_lyric(app: &Application, wind: &app::Window) {
    let action = SimpleAction::new("remove-lyric", None);
    action.connect_activate(clone!(
        #[weak(rename_to = window)]
        wind,
        move |_, _| {
            // Clear current lyric
            let origin = LyricOwned::LineTimestamp(vec![]);
            let translation = LyricOwned::None;
            LYRIC.set(LyricState {
                origin,
                translation,
            });
            let cache_lyrics = window.imp().cache_lyrics.get();
            // Update cache
            if cache_lyrics {
                utils::update_cache();
            }
            // Remove current lyric inside window
            reset_lyric_labels(&window, None);
            info!("removed lyric");
        }
    ));
    app.add_action(&action);
}

#[cfg(feature = "import-lyric")]
pub fn register_import_lyric(app: &Application, wind: &app::Window) {
    use utils::import_lyric;

    let action = SimpleAction::new("import-lyric", Some(VariantTy::BOOLEAN));
    action.connect_activate(clone!(
        #[weak(rename_to = window)]
        wind,
        move |_, arg| {
            let arg = arg.cloned();
            glib_spawn!(async move {
                let Some(arg) = arg else { return };
                let Some(is_original) = arg.get() else { return };
                import_lyric(&window, is_original).await;
            });
        }
    ));
    app.add_action(&action);
}

#[cfg(feature = "export-lyric")]
pub fn register_export_lyric(app: &Application, wind: &app::Window) {
    use utils::export_lyric;

    let action = SimpleAction::new("export-lyric", Some(VariantTy::BOOLEAN));
    action.connect_activate(clone!(
        #[weak(rename_to = window)]
        wind,
        move |_, arg| {
            let arg = arg.cloned();
            glib_spawn!(async move {
                let Some(arg) = arg else { return };
                let Some(is_original) = arg.get() else { return };
                export_lyric(&window, is_original).await;
            });
        }
    ));
    app.add_action(&action);
}

pub fn register_connect(app: &Application) {
    let connect = SimpleAction::new("connect", Some(VariantTy::STRING));
    connect.connect_activate(|_, player_id| {
        let Some(player_id) = player_id.and_then(|p| p.str()) else {
            warn!("did not received string paramter for action \'app.connect\'");
            return;
        };

        OS::connect_player_with_id(player_id)
    });
    app.add_action(&connect);
}

#[cfg(feature = "action-event")]
mod event;
#[cfg(feature = "action-event")]
pub use event::{init_play_action_channel, PlayAction, PLAY_ACTION};
