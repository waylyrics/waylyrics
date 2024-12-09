use anyhow::Result;

use crate::lyric_providers::qqmusic::{QQMusic, QQMusicConfig};
use crate::lyric_providers::LyricProvider;

fn init_with_base(base: &str) -> Result<()> {
    let config = QQMusicConfig {
        api_base_url: base.into(),
        ..Default::default()
    };
    QQMusic.init(&serde_json::to_string(&config)?)
}

#[test]
fn test_qqmusic_base_url_init() {
    // empty URL
    assert!(init_with_base("").is_err());
    // ill-formed URL
    assert!(init_with_base("http//127.0.0.1:1000").is_err());
    // legal URL example
    assert!(init_with_base("https://127.0.0.1:1000").is_ok());
    // prevents to initialize again
    assert!(init_with_base("http://127.0.0.1:1000").is_err());
}
