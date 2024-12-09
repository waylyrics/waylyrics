use std::time::Duration;

use anyhow::Result;
use lrclib_api_rs::{
    types::{ErrorResponse, GetLyricsResponse, LyricsData},
    LRCLibAPI,
};
use once_cell::sync::Lazy;
use reqwest::Client;

use super::{Lyric, LyricOwned, LyricStore, SongInfo};
use crate::tokio_spawn;

pub struct LRCLib;

pub static LRCLIB_API_CLIENT: Lazy<LRCLibAPI> = Lazy::new(LRCLibAPI::default);
pub static REQWEST_CLIENT: Lazy<Client> = Lazy::new(Client::default);

impl super::LyricParse for LRCLib {
    fn parse_lyric(&self, store: &LyricStore) -> LyricOwned {
        let LyricStore { lyric, .. } = store;
        verify_lyric(lyric.as_deref()).into_owned()
    }

    fn parse_translated_lyric(&self, store: &LyricStore) -> LyricOwned {
        let LyricStore { tlyric, .. } = store;
        verify_lyric(tlyric.as_deref()).into_owned()
    }
}

#[async_trait::async_trait]
impl super::LyricProvider for LRCLib {
    async fn query_lyric(&self, id: &str) -> Result<LyricStore> {
        let id: u64 = id.parse()?;

        tokio_spawn!(async move {
            let req = LRCLIB_API_CLIENT.get_lyrics_by_id(id)?;
            let resp = REQWEST_CLIENT.get(req.uri().to_string()).send().await?;
            let result: GetLyricsResponse = resp.json().await?;
            match result {
                GetLyricsResponse::Success(LyricsData { synced_lyrics, .. }) => Ok(LyricStore {
                    lyric: synced_lyrics,
                    tlyric: None,
                }),
                GetLyricsResponse::Error(ErrorResponse { message, .. }) => {
                    crate::log::debug!("query failed: {message}");
                    Err(super::Error::NoResult)?
                }
            }
        })
        .await?
    }
    async fn search_song_detailed(
        &self,
        album: &str,
        artists: &[&str],
        title: &str,
    ) -> Result<Vec<SongInfo>> {
        let album = if album.is_empty() {
            None
        } else {
            Some(album.to_owned())
        };
        let artist = if artists.is_empty() {
            None
        } else {
            Some(artists.join(" / "))
        };
        let title = title.to_owned();

        tokio_spawn!(async move {
            let req = LRCLIB_API_CLIENT.search_lyrics_detailed(
                &title,
                artist.as_deref(),
                album.as_deref(),
            )?;
            let resp = REQWEST_CLIENT.get(req.uri().to_string()).send().await?;
            let result: Vec<LyricsData> = resp.json().await.map_err(|_| super::Error::NoResult)?;
            Ok(map_lyrics_result(result))
        })
        .await?
    }

    async fn search_song(&self, keyword: &str) -> Result<Vec<SongInfo>> {
        let keyword = keyword.to_owned();

        tokio_spawn!(async move {
            let req = LRCLIB_API_CLIENT.search_lyrics_query(&keyword)?;
            let resp = REQWEST_CLIENT.get(req.uri().to_string()).send().await?;
            let result: Vec<LyricsData> = resp.json().await.map_err(|_| super::Error::NoResult)?;
            Ok(map_lyrics_result(result))
        })
        .await?
    }
    fn unique_name(&self) -> &'static str {
        "LRCLib"
    }
    fn init(self, _config: &str) -> Result<()> {
        Ok(())
    }
    fn is_likely_songid(&self, s: &str) -> bool {
        s.parse::<u64>().is_ok()
    }
}

fn verify_lyric(lyric: Option<&str>) -> Lyric<'_> {
    match lyric {
        Some("") | None => super::Lyric::None,
        Some(lyric) => {
            if let Ok(parsed) = super::utils::lrc_iter(lyric.lines()) {
                Lyric::LineTimestamp(parsed)
            } else {
                Lyric::NoTimestamp
            }
        }
    }
}

fn map_lyrics_result(data: Vec<LyricsData>) -> Vec<SongInfo> {
    data.into_iter()
        .map(
            |LyricsData {
                 id,
                 track_name,
                 artist_name,
                 album_name,
                 duration,
                 ..
             }| {
                SongInfo {
                    id: id.to_string(),
                    title: track_name,
                    singer: artist_name,
                    album: album_name,
                    length: duration.map(Duration::from_secs_f64).unwrap_or_default(),
                }
            },
        )
        .collect()
}
