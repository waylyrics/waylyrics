#[cfg(not(feature = "offline-test"))]
use anyhow::Result;

/// This test cannot be run from offline environment!
#[cfg(not(feature = "offline-test"))]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn get_lrclib_lyric() -> Result<()> {
    use crate::lyric_providers::lrclib::LRCLib;
    use crate::lyric_providers::{LyricOwned, LyricParse, LyricProvider};

    let provider = LRCLib;
    let lyric_store = provider.query_lyric("1").await?;
    let lyric = provider.parse_lyric(&lyric_store);
    let LyricOwned::LineTimestamp(_) = lyric else {
        anyhow::bail!("cannot get lyric from lrclib");
    };

    Ok(())
}

#[cfg(not(feature = "offline-test"))]
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn search_lrclib_lyric() -> Result<()> {
    use crate::lyric_providers::lrclib::LRCLib;
    use crate::lyric_providers::LyricProvider;

    let provider = LRCLib;
    let _lyric_store = provider.search_song("周杰伦").await?;

    Ok(())
}
