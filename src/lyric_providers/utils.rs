use lrc_nom::{parse_single, LrcParseError};
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
    let mut lrc_vec: Vec<_> = filtered_lines
        .enumerate()
        .filter_map(|(line_num, line)| parse_single(line, line_num).ok())
        .flatten()
        .filter_map(|lrc_item| match lrc_item {
            lrc_nom::LrcItem::Metadata(_) => None,
            lrc_nom::LrcItem::Lyric(lyric, timestamps) => {
                Some(timestamps.into_iter().map(|timestamp| LyricLine {
                    text: lyric.trim(),
                    start_time: Duration::from_millis(timestamp as _),
                }))
            }
        })
        .flatten()
        .collect();
    // handling malformed LRC timestamp by sorting them here
    //
    // DO NOT use sort_unstable_by_key, we should let lyrics
    // preserve original relative order, that extracting
    // translation will work.
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
    use super::lrclib::LRCLib;
    use super::netease::Netease;
    use super::qqmusic::QQMusic;
    let providers: [&'static dyn super::LyricProvider; 3] = [&Netease, &QQMusic, &LRCLib];
    providers
        .into_iter()
        .find(|p| p.unique_name() == provider_id)
}
