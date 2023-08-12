use anyhow::Result;
use async_compat::CompatExt;
use std::time::Duration;

use ncmapi::{
    types::{Album, Artist, Song},
    NcmApi,
};

use ncmapi::types::{LyricResp, SearchSongResp};

use super::{Lyric, LyricStore};

pub struct NeteaseLyricProvider;

impl super::LyricProvider for NeteaseLyricProvider {
    fn provider_name(&self) -> &'static str {
        "网易云音乐"
    }
    fn search_song(
        &self,
        album: &str,
        artists: &[&str],
        title: &str,
    ) -> Result<Vec<super::SongInfo>> {
        let keyword = format!("{title} {album} {}", artists.join("/"));

        tracing::debug!("search keyword: {keyword}");

        let cookie_path = crate::CONFIG_HOME.with_borrow(|home| home.to_owned() + "ncm-cookie");
        let api = NcmApi::new(
            false,
            Duration::from_secs(60 * 60),
            Duration::from_secs(5 * 60),
            true,
            &cookie_path,
        );
        let search_result = smol::block_on(async { api.search(&keyword, None).compat().await })?;
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
    }

    fn query_lyric(&self, id: &str) -> Result<LyricStore> {
        let cookie_path = crate::CONFIG_HOME.with_borrow(|home| home.to_owned() + "ncm-cookie");
        let api = NcmApi::new(
            false,
            Duration::from_secs(60 * 60),
            Duration::from_secs(5 * 60),
            true,
            &cookie_path,
        );
        let id = id.parse()?;
        let query_result = smol::block_on(async { api.lyric(id).compat().await })?;

        let lyric_resp: LyricResp = query_result.deserialize()?;

        tracing::debug!("lyric query result: {lyric_resp:?}");

        Ok(LyricStore {
            lyric: lyric_resp.lrc.map(|l| l.lyric),
            tlyric: lyric_resp.tlyric.map(|l| l.lyric),
        })
    }
}

impl super::LyricParse for NeteaseLyricProvider {
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
