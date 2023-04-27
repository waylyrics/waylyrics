use std::{thread, time::Duration};

use ncmapi::NcmApi;
use tokio::runtime::Handle;

use crate::lyric::netease::ncmtype::SearchResp;

use self::ncmtype::{Ar, Song};

mod ncmtype;

pub struct NeteaseLyricProvider {}

pub struct NeteaseLyric {
    lyric: String,
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
        .unwrap()
        .unwrap();
        let resp: SearchResp = search_result.deserialize()?;
        Ok(resp
            .result
            .songs
            .iter()
            .map(|Song { name, id, ar, alia }| super::SongInfo {
                id: *id as _,
                title: name.into(),
                singer: ar
                    .iter()
                    .map(|Ar { name }| name)
                    .fold(String::new(), |mut s, op| {
                        if !s.is_empty() {
                            s.push(',')
                        }
                        s += op;
                        s
                    }),
            })
            .collect())
    }

    fn query_lyric(
        &self,
        handle: Handle,
        id: Self::Id,
        translate: bool,
    ) -> Self::LResult<NeteaseLyric> {
        todo!()
    }
}

impl super::LyricStore for NeteaseLyric {
    fn get_lyric<'a>(&'a self) -> super::Lyric<'a> {
        super::Lyric::LineTimestamp(super::utils::lrc_iter(&self.lyric, "\n").unwrap())
    }
}
