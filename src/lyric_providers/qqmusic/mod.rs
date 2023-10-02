use std::time::Duration;

use anyhow::Result;
use qqmusic_rs::{
    lyric::{QueryLyric, QueryLyricResp},
    search::{Search, SearchType, Track},
    song::{SongDetail, SongDetailResp},
    QQMusicApi, SongId,
};
use reqwest::blocking::Client;

use crate::{lyric_providers::{SongInfo, default_search_query}, QQMUSIC_API_CLIENT};

use super::{Lyric, LyricOwned, LyricStore};

#[derive(Clone, Copy)]
pub struct QQMusicLyricProvider;

impl super::LyricProvider for QQMusicLyricProvider {
    fn provider_unique_name(&self) -> &'static str {
        "QQ音乐"
    }

    fn search_song(&self, album: &str, artists: &[&str], title: &str) -> Result<Vec<SongInfo>> {
        let keyword = default_search_query(album, artists, title);
        tracing::debug!("search keyword: {keyword}");

        let client = Client::builder().user_agent("Waylyrics/0.1").build()?;

        QQMUSIC_API_CLIENT.with_borrow(|api| {
            let Some(api) = api.as_ref() else {
                return Err(Error::ApiClientNotInit)?;
            };

            let url = api.search::<Track>(&keyword, None, None);
            let resp: <Track as SearchType>::Resp =
                serde_json::from_slice(client.get(url).send()?.bytes()?.as_ref())?;

            Ok(resp
                .data
                .list
                .into_iter()
                .map(|song| SongInfo {
                    id: song.songmid,
                    title: song.songname,
                    singer: song.singer.iter().map(|singer| &singer.name).fold(
                        String::new(),
                        |mut s, op| {
                            if !s.is_empty() {
                                s.push(',')
                            }
                            s += &op;
                            s
                        },
                    ),
                    album: Some(song.albumname),
                    length: Duration::from_secs(song.interval as _),
                })
                .collect())
        })
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
                SongId::Songid(id) => get_songmid(api, &client, id)?,
            };

            let url = api.query_lyric(&mid);
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
    fn get_lyric<'a>(&self, store: &'a LyricStore) -> LyricOwned {
        let lyric = store.lyric.as_deref();
        verify_lyric(lyric)
    }

    fn get_translated_lyric<'a>(&self, store: &'a LyricStore) -> LyricOwned {
        let lyric = store.tlyric.as_deref();
        verify_lyric(lyric)
    }
}

fn verify_lyric(lyric: Option<&str>) -> LyricOwned {
    match lyric {
        Some("") | None => super::LyricOwned::None,
        Some(lyric) => {
            let lyric = lyric
                .replace("&amp;", "&")
                .replace("&lt;", "<")
                .replace("&gt;", ">")
                .replace("&quot;", "\"")
                .replace("&apos;", "\'");

            if let Ok(parsed) = super::utils::lrc_iter(lyric.split('\n')) {
                Lyric::LineTimestamp(parsed).into_owned()
            } else {
                LyricOwned::NoTimestamp
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
