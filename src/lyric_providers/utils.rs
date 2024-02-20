use lrc_nom::{parse, LrcParseError};
use std::time::Duration;

use super::{LyricLine, LyricLineOwned, LyricProvider};

/// parses lrc tags in each line
/// ignores lines not started with '['
/// returned `Vec` is sorted by it's Duration
/// In the case multiple lyrics comes with the same start_time,
/// they are not reordered between them
pub fn lrc_iter<'a>(
    lyric_lines: impl Iterator<Item = &'a str>,
) -> Result<Vec<LyricLine<'a>>, LrcParseError> {
    let filtered_lines = lyric_lines.filter(|l| l.starts_with('['));
    let mut lrc_vec: Vec<_> = parse(filtered_lines)?
        .into_iter()
        .filter_map(|lrc_item| match lrc_item {
            lrc_nom::LrcItem::Metadata(_) => None,
            lrc_nom::LrcItem::Lyric(lyric, timestamp) => Some(LyricLine {
                text: lyric.trim(),
                start_time: Duration::from_millis(timestamp as _),
            }),
        })
        .collect();
    // handling malformed LRC timestamp by sorting them here
    lrc_vec.sort_by_key(|line| line.start_time);

    Ok(lrc_vec)
}

pub fn find_next_lyric<'a>(
    elapsed: &Duration,
    lyric: &'a [LyricLineOwned],
) -> Option<&'a LyricLineOwned> {
    lyric
        .iter()
        .take_while(
            |LyricLineOwned {
                 start_time: off, ..
             }| off <= elapsed,
        )
        .last()
}

pub fn get_provider(provider_id: &str) -> Option<&'static dyn LyricProvider> {
    use super::netease::Netease;
    use super::qqmusic::QQMusic;
    let providers: [&'static dyn super::LyricProvider; 2] = [&Netease, &QQMusic];
    providers
        .into_iter()
        .find(|p| p.unique_name() == provider_id)
}
