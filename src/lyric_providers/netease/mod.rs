use anyhow::Result;
use std::time::Duration;

use ncmapi::{
    types::{Album, Artist, Song},
    NcmApi,
};

use ncmapi::types::{LyricResp, SearchSongResp};

use crate::tokio_spawn;

use super::{
    default_search_query, dict::ARTIST_ALIAS_MAP, Lyric, LyricLineOwned, LyricOwned, LyricStore,
};

#[derive(Clone, Copy)]
pub struct Netease;

#[async_trait::async_trait]
impl super::LyricProvider for Netease {
    fn init(self, _config: &str) -> Result<()> {
        Ok(())
    }

    fn unique_name(&self) -> &'static str {
        "网易云音乐"
    }

    async fn search_song_detailed(
        &self,
        album: &str,
        artists: &[&str],
        title: &str,
    ) -> Result<Vec<super::SongInfo>> {
        let artists = artists
            .iter()
            .map(|&a| *ARTIST_ALIAS_MAP.get(&*a.to_lowercase()).unwrap_or(&a))
            .collect::<Vec<&str>>();
        let keyword = default_search_query(album, &artists, title);
        self.search_song(&keyword).await
    }

    async fn query_lyric(&self, id: &str) -> Result<LyricStore> {
        let id = id.to_owned();
        tokio_spawn!(async move {
            let api = NcmApi::new(false, "");
            let id = id.parse()?;
            let query_result = api.lyric(id).await?;

            let lyric_resp: LyricResp = query_result.deserialize()?;

            crate::log::debug!("lyric query result: {lyric_resp:?}");

            Ok(LyricStore {
                lyric: lyric_resp.lrc.map(|l| l.lyric),
                tlyric: lyric_resp.tlyric.map(|l| l.lyric),
            })
        })
        .await?
    }

    async fn search_song(&self, keyword: &str) -> Result<Vec<super::SongInfo>> {
        let keyword = keyword.to_owned();
        tokio_spawn!(async move {
            crate::log::debug!("search keyword: {keyword}");

            let api = NcmApi::new(false, "");
            let search_result = api.search(&keyword, None).await?;
            let resp: SearchSongResp = search_result.deserialize()?;
            crate::log::debug!("search result: {resp:?}");

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
                            .fold(String::default(), |mut s, op| {
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
        })
        .await?
    }

    fn is_likely_songid(&self, s: &str) -> bool {
        s.parse::<u32>().is_ok()
    }
}

impl super::LyricParse for Netease {
    fn parse_lyric(&self, store: &LyricStore) -> LyricOwned {
        let lyric = store.lyric.as_deref();
        verify_lyric(lyric).into_owned()
    }

    fn parse_translated_lyric(&self, store: &LyricStore) -> LyricOwned {
        let olyric = self.parse_lyric(store);
        let tlyric = store.tlyric.as_deref();
        let mut tlyric = verify_lyric(tlyric).into_owned();

        // workaround: hide translation if original line ends
        if let (LyricOwned::LineTimestamp(tlyric), LyricOwned::LineTimestamp(olyric)) =
            (&mut tlyric, &olyric)
        {
            let last_t = tlyric.last();
            if let Some(next_line_time) = last_t.and_then(|t| {
                olyric
                    .iter()
                    .find(|o| o.start_time > t.start_time)
                    .map(|o| o.start_time)
            }) {
                tlyric.push(LyricLineOwned {
                    text: String::default(),
                    start_time: next_line_time,
                });
            }
        }
        tlyric
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
