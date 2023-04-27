pub mod utils;

pub mod netease;

use std::time::Duration;

use tokio::runtime::Handle;

pub enum Lyric<'a> {
    None,
    NoTimestamp,
    LineTimestamp(Vec<(&'a str, Duration)>),
    WordTimestamp(Vec<(Option<&'a str>, Duration)>),
}

#[derive(Debug)]
pub struct SongInfo<Id> {
    pub id: Id,
    pub title: String,
    pub singer: String,
}

pub trait LyricProvider<L>
where
    L: LyricStore,
{
    type Id;
    type LResult<T>;

    const NAME: &'static str;
    fn query_lyric(&self, handle: Handle, id: Self::Id, translate: bool) -> Self::LResult<L>;
    fn search_song(
        &self,
        handle: Handle,
        singer: &str,
        title: &str,
    ) -> Self::LResult<Vec<SongInfo<Self::Id>>>;
}

pub trait LyricStore {
    fn get_lyric<'a>(&'a self) -> Lyric<'a>;
}
