pub mod utils;

pub mod netease;

use std::time::Duration;

use tokio::runtime::Handle;

#[derive(Debug)]
pub enum Lyric<'a> {
    None,
    NoTimestamp,
    LineTimestamp(Vec<(&'a str, Duration)>),
}

#[derive(Debug)]
pub enum LyricOwned {
    None,
    NoTimestamp,
    LineTimestamp(Vec<(String, Duration)>),
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
    fn get_lyric<'a>(&'a self) -> Lyric<'a>;
    fn get_translated_lyric<'a>(&'a self) -> Lyric<'a>;
}

impl<'a> Lyric<'a> {
    pub fn into_owned(self) -> LyricOwned {
        match self {
            Lyric::None => LyricOwned::None,
            Lyric::NoTimestamp => LyricOwned::NoTimestamp,
            Lyric::LineTimestamp(line) => LyricOwned::LineTimestamp(
                line.into_iter()
                    .map(|(text, off)| (text.to_owned(), off))
                    .collect(),
            ),
        }
    }
}
