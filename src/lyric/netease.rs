use reqwest::blocking::Client;

pub struct NeteaseLyricProvider {
    client: Client,
}

pub struct NeteaseLyric {
    lyric: String,
}

impl super::LyricProvider<NeteaseLyric> for NeteaseLyricProvider {
    type Id = u64;
    const NAME: &'static str = "网易云音乐";

    fn search_song(&self, singer: &str, title: &str) -> Vec<super::SongInfo<Self::Id>> {
        todo!()
    }

    fn query_lyric(&self, id: Self::Id) -> NeteaseLyric {
        todo!()
    }
}

impl super::LyricStore for NeteaseLyric {
    fn get_lyric<'a>(&'a self) -> super::Lyric<'a> {
        super::Lyric::LineTimestamp(super::utils::lrc_iter(&self.lyric, "\n").unwrap())
    }
}
