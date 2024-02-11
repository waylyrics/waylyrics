pub mod utils;
use anyhow::Result;

pub mod netease;
pub mod qqmusic;

use std::{fmt::Debug, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Lyric<'a> {
    None,
    NoTimestamp,
    LineTimestamp(Vec<LyricLine<'a>>),
}

#[derive(Debug)]
pub struct LyricLine<'a> {
    pub text: &'a str,
    pub start_time: Duration,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
#[serde(tag = "type", content = "content")]
pub enum LyricOwned {
    #[default]
    None,
    NoTimestamp,
    LineTimestamp(Vec<LyricLineOwned>),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct LyricLineOwned {
    pub text: String,
    pub start_time: Duration,
}

#[derive(Debug)]
pub struct SongInfo {
    pub id: String,
    pub title: String,
    pub singer: String,
    pub album: Option<String>,
    pub length: Duration,
}

#[async_trait::async_trait]
pub trait LyricProvider: LyricParse + Send + Sync {
    async fn query_lyric(&self, id: &str) -> Result<LyricStore>;
    async fn search_song_detailed(
        &self,
        album: &str,
        artists: &[&str],
        title: &str,
    ) -> Result<Vec<SongInfo>>;
    async fn search_song(&self, keyword: &str) -> Result<Vec<SongInfo>>;
    fn unique_name(&self) -> &'static str;
    fn init(self, config: &str) -> Result<()>;
    fn is_likely_songid(&self, s: &str) -> bool;
}

pub trait LyricParse {
    fn parse_lyric(&self, store: &LyricStore) -> LyricOwned;
    fn parse_translated_lyric(&self, store: &LyricStore) -> LyricOwned;
}

pub fn provider_fmt(
    p: impl AsRef<dyn LyricProvider>,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    f.write_str(p.as_ref().unique_name())
}

impl<'a> Lyric<'a> {
    pub fn into_owned(self) -> LyricOwned {
        match self {
            Lyric::None => LyricOwned::None,
            Lyric::NoTimestamp => LyricOwned::NoTimestamp,
            Lyric::LineTimestamp(lyrics) => LyricOwned::LineTimestamp(
                lyrics
                    .into_iter()
                    .map(LyricLine::<'_>::into_owned)
                    .collect(),
            ),
        }
    }
}

impl<'a> LyricLine<'a> {
    pub fn into_owned(Self { text, start_time }: Self) -> LyricLineOwned {
        LyricLineOwned {
            text: text.into(),
            start_time,
        }
    }
}

pub struct LyricStore {
    lyric: Option<String>,
    tlyric: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no search result!")]
    NoResult,
    #[error("no lyrics for such song")]
    NoLyric,
}

pub fn default_search_query(album: &str, artists: &[&str], title: &str) -> String {
    format!("{title} {album} {}", artists.join("/"))
}
