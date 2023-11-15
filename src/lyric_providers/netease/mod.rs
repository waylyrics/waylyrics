use anyhow::Result;
use std::time::Duration;

use ncmapi::{
    types::{Album, Artist, Song},
    NcmApi,
};

use ncmapi::types::{LyricResp, SearchSongResp};

use crate::tokio_spawn;

use super::{default_search_query, Lyric, LyricOwned, LyricStore};

#[derive(Clone, Copy)]
pub struct Netease;

#[async_trait::async_trait]
impl super::LyricProvider for Netease {
    fn unique_name(&self) -> &'static str {
        "网易云音乐"
    }
    async fn search_song_detailed(
        &self,
        album: &str,
        artists: &[&str],
        title: &str,
    ) -> Result<Vec<super::SongInfo>> {
        let keyword = default_search_query(album, artists, title);
        self.search_song(&keyword).await
    }

    async fn query_lyric(&self, id: &str) -> Result<LyricStore> {
        let id = id.to_owned();
        tokio_spawn!(async move {
            let cookie_path = crate::CONFIG_HOME
                .get()
                .expect("should set CONFIG_HOME")
                .to_owned()
                + "ncm-cookie";
            let api = NcmApi::new(true, &cookie_path);
            let id = id.parse()?;
            let query_result = api.lyric(id).await?;

            let lyric_resp: LyricResp = query_result.deserialize()?;

            tracing::debug!("lyric query result: {lyric_resp:?}");

            Ok(LyricStore {
                lyric: lyric_resp.lrc.map(|l| l.lyric),
                tlyric: lyric_resp.tlyric.map(|l| l.lyric),
            })
        }).await?
    }

    async fn search_song(&self, keyword: &str) -> Result<Vec<super::SongInfo>> {
        let keyword = keyword.to_owned();
        tokio_spawn!(async move {
            tracing::debug!("search keyword: {keyword}");

            let cookie_path = crate::CONFIG_HOME
                .get()
                .expect("should set CONFIG_HOME")
                .to_owned()
                + "ncm-cookie";
            let api = NcmApi::new(true, &cookie_path);
            let search_result = api.search(&keyword, None).await?;
            let resp: SearchSongResp = search_result.deserialize()?;
            tracing::debug!("search result: {resp:?}");

            Ok(resp
                .result
                .ok_or(super::Error::NoResult)?
                .songs
                .iter()
                .map(
                    |Song {
                        name,
                        id,
                        artists,
                        duration,
                        album: Album { name: album, .. },
                        ..
                    }| super::SongInfo {
                        id: id.to_string(),
                        title: name.into(),
                        album: album.clone(),
                        singer: artists
                            .iter()
                            .filter_map(|Artist { name, .. }| name.as_ref())
                            .fold(String::new(), |mut s, op| {
                                if !s.is_empty() {
                                    s.push(',')
                                }
                                s += op;
                                s
                            }),
                        length: Duration::from_millis(*duration as _),
                    },
                )
                .collect())
        }).await?
    }
}

impl super::LyricParse for Netease {
    fn get_lyric(&self, store: &LyricStore) -> LyricOwned {
        let lyric = store.lyric.as_deref();
        match_lyric(lyric).into_owned()
    }

    fn get_translated_lyric(&self, store: &LyricStore) -> LyricOwned {
        let lyric = store.tlyric.as_deref();
        match_lyric(lyric).into_owned()
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
