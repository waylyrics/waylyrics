use anyhow::Result;
use qqmusic_rs::{
    song::{SongDetail, SongDetailResp},
    lyric::{QueryLyric, QueryLyricResp},
    SongId,
};
use reqwest::blocking::Client;

use crate::QQMUSIC_API_CLIENT;

use super::{Lyric, LyricStore};

#[derive(Clone, Copy)]
pub struct QQMusicLyricProvider;

impl super::LyricProvider for QQMusicLyricProvider {
    fn provider_unique_name(&self) -> &'static str {
        "QQ音乐"
    }

    fn search_song(
        &self,
        album: &str,
        artists: &[&str],
        title: &str,
    ) -> Result<Vec<super::SongInfo>> {
        let keyword = format!("{title} {album} {}", artists.join("/"));
        tracing::debug!("search keyword: {keyword}");

        Err(Error::NotImplemented)
    }

    fn query_lyric(&self, id: &str) -> Result<LyricStore> {
        let client = Client::builder().user_agent("Waylyrics/0.1").build()?;

        // might be a little tricky
        let songid = if id.parse::<u32>().is_ok() {
            SongId::Songid(id)
        } else {
            SongId::Songmid(id)
        };

        QQMUSIC_API_CLIENT.with_borrow(|api| {
            let Some(api) = api.as_ref() else {
                return Err(Error::ApiClientNotInit)?;
            };
            
            let mid = match songid {
                SongId::Songmid(mid) => mid.to_owned(),
                SongId::Songid(id) => get_songmid(api, &client, songid)?,
            };

            let url = api.query_lyric(SongId::Songmid(&mid));
            let resp: QueryLyricResp =
                serde_json::from_slice(client.get(url).send()?.bytes()?.as_ref())?;

            Ok(LyricStore {
                lyric: Some(resp.data.lyric),
                tlyric: Some(resp.data.trans),
            })
        })
    }
}

fn get_songmid(api: &QQMusicApi, client: &Client, songid: &str) -> Result<String> {
    let url = api.song_detail(SongId::Songid(songid));
    let resp: SongDetailResp = serde_json::from_slice(client.get(url).send()?.bytes()?.as_ref())?;
    Ok(resp.data.track_info.mid)
}

impl super::LyricParse for QQMusicLyricProvider {
    fn get_lyric<'a>(&self, store: &'a LyricStore) -> Lyric<'a> {
        let lyric = store.lyric.as_deref();
        match_lyric(lyric)
    }

    fn get_translated_lyric<'a>(&self, store: &'a LyricStore) -> Lyric<'a> {
        let lyric = store.tlyric.as_deref();
        match_lyric(lyric)
    }
}

fn match_lyric(lyric: Option<&str>) -> Lyric<'_> {
    match lyric {
        Some("") | None => super::Lyric::None,
        Some(lyric) => {
            if let Ok(parsed) = super::utils::lrc_iter(lyric.split('\n')) {
                Lyric::LineTimestamp(parsed)
            } else {
                Lyric::NoTimestamp
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("please make sure you configured QQMusicApi base URL")]
    ApiClientNotInit,
    #[error("Not implemented")]
    NotImplemented,
}
