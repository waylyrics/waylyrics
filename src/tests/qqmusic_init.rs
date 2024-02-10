use crate::lyric_providers::qqmusic::QQMusic;
use crate::lyric_providers::LyricProvider;

#[test]
fn test_qqmusic_base_url_init() {
    // empty URL
    assert!(QQMusic.init("").is_err());
    // ill-formed URL
    assert!(QQMusic.init("http//127.0.0.1:1000").is_err());
    // legal URL example
    assert!(QQMusic.init("https://127.0.0.1:1000").is_ok());
    // prevents to initialize again
    assert!(QQMusic.init("http://127.0.0.1:1000").is_err());
}
