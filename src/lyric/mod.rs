pub mod utils;

pub mod netease;

use std::time::Duration;

pub enum Lyric<'a> {
    None,
    NoTimestamp,
    LineTimestamp(Vec<(&'a str, Duration)>),
    WordTimestamp(Vec<(Option<&'a str>, Duration)>),
}

pub struct SongInfo<Id> {
    pub id: Id,
    pub title: String,
    pub singer: String,
}

pub trait LyricProvider<L> where L: LyricStore {
    type Id;
    const NAME: &'static str;
    fn query_lyric(&self, id: Self::Id) -> L;
    fn search_song(&self, singer: &str, title: &str) -> Vec<SongInfo<Self::Id>>;
}

pub trait LyricStore {
    fn get_lyric<'a>(&'a self) -> Lyric<'a>;
}
