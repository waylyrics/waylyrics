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

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum LyricOwned {
    None,
    NoTimestamp,
    LineTimestamp(Vec<LyricLineOwned>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LyricLineOwned {
    pub text: String,
    pub start_time: Duration,
}

#[derive(Debug)]
pub struct SongInfo<Id> {
    pub id: Id,
    pub title: String,
    pub singer: String,
    pub length: Duration,
}

pub trait LyricProvider<L>
where
    L: LyricStore,
{
    type Id;
    type LResult<T>;

    const NAME: &'static str;
    fn query_lyric(&self, handle: &Handle, id: Self::Id) -> Self::LResult<L>;
    fn search_song(
        &self,
        handle: &Handle,
        singer: &str,
        title: &str,
    ) -> Self::LResult<Vec<SongInfo<Self::Id>>>;
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
