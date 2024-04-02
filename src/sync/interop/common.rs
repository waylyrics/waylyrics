use anyhow::Error;
use gtk::subclass::prelude::*;
use tokio::sync::Mutex;

use std::sync::OnceLock;

use std::time::Duration;

use gtk::{
    glib::{self, WeakRef},
    prelude::*,
    Application,
};

use anyhow::Result;

use crate::{
    app,
    log::*,
    sync::{
        interop::PlayerStatus,
        lyric::{cache, fetch},
        utils::clean_lyric,
        TrackMeta, TrackState, TRACK_PLAYING_STATE,
    },
    utils::reset_lyric_labels,
};

use super::{reconnect_player, try_sync_track};

pub async fn update_lyric(
    track_meta: &TrackMeta,
    window: &app::Window,
    ignore_cache: bool,
) -> Result<(), Error> {
    static UPDATE_LYRIC_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let lock = UPDATE_LYRIC_LOCK.get_or_init(|| Mutex::new(()));
    let Ok(_gaurd) = lock.try_lock() else {
        return Err(anyhow::anyhow!("update_lyric already in queue"));
    };

    crate::sync::utils::clean_lyric(window);

    if window.imp().cache_lyrics.get() {
        cache::fetch_lyric_cached(track_meta, ignore_cache, window).await?
    } else {
        fetch::fetch_lyric(track_meta, window).await?
    };

    drop(_gaurd);
    Ok(())
}

pub fn register_sync_task(app: WeakRef<Application>, interval: Duration) {
    glib::timeout_add_local(interval, move || {
        let Some(app) = app.upgrade() else {
            return glib::ControlFlow::Break;
        };

        let mut windows = app.windows();
        if windows.is_empty() {
            return glib::ControlFlow::Continue;
        }
        let window: app::Window = windows.remove(0).downcast().unwrap();

        match try_sync_track(&window) {
            Err(PlayerStatus::Missing) => {
                reconnect_player();
                reset_lyric_labels(&window);
                clean_lyric(&window);
                TRACK_PLAYING_STATE.take();
            }
            Err(PlayerStatus::Unsupported(kind)) => {
                app::get_label(&window, "above").set_label("Unsupported Player");
                app::get_label(&window, "below").set_label(kind);

                clean_lyric(&window);
                error!(kind);
            }
            Err(PlayerStatus::Paused) => {
                TRACK_PLAYING_STATE.with_borrow_mut(|TrackState { paused, .. }| *paused = true)
            }
            Err(PlayerStatus::Stopped) => {
                reset_lyric_labels(&window);
                clean_lyric(&window);
                TRACK_PLAYING_STATE.take();
            }
            _ => (),
        }

        glib::ControlFlow::Continue
    });
}
