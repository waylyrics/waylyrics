use anyhow::Result;
use gtk::glib::Variant;
use std::path::PathBuf;

use crate::log::{debug, error, info, warn};
use crate::GTK_DBUS_CONNECTION;
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use serde::{Deserialize, Serialize};

use crate::sync::{lyric::fetch::fetch_lyric, LYRIC};
use crate::sync::{LyricState, TrackMeta};
use crate::{app, lyric_providers::LyricOwned, CACHE_DIR};

/// This will not create cache dir for you -- you should create it yourself.
///
/// Note that window.imp().cache_lyrics controls whether to cache lyrics.
///
/// When `track_meta.title == None`, this returns `None` as well,
///
/// because we should not cache lyric for an unknown song
pub fn get_cache_path(track_meta: &TrackMeta) -> Option<PathBuf> {
    match track_meta {
        TrackMeta {
            title: Some(title),
            album,
            artists,
            length,
            ..
        } => {
            let cache_key = format!("{title}-{artists:?}-{album:?}-{length:?}");
            debug!("get_cache_path: received {cache_key}");
            let digest = md5::compute(&cache_key);

            let cache_dir = CACHE_DIR
                .with_borrow(|cache_home| PathBuf::from(cache_home).join(md5_cache_dir(digest)));
            Some(cache_dir.join(format!("{digest:x}.json")))
        }

        _ => None,
    }
}

pub async fn fetch_lyric_cached(
    track_meta: &TrackMeta,
    ignore_cache: bool,
    window: &app::Window,
) -> Result<()> {
    let Some(cache_path) = get_cache_path(track_meta) else {
        warn!("cannot cache lyric due to missing title");
        return fetch_lyric(track_meta, window).await;
    };

    info!(
        "cache_path for {}: {cache_path:?}",
        track_meta.title.as_deref().unwrap()
    );

    if !ignore_cache {
        if let Ok(lyric) = std::fs::read_to_string(&cache_path) {
            let cached_lyric: Result<LyricCache, _> = serde_json::from_str(&lyric);
            match cached_lyric {
                Ok(LyricCache {
                    olyric: origin,
                    tlyric: translation,
                    offset,
                }) => {
                    let dbus_conn = GTK_DBUS_CONNECTION
                        .with_borrow(|conn| conn.as_ref().cloned())
                        .expect("GApplication was not set");
                    let _ = dbus_conn.emit_signal(
                        None,
                        "/io/github/waylyrics/Waylyrics",
                        crate::APP_ID,
                        "LoadLyricCache",
                        Some(&Variant::tuple_from_iter([cache_path
                            .to_string_lossy()
                            .to_variant()])),
                    );
                    LYRIC.set(LyricState {
                        origin,
                        translation,
                    });
                    window.imp().lyric_offset_ms.set(offset);
                    info!("set offset: {offset}ms");
                    return Ok(());
                }
                Err(e) => error!("cache parse error: {e} from {cache_path:?}"),
            }
        }
    }

    let result = fetch_lyric(track_meta, window).await;
    if result.is_ok() && update_lyric_cache(&cache_path) {
        let dbus_conn = GTK_DBUS_CONNECTION
            .with_borrow(|conn| conn.as_ref().cloned())
            .expect("GApplication was not set");
        let _ = dbus_conn.emit_signal(
            None,
            "/io/github/waylyrics/Waylyrics",
            crate::APP_ID,
            "NewLyricCache",
            Some(&Variant::tuple_from_iter([cache_path
                .to_string_lossy()
                .to_variant()])),
        );
    }
    result
}

/// Using olyric and tlyric inside LYRIC to update corresponding cache file.
pub fn update_lyric_cache(cache_path: &PathBuf) -> bool {
    let cache_dir = cache_path.parent().unwrap();
    if let Err(e) = std::fs::create_dir_all(cache_dir) {
        error!("cannot create cache dir {cache_dir:?}: {e}");
        return false;
    }

    LYRIC.with_borrow(
        |LyricState {
             origin,
             translation,
         }| {
            // do not cache empty lyric
            if origin.is_none() && translation.is_none() {
                return false;
            }

            let Err(e) = std::fs::write(
                cache_path,
                serde_json::to_string(&LyricCache {
                    olyric: origin.clone(),
                    tlyric: translation.clone(),
                    offset: 0,
                })
                .expect("cannot serialize lyrics!"),
            ) else {
                info!("cached to {cache_path:?}");
                return true;
            };

            error!("cannot write cache {cache_path:?}: {e}");
            false
        },
    )
}

#[derive(Deserialize, Serialize)]
struct LyricCache {
    olyric: LyricOwned,
    tlyric: LyricOwned,
    offset: i64,
}

fn md5_cache_dir(digest: md5::Digest) -> PathBuf {
    let mut cache_path = PathBuf::new();
    for i in 0..3 {
        cache_path.push(format!("{:02x}", digest[i]));
    }
    cache_path
}
