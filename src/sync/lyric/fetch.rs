mod tricks;

use anyhow::Result;
use std::borrow::Cow;

use gtk::prelude::*;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use mpris::{Metadata, Player};
use tracing::{debug, error, info};

use crate::lyric_providers::{LyricOwned, LyricProvider, SongInfo};
use crate::sync::{TrackMeta, LYRIC};
use crate::{app, LYRIC_PROVIDERS};

use crate::sync::utils;

pub fn fetch_lyric(track_meta: &TrackMeta, window: &app::Window) -> Result<()> {
    let title = &track_meta.title;
    let album = track_meta.meta.album_name();
    let artists = track_meta.meta.artists();
    let artists = artists.as_deref();
    let length = track_meta.meta.length();

    let artists_str = artists
        .map(|s| Cow::Owned(s.join(",")))
        .unwrap_or(Cow::Borrowed("Unknown"));

    if let Some(result) = tricks::get_accurate_lyric(title, &artists_str, window) {
        info!("fetched lyric directly");
        return result;
    }

    let mut results: Vec<(usize, String, u8)> = vec![];
    LYRIC_PROVIDERS.with_borrow(|providers| {
        for (idx, provider) in providers.iter().enumerate() {
            let provider_id = provider.provider_unique_name();
            let tracks = match search_song(
                provider.as_ref(),
                album.unwrap_or_default(),
                artists.unwrap_or(&[]),
                title,
            ) {
                Ok(songs) => songs,
                Err(e) => {
                    error!("{e} occurs when search {title} on {}", provider_id);
                    continue;
                }
            };

            let length_toleration_ms = window.imp().length_toleration_ms.get();
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

    if results.is_empty() {
        info!("Failed searching for {artists_str} - {title}",);
        utils::clean_lyric(window);
        Err(crate::lyric_providers::Error::NoLyric)?;
    }

    results.sort_by_key(|(_, _, weight)| *weight);

    for (platform_idx, song_id, _) in results {
        let fetch_result = LYRIC_PROVIDERS.with_borrow(|providers| {
            let provider = &providers[platform_idx];
            match provider.query_lyric(&song_id) {
                Ok(lyric) => {
                    let olyric = provider.get_lyric(&lyric);
                    let tlyric = provider.get_translated_lyric(&lyric);
                    set_lyric(olyric, tlyric, title, &artists_str, window)
                }
                Err(e) => {
                    error!(
                        "{e} when get lyric for {title} on {}",
                        provider.provider_unique_name()
                    );
                    Err(crate::lyric_providers::Error::NoResult)?
                }
            }
        });

        if fetch_result.is_ok() {
            return Ok(());
        }
    }

    Err(crate::lyric_providers::Error::NoResult)?
}

pub fn search_song<P: LyricProvider + ?Sized>(
    provider: &P,
    album: &str,
    artists: &[&str],
    title: &str,
) -> Result<Vec<SongInfo>> {
    provider.search_song(album, artists, title)
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

fn get_song_id_from_player(
    player: &Player,
    extract_field: impl for<'a> FnOnce(&'a Metadata) -> Option<&'a str>,
) -> Option<String> {
    player
        .get_metadata()
        .ok()
        .and_then(|metadata| extract_field(&metadata).map(|s| s.to_owned()))
}
