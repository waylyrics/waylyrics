use std::str::FromStr;

use crate::lyric_providers::netease::Netease;
use crate::lyric_providers::qqmusic::QQMusic;

use crate::sync::interop::mpris::PLAYER;
use crate::sync::lyric::fetch::tricks::get_lrc_path;
use crate::sync::lyric::fetch::LyricHint;
use crate::sync::ENABLE_LOCAL_LYRIC;

pub fn hint_from_player() -> Option<LyricHint> {
    PLAYER.with_borrow(|player| {
        let player = player
            .as_ref()
            .expect("player not exists in lyric fetching");
        let player_name = player.identity();
        let player_bus_name = player.bus_name_player_name_part();
        let meta = player.get_metadata().ok()?;

        match (player_name, player_bus_name) {
            ("ElectronNCM" | "Qcm", _) | (_, "musicfox" | "NeteaseCloudMusicGtk4") => meta
                .get("mpris:trackid")
                .and_then(mpris::MetadataValue::as_str)
                .and_then(|s| s.split('/').last())
                .map(str::to_owned)
                .map(|song_id| LyricHint::SongId {
                    song_id,
                    provider: &Netease,
                }),
            ("feeluown", _) => meta
                .url()?
                .strip_prefix("fuo://netease/songs/")
                .map(str::to_owned)
                .map(|song_id| LyricHint::SongId {
                    song_id,
                    provider: &Netease,
                })
                .or_else(|| {
                    meta.url()?
                        .strip_prefix("fuo://qqmusic/songs/")
                        .map(str::to_owned)
                        .map(|song_id| LyricHint::SongId {
                            song_id,
                            provider: &QQMusic,
                        })
                }),
            ("YesPlayMusic", _) => meta
                .url()?
                .strip_prefix("/trackid/")
                .map(str::to_owned)
                .map(|song_id| LyricHint::SongId {
                    song_id,
                    provider: &Netease,
                }),
            _ => meta.url().and_then(|meta_url| match meta_url {
                _ if meta_url.starts_with("file://") => {
                    if !*ENABLE_LOCAL_LYRIC
                        .get()
                        .expect("enable-local-lyric was not set!")
                    {
                        return None;
                    }
                    let music_path = url::Url::from_str(meta_url)
                        .ok()
                        .and_then(|music_uri| music_uri.to_file_path().ok())?;
                    get_lrc_path(music_path)
                        .filter(|lyric_path| lyric_path.exists())
                        .map(LyricHint::LyricFile)
                }
                _ => None,
            }),
        }
    })
}
