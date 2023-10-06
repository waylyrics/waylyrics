mod tricks;

use anyhow::Result;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use std::borrow::Cow;
use std::sync::Arc;

use gtk::prelude::*;
use tracing::{debug, error, info};

use crate::lyric_providers::LyricOwned;
use crate::sync::{TrackMeta, LYRIC};
use crate::{app, LYRIC_PROVIDERS};

use crate::sync::utils;

pub fn fetch_lyric(track_meta: &TrackMeta, window: &app::Window) -> Result<()> {
    let title = track_meta.title.as_str();
    let album = track_meta.album.as_ref().map(|album| album.as_str());
    let artists = &track_meta.artists;
    let length = track_meta.length;

    let artists_str = artists
        .as_ref()
        .map(|s| Cow::Owned(s.join(",")))
        .unwrap_or(Cow::Borrowed("Unknown"));

    let artists = if let Some(artists) = artists {
        artists.iter().map(|s| s.as_str()).collect()
    } else {
        vec![]
    };

    if let Some(result) = tricks::get_accurate_lyric(title, &artists_str, window) {
        info!("fetched lyric directly");
        return result;
    }

    let mut results: Vec<(usize, String, u8)> = vec![];
    let artists = Arc::new(artists);
    let providers = LYRIC_PROVIDERS.with_borrow(|providers| providers.clone());
    let length_toleration_ms = window.imp().length_toleration_ms.get();

    tokio_scoped::scope(|s| {
        s.spawn(async {
            for (idx, provider) in providers.iter().enumerate() {
                let provider_id = provider.unique_name();
                let artists = artists.clone();

                let search = provider
                    .search_song_detailed(album.unwrap_or_default(), &artists, title)
                    .await;
                let tracks = match search {
                    Ok(songs) => songs,
                    Err(e) => {
                        error!("{e} occurs when search {title} on {}", provider_id);
                        continue;
                    }
                };

                if let Some((song_id, weight)) = utils::match_likely_lyric(
                    album.zip(Some(title)),
                    length,
                    &tracks,
                    length_toleration_ms,
                ) {
                    info!("matched {song_id} from {}", provider_id);
                    results.push((idx, song_id.to_string(), weight));
                }
            }
        });
    });

    if results.is_empty() {
        info!("Failed searching for {artists_str} - {title}",);
        utils::clean_lyric(window);
        Err(crate::lyric_providers::Error::NoLyric)?;
    }

    results.sort_by_key(|(_, _, weight)| *weight);

    for (platform_idx, song_id, _) in results {
        let provider = LYRIC_PROVIDERS.with_borrow(|p| p[platform_idx].clone());
        let fetch_result = {
            match tokio::runtime::Handle::current().block_on(provider.query_lyric(&song_id)) {
                Ok(lyric) => {
                    let olyric = provider.get_lyric(&lyric);
                    let tlyric = provider.get_translated_lyric(&lyric);
                    set_lyric(olyric, tlyric, title, &artists_str, window)
                }
                Err(e) => {
                    error!(
                        "{e} when get lyric for {title} on {}",
                        provider.unique_name()
                    );
                    Err(crate::lyric_providers::Error::NoResult)?
                }
            }
        };

        if fetch_result.is_ok() {
            return Ok(());
        }
    }

    Err(crate::lyric_providers::Error::NoResult)?
}

fn set_lyric(
    olyric: LyricOwned,
    tlyric: LyricOwned,
    title: &str,
    artists: &str,
    window: &app::Window,
) -> Result<()> {
    debug!("original lyric: {olyric:?}");
    debug!("translated lyric: {tlyric:?}");

    // show info to user if original lyric is empty or no timestamp
    match &olyric {
        LyricOwned::LineTimestamp(_) => (),
        _ => {
            info!("No lyric for {} - {title}", artists,);
        }
    }

    if !matches!(tlyric, LyricOwned::LineTimestamp(_)) {
        info!("No translated lyric for {} - {title}", artists,);
        app::get_label(window, "below").set_visible(false);
    }
    LYRIC.set((olyric, tlyric));

    Ok(())
}
