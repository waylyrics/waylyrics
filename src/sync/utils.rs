use std::time::Duration;

use crate::lyric::{LyricOwned, SongInfo};

use super::{
    LENGTH_TOLERATION_MILLISEC, LYRIC, LYRIC_CURRENT, LYRIC_OFFSET_MILLISEC,
    LYRIC_TRANSLATION_CURRENT,
};

pub fn clear_lyric() {
    LYRIC.set((LyricOwned::None, LyricOwned::None));
    LYRIC_CURRENT.set(None);
    LYRIC_TRANSLATION_CURRENT.set(None);
    LYRIC_OFFSET_MILLISEC.set(0);
}

pub fn match_likely_lyric<'a, Id>(
    album_title: Option<(&str, &str)>,
    length: Option<Duration>,
    search_result: &'a [SongInfo<Id>],
) -> Option<&'a Id> {
    length
        .and_then(|leng| {
            search_result.iter().find(|SongInfo { length, .. }| {
                length.as_millis().abs_diff(leng.as_millis())
                    <= LENGTH_TOLERATION_MILLISEC.with_borrow(|toleration| *toleration as _)
            })
        })
        .or_else(|| {
            album_title.and_then(|(_album, _title)| {
                search_result.iter().find(|SongInfo { title, album, .. }| {
                    title == _title && album.as_ref().is_some_and(|album| album == _album)
                })
            })
        })
        .or(search_result.get(0))
        .map(|song| &song.id)
}
