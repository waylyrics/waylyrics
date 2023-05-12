use std::path::PathBuf;

use crate::lyric::LyricOwned;

use super::{LYRIC, LYRIC_CURRENT, LYRIC_TRANSLATION_CURRENT, LYRIC_OFFSET_MILLISEC};

pub fn md5_cache_dir(digest: md5::Digest) -> PathBuf {
    let mut cache_path = PathBuf::new();
    for i in 0..3 {
        cache_path.push(format!("{:02x}", digest[i]));
    }
    cache_path
}

pub fn clear_lyric() {
    LYRIC.set((LyricOwned::None, LyricOwned::None));
    LYRIC_CURRENT.set(None);
    LYRIC_TRANSLATION_CURRENT.set(None);
    LYRIC_OFFSET_MILLISEC.set(0);
}