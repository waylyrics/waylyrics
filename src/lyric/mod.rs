pub mod utils;

pub mod netease;

use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::runtime::Handle;

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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "type", content = "content")]
pub enum LyricOwned {
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
pub struct SongInfo<Id> {
    pub id: Id,
    pub title: String,
    pub singer: String,
    pub album: Option<String>,
    pub length: Duration,
}

pub trait LyricProvider {
    type Id;
    type LStore: LyricStore;

    const NAME: &'static str;
    fn new() -> Result<Box<Self>, Box<dyn std::error::Error>>;
    fn query_lyric(
        &self,
        handle: &Handle,
        id: Self::Id,
    ) -> Result<Self::LStore, Box<dyn std::error::Error>>;
    fn search_song(
        &self,
        handle: &Handle,
        album: &str,
        artists: &[&str],
        title: &str,
    ) -> Result<Vec<SongInfo<Self::Id>>, Box<dyn std::error::Error>>;
}

pub trait LyricStore {
    fn get_lyric(&self) -> Lyric<'_>;
    fn get_translated_lyric(&self) -> Lyric<'_>;
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
