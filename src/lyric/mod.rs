use std::time::Duration;

pub enum Lyric<'a, L: Iterator<Item = (&'a str, Duration)>> {
    None,
    NoTimestamp,
    LineTimestamp(L),
}

pub trait LyricProvider<'a, L>
where
    L: Iterator<Item = (&'a str, Duration)>,
{
    type Id;
    fn get_lyric(&self, id: Self::Id) -> Lyric<'a, L>;
}
