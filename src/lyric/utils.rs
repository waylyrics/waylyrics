use lrc_nom::{parse, LrcParseError};
use std::time::Duration;

pub fn lrc_iter<'a>(lyric: &'a str, lf: &str) -> Result<Vec<(&'a str, Duration)>, LrcParseError> {
    Ok(parse(lyric, lf)?
        .into_iter()
        .filter_map(|lrc_item| match lrc_item {
            lrc_nom::LrcItem::Metadata(_) => None,
            lrc_nom::LrcItem::Lyric(lyric, timestamp) => {
                Some((lyric.into(), Duration::from_millis(timestamp as u64)))
            }
        })
        .collect())
}
