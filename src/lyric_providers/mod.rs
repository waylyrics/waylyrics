pub mod utils;
use anyhow::Result;

pub mod netease;
pub mod qqmusic;

use std::time::Duration;

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

pub trait LyricProvider: LyricParse {
    fn query_lyric(&self, id: &str) -> Result<LyricStore>;
    fn search_song(&self, album: &str, artists: &[&str], title: &str) -> Result<Vec<SongInfo>>;
    fn provider_unique_name(&self) -> &'static str;
}

pub trait LyricParse {
    fn get_lyric(&self, store: &LyricStore) -> LyricOwned;
    fn get_translated_lyric(&self, store: &LyricStore) -> LyricOwned;
}

impl<'a> Lyric<'a> {
    pub fn into_owned(self) -> LyricOwned {
        match self {
            Lyric::None => LyricOwned::None,
            Lyric::NoTimestamp => LyricOwned::NoTimestamp,
            Lyric::LineTimestamp(line) => LyricOwned::LineTimestamp(
                line.into_iter()
                    .map(
                        |LyricLine {
                             text,
                             start_time: time,
                         }| LyricLineOwned {
                            text: text.into(),
                            start_time: time,
                        },
                    )
                    .collect(),
            ),
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
