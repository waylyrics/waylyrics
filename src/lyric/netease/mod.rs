use std::{thread, time::Duration};

use ncmapi::{
    types::{Artist, Song},
    NcmApi,
};
use tokio::runtime::Handle;

use ncmapi::types::{LyricResp, SearchSongResp};

use super::Lyric;

pub struct NeteaseLyricProvider {}

pub struct NeteaseLyric {
    lyric: Option<String>,
    tlyric: Option<String>,
}

impl NeteaseLyricProvider {
    pub fn new() -> Result<Self, std::io::Error> {
        Ok(Self {})
    }
}

impl super::LyricProvider<NeteaseLyric> for NeteaseLyricProvider {
    type Id = usize;
    type LResult<T> = Result<T, Box<dyn std::error::Error>>;

    const NAME: &'static str = "网易云音乐";

    fn search_song(
        &self,
        handle: Handle,
        singer: &str,
        title: &str,
    ) -> Self::LResult<Vec<super::SongInfo<Self::Id>>> {
        let keyword = format!("{title} {singer}");
        let search_result = thread::spawn(move || {
            let api = NcmApi::new(
                true,
                Duration::from_secs(60 * 60),
                Duration::from_secs(5 * 60),
                true,
                "ncmcookie",
            );
            handle.block_on(async { api.search(&keyword, None).await })
        })
        .join()
        .unwrap()?;
        let resp: SearchSongResp = search_result.deserialize()?;
        Ok(resp
            .result
            .ok_or("no search result")?
            .songs
            .iter()
            .map(
                |Song {
                     name, id, artists, ..
                 }| super::SongInfo {
                    id: *id as _,
                    title: name.into(),
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
                },
            )
            .collect())
    }

    fn query_lyric(&self, handle: Handle, id: Self::Id) -> Self::LResult<NeteaseLyric> {
        let query_result = thread::spawn(move || {
            let api = NcmApi::new(
                true,
                Duration::from_secs(60 * 60),
                Duration::from_secs(5 * 60),
                true,
                "ncmcookie",
            );
            handle.block_on(async { api.lyric(id).await })
        })
        .join()
        .unwrap()?;

        let lyric_resp: LyricResp = query_result.deserialize()?;

        Ok(NeteaseLyric {
            lyric: lyric_resp.lrc.and_then(|l| Some(l.lyric)),
            tlyric: lyric_resp.tlyric.and_then(|l| Some(l.lyric)),
        })
    }
}

impl super::LyricStore for NeteaseLyric {
    fn get_lyric<'a>(&'a self) -> Lyric<'a> {
        let lyric = self.lyric.as_ref().map(|s| s.as_str());
        match_lyric(lyric)
    }

    fn get_translated_lyric<'a>(&'a self) -> Lyric<'a> {
        let lyric = self.tlyric.as_ref().map(|s| s.as_str());
        match_lyric(lyric)
    }
}

fn match_lyric<'a>(lyric: Option<&'a str>) -> Lyric<'a> {
    match lyric {
        Some("") | None => super::Lyric::None,
        Some(lyric) => super::Lyric::LineTimestamp(super::utils::lrc_iter(lyric, "\n").unwrap()),
    }
}
