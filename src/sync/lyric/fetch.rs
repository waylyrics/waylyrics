pub mod tricks;

use anyhow::Result;
use std::borrow::Cow;
use std::sync::Arc;
use tokio::task::JoinSet;

use crate::log::{debug, error, info};
use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::lyric_providers::LyricOwned;
use crate::sync::{LyricState, TrackMeta, LYRIC};
use crate::{app, tokio_spawn, LYRIC_PROVIDERS};

use crate::sync::utils::{self, match_likely_lyric};

pub(crate) use tricks::LyricHint;

use self::tricks::LyricHintResult;

pub async fn fetch_lyric(track_meta: &TrackMeta, window: &app::Window) -> Result<()> {
    utils::clean_lyric(window);

    let title = Arc::new(track_meta.title.as_deref().unwrap_or("Unknown").to_owned());
    let album = Arc::new(track_meta.album.as_ref().map(|album| album.to_owned()));
    let artists = &track_meta.artists;
    let length = track_meta.length;

    let artists_str = artists
        .as_ref()
        .map(|s| Cow::Owned(s.join(",")))
        .unwrap_or_else(|| Cow::Borrowed("Unknown"));

    if let Some(LyricHintResult::Lyric { olyric, tlyric }) =
        tricks::get_lyric_hint_from_player().await
    {
        info!("fetched lyrics by player hint");
        set_lyric(olyric, tlyric, &title, &artists_str);
        return Ok(());
    }

    let providers = LYRIC_PROVIDERS
        .get()
        .expect("lyric providers should be initialized");

    let artists = Arc::new(artists.as_ref().cloned().unwrap_or_else(std::vec::Vec::new).clone());

    let length_toleration_ms = window.imp().length_toleration_ms.get();

    let (mut results, artists_str, title) = tokio_spawn!(async move {
        let mut set = JoinSet::new();
        for (idx, provider) in providers.iter().enumerate() {
            let title = title.clone();
            let artists = artists.clone();
            let album = album.clone();

            set.spawn(async move {
                let artists = artists.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
                let title = title.as_ref();
                let album = album.as_deref();
                let singer = if artists.is_empty() {
                    None
                } else {
                    Some(artists.join(","))
                };
                let search_result = provider
                    .search_song_detailed(album.unwrap_or_default(), &artists, title)
                    .await;
                search_result.map(|songs| {
                    match_likely_lyric(
                        album,
                        title,
                        singer.as_deref(),
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
                let olyric = provider.parse_lyric(&lyric);
                let tlyric = provider.parse_translated_lyric(&lyric);

                info!(
                    "fetched {song_id} from {} with weight {weight}",
                    provider.unique_name()
                );
                set_lyric(olyric, tlyric, &title, &artists_str);
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

fn set_lyric(origin: LyricOwned, translation: LyricOwned, title: &str, artists: &str) {
    debug!("original lyric: {origin:?}");
    debug!("translated lyric: {translation:?}");

    // show info to user if original lyric is empty or no timestamp
    match &origin {
        LyricOwned::LineTimestamp(_) => (),
        _ => {
            info!("No lyric for {} - {title}", artists,);
        }
    }

    if !matches!(translation, LyricOwned::LineTimestamp(_)) {
        info!("No translated lyric for {} - {title}", artists,);
    }
    LYRIC.set(LyricState {
        origin,
        translation,
    });
}
