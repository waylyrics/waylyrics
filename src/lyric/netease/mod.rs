use async_compat::CompatExt;
use std::time::Duration;
use anyhow::Result;

use ncmapi::{
    types::{Album, Artist, Song},
    NcmApi,
};

use ncmapi::types::{LyricResp, SearchSongResp};

use super::Lyric;

pub struct NeteaseLyricProvider {}

pub struct NeteaseLyric {
    lyric: Option<String>,
    tlyric: Option<String>,
}

impl super::LyricProvider for NeteaseLyricProvider {
    type Id = usize;
    type LStore = NeteaseLyric;

    const NAME: &'static str = "网易云音乐";

    fn search_song(
        &self,
        album: &str,
        artists: &[&str],
        title: &str,
    ) -> Result<Vec<super::SongInfo<Self::Id>>> {
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
                    id: *id as _,
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

    fn query_lyric(&self, id: Self::Id) -> Result<NeteaseLyric> {
        let cookie_path = crate::CONFIG_HOME.with_borrow(|home| home.to_owned() + "ncm-cookie");
        let api = NcmApi::new(
            false,
            Duration::from_secs(60 * 60),
            Duration::from_secs(5 * 60),
            true,
            &cookie_path,
        );
        let query_result = smol::block_on(async { api.lyric(id).compat().await })?;

        let lyric_resp: LyricResp = query_result.deserialize()?;

        tracing::debug!("lyric query result: {lyric_resp:?}");

        Ok(NeteaseLyric {
            lyric: lyric_resp.lrc.map(|l| l.lyric),
            tlyric: lyric_resp.tlyric.map(|l| l.lyric),
        })
    }

    fn new() -> Result<Box<Self>> {
        Ok(Box::new(Self {}))
    }
}

impl super::LyricStore for NeteaseLyric {
    fn get_lyric(&self) -> Lyric<'_> {
        let lyric = self.lyric.as_deref();
        match_lyric(lyric)
    }

    fn get_translated_lyric(&self) -> Lyric<'_> {
        let lyric = self.tlyric.as_deref();
        match_lyric(lyric)
    }
}

fn match_lyric(lyric: Option<&str>) -> Lyric<'_> {
    match lyric {
        Some("") | None => super::Lyric::None,
        Some(lyric) => {
            if let Ok(parsed) = super::utils::lrc_iter(lyric.split("\n")) {
                Lyric::LineTimestamp(parsed)
            } else {
                Lyric::NoTimestamp
            }
        }
    }
}
