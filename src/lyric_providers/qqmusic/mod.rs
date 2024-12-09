use std::{sync::OnceLock, time::Duration};

use anyhow::Result;
use qqmusic_rs::{
    lyric::{QueryLyric, QueryLyricResp},
    search::{Search, SearchType, Track},
    song::{SongDetail, SongDetailResp},
    user::SetCookie,
    QQMusicApi, SongId,
};
use reqwest::Client;
use url::Url;

use crate::{
    lyric_providers::{default_search_query, SongInfo},
    tokio_spawn,
};

use super::{Lyric, LyricOwned, LyricStore};

mod typo;
pub use typo::QQMusicConfig;

#[derive(Clone, Copy)]
pub struct QQMusic;

#[async_trait::async_trait]
impl super::LyricProvider for QQMusic {
    fn init(self, config: &str) -> Result<()> {
        let QQMusicConfig {
            api_base_url,
            cookies,
        } = serde_json::from_str(config)?;

        let base_url: Url = api_base_url.parse()?;
        QQMUSIC_API_CLIENT
            .set(Some(QQMusicApi::new(base_url)))
            .map_err(|_| Error::ApiClientInited)?;

        tokio_spawn!(async move {
            async fn login_qqmusic(cookies: &str, api: &QQMusicApi) -> Result<()> {
                let req = api.set_cookie(cookies)?;
                let reqw_req = reqwest::Request::try_from(req)?;
                let client = Client::builder().user_agent("Waylyrics/0.1").build()?;
                client.execute(reqw_req).await?;
                Ok(())
            }
            let Some(cookies) = cookies else { return };
            let Some(Some(api)) = QQMUSIC_API_CLIENT.get() else {
                return;
            };
            let _ = login_qqmusic(&cookies, api).await;
        });

        Ok(())
    }

    fn unique_name(&self) -> &'static str {
        "QQ音乐"
    }

    async fn search_song_detailed(
        &self,
        album: &str,
        artists: &[&str],
        title: &str,
    ) -> Result<Vec<SongInfo>> {
        let keyword = default_search_query(album, artists, title);
        self.search_song(&keyword).await
    }

    async fn query_lyric(&self, id: &str) -> Result<LyricStore> {
        let id = id.to_owned();
        tokio_spawn!(async move {
            let client = Client::builder().user_agent("Waylyrics/0.1").build()?;

            // might be a little tricky
            let songid = if id.parse::<usize>().is_ok() {
                SongId::Songid(&id)
            } else {
                SongId::Songmid(&id)
            };

            let Some(Some(api)) = QQMUSIC_API_CLIENT.get() else {
                return Err(Error::ApiClientNotInit)?;
            };

            let mid = match songid {
                SongId::Songmid(mid) => mid.to_owned(),
                SongId::Songid(id) => get_songmid(api, &client, id).await?,
            };

            let url = api.query_lyric(&mid)?.uri().to_string();
            let resp: QueryLyricResp =
                serde_json::from_slice(client.get(url).send().await?.bytes().await?.as_ref())?;

            if resp.data.code == -1901 {
                return Ok(LyricStore {
                    lyric: None,
                    tlyric: None,
                });
            }

            Ok(LyricStore {
                lyric: Some(resp.data.lyric),
                tlyric: Some(resp.data.trans),
            })
        })
        .await?
    }

    async fn search_song(&self, keyword: &str) -> Result<Vec<SongInfo>> {
        let keyword = keyword.to_owned();
        tokio_spawn!(async move {
            crate::log::debug!("search keyword: {keyword}");

            let client = Client::builder().user_agent("Waylyrics/0.1").build()?;

            let Some(Some(api)) = QQMUSIC_API_CLIENT.get() else {
                return Err(Error::ApiClientNotInit)?;
            };

            let url = api.search::<Track>(&keyword, None, None)?.uri().to_string();
            let resp: <Track as SearchType>::Resp =
                serde_json::from_slice(client.get(url).send().await?.bytes().await?.as_ref())?;

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
                            s += op;
                            s
                        },
                    ),
                    album: Some(song.albumname),
                    length: Duration::from_secs(song.interval as _),
                })
                .collect())
        })
        .await?
    }

    fn is_likely_songid(&self, s: &str) -> bool {
        (s.len() == 14 && s.starts_with('0')) || s.parse::<usize>().is_ok()
    }
}

async fn get_songmid(api: &QQMusicApi, client: &Client, songid: &str) -> Result<String> {
    let url = api.song_detail(SongId::Songid(songid))?.uri().to_string();
    let resp: SongDetailResp =
        serde_json::from_slice(client.get(url).send().await?.bytes().await?.as_ref())?;
    Ok(resp.data.track_info.mid)
}

impl super::LyricParse for QQMusic {
    fn parse_lyric(&self, store: &LyricStore) -> LyricOwned {
        let lyric = store.lyric.as_deref();
        verify_lyric(lyric)
    }

    fn parse_translated_lyric(&self, store: &LyricStore) -> LyricOwned {
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

            if let Ok(parsed) = super::utils::lrc_iter(lyric.lines()) {
                Lyric::LineTimestamp(parsed).into_owned()
            } else {
                LyricOwned::NoTimestamp
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Please make sure you had configured QQMusicApi base URL")]
    ApiClientNotInit,
    #[error("Not implemented")]
    NotImplemented,
    #[error("QQMusicApi already initialized")]
    ApiClientInited,
}

pub static QQMUSIC_API_CLIENT: OnceLock<Option<QQMusicApi>> = OnceLock::new();
