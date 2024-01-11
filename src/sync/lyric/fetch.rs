pub mod tricks;

use anyhow::Result;
use std::borrow::Cow;
use std::sync::Arc;
use tokio::task::JoinSet;

use crate::log::{debug, error, info};
use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::lyric_providers::LyricOwned;
use crate::sync::{TrackMeta, LYRIC};
use crate::{app, tokio_spawn, LYRIC_PROVIDERS};

use crate::sync::utils::{self, match_likely_lyric};

pub(crate) use tricks::LyricHint;

pub async fn fetch_lyric(track_meta: &TrackMeta, window: &app::Window) -> Result<()> {
    utils::clean_lyric(window);

    let title = Arc::new(track_meta.title.as_deref().unwrap_or("Unknown").to_owned());
    let album = Arc::new(track_meta.album.as_ref().map(|album| album.to_owned()));
    let artists = &track_meta.artists;
    let length = track_meta.length;

    let artists_str = artists
        .as_ref()
        .map(|s| Cow::Owned(s.join(",")))
        .unwrap_or(Cow::Borrowed("Unknown"));

    if let Some(result) = tricks::get_lyric_hint_from_player(&title, &artists_str, window) {
        info!("fetched lyric directly");
        return result;
    }

    let providers = LYRIC_PROVIDERS
        .get()
        .expect("lyric providers should be initialized");

    let artists = Arc::new(artists.as_ref().unwrap_or(&vec![]).clone());

    let length_toleration_ms = window.imp().length_toleration_ms.get();

    let (mut results, artists_str, title) = tokio_spawn!(async move {
        let mut set = JoinSet::new();
        for (idx, provider) in providers.iter().enumerate() {
            let title = title.clone();
            let provider = provider.clone();
            let artists = artists.clone();
            let album = album.clone();

            set.spawn(async move {
                let artists = artists.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
                let album = album.as_deref();
                let search_result = provider
                    .search_song_detailed(album.unwrap_or_default(), &artists, &title)
                    .await;
                search_result.map(|songs| {
                    match_likely_lyric(
                        album.zip(Some(&title)),
                        length,
                        &songs,
                        length_toleration_ms,
                    )
                    .map(|(id, weight)| (id.to_owned(), weight, idx))
                })
            });
        }

        let artists_str = artists_str.to_string();
        let title = title.clone();
        let mut results = vec![];
        while let Some(Ok(re)) = set.join_next().await {
            let Ok(Some((id, weight, idx))) = re else {
                continue;
            };
            results.push((id, weight, idx));
        }
        (results, artists_str, title)
    })
    .await?;

    if results.is_empty() {
        info!("Failed searching for {artists_str} - {title}",);
        Err(crate::lyric_providers::Error::NoResult)?;
    };

    let providers = LYRIC_PROVIDERS
        .get()
        .expect("lyric providers should be initialized");

    results.sort_by_key(|(_, _, weight)| *weight);

    for (song_id, weight, platform_idx) in results {
        let provider = &providers[platform_idx];
        match provider.query_lyric(&song_id).await {
            Ok(lyric) => {
                let olyric = provider.get_lyric(&lyric);
                let tlyric = provider.get_translated_lyric(&lyric);

                info!(
                    "fetched {song_id} from {} with weight {weight}",
                    provider.unique_name()
                );
                set_lyric(olyric, tlyric, &title, &artists_str, window);
                return Ok(());
            }
            Err(e) => {
                error!(
                    "{e} when get lyric for {title} on {}",
                    provider.unique_name()
                );
            }
        };
    }
    Err(crate::lyric_providers::Error::NoResult)?
}

fn set_lyric(
    olyric: LyricOwned,
    tlyric: LyricOwned,
    title: &str,
    artists: &str,
    window: &app::Window,
) {
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
}
