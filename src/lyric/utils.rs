use lrc_nom::{parse, LrcParseError};
use std::time::Duration;

pub fn lrc_iter<'a>(lyric: &'a str, lf: &str) -> Result<Vec<(&'a str, Duration)>, LrcParseError> {
    let mut lrc_vec: Vec<_> = parse(lyric, lf)?
        .into_iter()
        .filter_map(|lrc_item| match lrc_item {
            lrc_nom::LrcItem::Metadata(_) => None,
            lrc_nom::LrcItem::Lyric(lyric, timestamp) => {
                Some((lyric.trim(), Duration::from_millis(timestamp as u64)))
            }
        })
        .collect();
    // handling malformed LRC timestamp by sorting them here
    lrc_vec.sort_by(|(_, a), (_, b)| a.cmp(b));
    Ok(lrc_vec)
}
