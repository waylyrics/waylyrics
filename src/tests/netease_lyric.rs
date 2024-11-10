#[cfg(not(feature = "offline-test"))]
use anyhow::Result;

/// This test cannot be run from offline environment!
#[cfg(not(feature = "offline-test"))]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_netease_lyric() -> Result<()> {
    use crate::lyric_providers::netease::Netease;
    use crate::lyric_providers::{LyricOwned, LyricParse, LyricProvider};

    let provider = Netease;
    let lyric_store = provider.query_lyric("708965").await?;
    let lyric = provider.parse_lyric(&lyric_store);
    let LyricOwned::LineTimestamp(lyrics) = lyric else {
        panic!("cannot get lyric from netease");
    };

    assert_eq!(lyrics.last().unwrap().text, "纯音乐，请欣赏".to_owned(),);

    Ok(())
}
