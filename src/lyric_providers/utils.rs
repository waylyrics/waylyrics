use lrc_nom::{parse, LrcParseError};
use std::{sync::Arc, time::Duration};

use super::{LyricLine, LyricLineOwned, LyricProvider};

pub fn lrc_iter<'a>(
    lyric_lines: impl Iterator<Item = &'a str>,
) -> Result<Vec<LyricLine<'a>>, LrcParseError> {
    let mut lrc_vec: Vec<_> = parse(lyric_lines)?
        .into_iter()
        .filter_map(|lrc_item| match lrc_item {
            lrc_nom::LrcItem::Metadata(_) => None,
            lrc_nom::LrcItem::Lyric(lyric, timestamp) => Some(LyricLine {
                text: lyric,
                start_time: Duration::from_millis(timestamp as _),
            }),
        })
        .collect();
    // handling malformed LRC timestamp by sorting them here
    lrc_vec.sort_by(|left, right| left.start_time.cmp(&right.start_time));
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

pub fn get_provider(provider_id: &str) -> Option<Arc<dyn LyricProvider>> {
    use super::netease::Netease;
    use super::qqmusic::QQMusic;
    let providers: [Arc<dyn super::LyricProvider>; 2] = [Arc::new(Netease), Arc::new(QQMusic)];
    providers
        .into_iter()
        .find(|p| p.unique_name() == provider_id)
}
