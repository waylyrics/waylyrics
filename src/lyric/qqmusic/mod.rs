use anyhow::Result;
use qqmusic_rs::lyric::{QueryLyric, QueryLyricResp};
use reqwest::blocking::Client;

use crate::QQMUSIC_API_CLIENT;

use super::Lyric;

pub struct QQMusicLyricProvider;

pub struct QQMusicLyric {
    lyric: Option<String>,
    tlyric: Option<String>,
}

impl super::LyricProvider for QQMusicLyricProvider {
    type Id = u32;
    type LStore = QQMusicLyric;

    const NAME: &'static str = "QQ音乐";

    fn search_song(
        &self,
        album: &str,
        artists: &[&str],
        title: &str,
    ) -> Result<Vec<super::SongInfo<Self::Id>>> {
        let keyword = format!("{title} {album} {}", artists.join("/"));
        tracing::debug!("search keyword: {keyword}");

        todo!()
    }

    fn query_lyric(&self, id: Self::Id) -> Result<QQMusicLyric> {
        let client = Client::builder().user_agent("Waylyrics/0.1").build()?;
        let id = id.to_string();
        let songid = qqmusic_rs::SongId::Songid(&id);
        QQMUSIC_API_CLIENT.with_borrow(|api| {
            let Some(api) = api.as_ref() else {
                return Err(Error::ApiClientNotInit)?;
            };

            let url = api.query_lyric(songid);
            let resp: QueryLyricResp =
                serde_json::from_slice(client.get(url).send()?.bytes()?.as_ref())?;

            Ok(QQMusicLyric {
                lyric: Some(resp.data.lyric),
                tlyric: Some(resp.data.trans),
            })
        })
    }
}

impl super::LyricStore for QQMusicLyric {
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

#[derive(thiserror::Error, Debug)]
pub enum Error{
    #[error("please make sure you configured QQMusicApi base URL")]
    ApiClientNotInit,
}
