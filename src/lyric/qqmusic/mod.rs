use anyhow::Result;

use super::Lyric;

pub struct QQMusicLyricProvider {}

pub struct QQMusicLyric {
    lyric: Option<String>,
    tlyric: Option<String>,
}

impl super::LyricProvider for QQMusicLyricProvider {
    type Id = u32;
    type LStore = QQMusicLyric;

    const NAME: &'static str = "QQ音乐";

    fn search_song(
        &self,
        album: &str,
        artists: &[&str],
        title: &str,
    ) -> Result<Vec<super::SongInfo<Self::Id>>> {
        let keyword = format!("{title} {album} {}", artists.join("/"));
        tracing::debug!("search keyword: {keyword}");

        todo!()
    }

    fn query_lyric(&self, id: Self::Id) -> Result<QQMusicLyric> {
        todo!()
    }

    fn new() -> Result<Box<Self>> {
        Ok(Box::new(Self {}))
    }
}

impl super::LyricStore for QQMusicLyric {
    fn get_lyric(&self) -> Lyric<'_> {
        let lyric = self.lyric.as_deref();
        match_lyric(lyric)
    }

    fn get_translated_lyric(&self) -> Lyric<'_> {
        let lyric = self.tlyric.as_deref();
        match_lyric(lyric)
    }
}

fn match_lyric(lyric: Option<&str>) -> Lyric<'_> {
    match lyric {
        Some("") | None => super::Lyric::None,
        Some(lyric) => {
            if let Ok(parsed) = super::utils::lrc_iter(lyric.split("\n")) {
                Lyric::LineTimestamp(parsed)
            } else {
                Lyric::NoTimestamp
            }
        }
    }
}
