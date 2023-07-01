use std::{path::PathBuf, time::Duration};
use anyhow::Result;

use gtk::subclass::prelude::ObjectSubclassIsExt;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

use super::{player::fetch_lyric, LYRIC};
use crate::{app, lyric::LyricOwned, CACHE_DIR};

pub fn fetch_lyric_cached(
    title: &str,
    album: Option<&str>,
    artists: Option<&[&str]>,
    length: Option<Duration>,
    window: &app::Window,
) -> Result<()> {
    let digest = md5::compute(format!("{title}-{artists:?}-{album:?}-{length:?}"));
    let cache_dir =
        CACHE_DIR.with_borrow(|cache_home| PathBuf::from(cache_home).join(md5_cache_dir(digest)));
    let cache_path = cache_dir.join(format!("{digest:x}.json"));
    debug!(
        "cache_path for {title}-{artists:?}-{}-{length:?}: {cache_path:?}",
        album.unwrap_or("Unknown")
    );

    if let Err(e) = std::fs::create_dir_all(&cache_dir) {
        error!("cannot create cache dir {cache_dir:?}: {e}");
    }

    match std::fs::read_to_string(&cache_path) {
        Ok(lyric) => {
            let cached_lyric: Result<LyricCache, _> = serde_json::from_str(&lyric);
            match cached_lyric {
                Ok(LyricCache {
                    olyric,
                    tlyric,
                    offset,
                }) => {
                    LYRIC.set((olyric, tlyric));
                    window.imp().lyric_offset_ms.set(offset);
                    info!("set offset: {offset}ms");
                    return Ok(());
                }
                Err(e) => error!("cache parse error: {e} from {cache_path:?}"),
            }
        }
        Err(e) => info!("cache missed: {e}"),
    }
    let result = fetch_lyric(title, album, artists, length, window);
    if result.is_ok() {
        LYRIC.with_borrow(|(olyric, tlyric)| {
            if (&LyricOwned::None, &LyricOwned::None) == (olyric, tlyric) {
                return;
            }

            let Err(e) = std::fs::write(
                &cache_path,
                serde_json::to_string(&LyricCache {
                    olyric: olyric.clone(),
                    tlyric: tlyric.clone(),
                    offset: 0,
                })
                .expect("cannot serialize lyrics!"),)
            else {
                info!("cached to {cache_path:?}");
                return;
            };

            error!("cannot write cache {cache_path:?}: {e}");
        });
    }
    result
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
