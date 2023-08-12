use anyhow::Result;
use qqmusic_rs::{
    lyric::{QueryLyric, QueryLyricResp},
    QQMusicApi,
};
use reqwest::blocking::Client;

use super::Lyric;

pub struct QQMusicLyricProvider {
    api: QQMusicApi,
}

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
        let url = self.api.query_lyric(songid);
        let resp: QueryLyricResp =
            serde_json::from_slice(client.get(url).send()?.bytes()?.as_ref())?;

        Ok(QQMusicLyric {
            lyric: Some(resp.data.lyric),
            tlyric: Some(resp.data.trans),
        })
    }
}

impl QQMusicLyricProvider {
    pub fn new(base_url: url::Url) -> Result<Self> {
        Ok(Self {
            api: QQMusicApi::new(base_url),
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
