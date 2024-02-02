use anyhow::Error;
use gtk::subclass::prelude::*;
use tokio::sync::Mutex;

use std::sync::OnceLock;

use crate::{
    app,
    sync::{
        lyric::{cache, fetch},
        TrackMeta,
    },
};

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
