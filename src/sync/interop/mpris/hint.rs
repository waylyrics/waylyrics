use std::str::FromStr;

use mpris::{Metadata, Player};

use crate::lyric_providers::netease::Netease;
use crate::lyric_providers::qqmusic::QQMusic;

use crate::sync::interop::mpris::PLAYER;
use crate::sync::lyric::fetch::tricks::get_lrc_path;
use crate::sync::lyric::fetch::LyricHint;

pub fn hint_from_player() -> Option<LyricHint> {
    PLAYER.with_borrow(|player| {
        let player = player
            .as_ref()
            .expect("player not exists in lyric fetching");
        let player_name = player.identity();
        let player_bus_name = player
            .bus_name()
            .strip_prefix("org.mpris.MediaPlayer2.")
            .unwrap();

        match (player_name, player_bus_name) {
            ("ElectronNCM" | "Qcm", _)
            | (_, "com.gitee.gmg137.NeteaseCloudMusicGtk4" | "NeteaseCloudMusicGtk4") => {
                get_field_from_player(player, |meta| {
                    meta.get("mpris:trackid")
                        .and_then(mpris::MetadataValue::as_str)
                        .and_then(|s| s.split('/').last())
                })
                .map(|song_id| LyricHint::SongId {
                    song_id,
                    provider: Box::new(Netease) as _,
                })
            }
            ("feeluown", _) => get_field_from_player(player, |meta| {
                meta.url()?.strip_prefix("fuo://netease/songs/")
            })
            .map(|song_id| LyricHint::SongId {
                song_id,
                provider: Box::new(Netease) as _,
            })
            .or_else(|| {
                get_field_from_player(player, |meta| {
                    meta.url()?.strip_prefix("fuo://qqmusic/songs/")
                })
                .map(|song_id| LyricHint::SongId {
                    song_id,
                    provider: Box::new(QQMusic) as _,
                })
            }),
            ("YesPlayMusic", _) => {
                get_field_from_player(player, |meta| meta.url()?.strip_prefix("/trackid/")).map(
                    |song_id| LyricHint::SongId {
                        song_id,
                        provider: Box::new(Netease) as _,
                    },
                )
            }
            _ => {
                get_field_from_player(player, |meta| meta.url()).and_then(|meta_url| match meta_url
                {
                    _ if meta_url.starts_with("file://") => {
                        let music_path = url::Url::from_str(&meta_url)
                            .ok()
                            .and_then(|music_uri| music_uri.to_file_path().ok())?;
                        get_lrc_path(music_path)
                            .filter(|lyric_path| lyric_path.exists())
                            .map(LyricHint::LyricFile)
                    }
                    _ => None,
                })
            }
        }
    })
}

fn get_field_from_player(
    player: &Player,
    extract_field: impl for<'a> FnOnce(&'a Metadata) -> Option<&'a str>,
) -> Option<String> {
    player
        .get_metadata()
        .ok()
        .and_then(|metadata| extract_field(&metadata).map(|s| s.to_owned()))
}
