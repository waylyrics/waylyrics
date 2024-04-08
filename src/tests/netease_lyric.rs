#[cfg(feature = "net-test")]
use anyhow::Result;

/// This test cannot be run from offline environment!
#[cfg(feature = "net-test")]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_netease_lyric() -> Result<()> {
    use std::time::Duration;

    use crate::lyric_providers::netease::Netease;
    use crate::lyric_providers::{LyricLineOwned, LyricOwned, LyricParse, LyricProvider};

    let provider = Netease;
    let lyric_store = provider.query_lyric("708965").await?;
    let lyric = provider.parse_lyric(&lyric_store);
    let LyricOwned::LineTimestamp(lyrics) = lyric else {
        panic!("cannot get lyric from netease");
    };

    assert_eq!(
        lyrics.last(),
        Some(&LyricLineOwned {
            text: "纯音乐，请欣赏".into(),
            start_time: Duration::from_secs(60 * 99),
        })
    );

    Ok(())
}
