use crate::lyric_providers::netease::Netease;
use crate::lyric_providers::qqmusic::QQMusic;
use crate::lyric_providers::LyricProvider;

#[test]
fn is_qqmusic_songid() {
    for songid in ["004Knor205SbZb", "002jGrGi0OXxDJ", "1145141919"] {
        assert!(QQMusic.is_likely_songid(songid));
    }
    for invalid_songid in ["04Knor205SbZb", "102jGrGi0OXxDJ", "1145.141919", ""] {
        assert!(!QQMusic.is_likely_songid(invalid_songid));
    }
}

#[test]
fn is_netease_songid() {
    for songid in ["1145141919"] {
        assert!(Netease.is_likely_songid(songid));
    }
    for invalid_songid in ["11451.41919", ""] {
        assert!(!Netease.is_likely_songid(invalid_songid));
    }
}
