use std::ffi::OsStr;
use std::path::PathBuf;

use mpris::{Metadata, Player};

use crate::lyric_providers::netease::Netease;
use crate::lyric_providers::qqmusic::QQMusic;

use crate::sync::interop::mpris::PLAYER;
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
            _ => get_field_from_player(player, |meta| meta.url()?.strip_prefix("file://"))
                .and_then(|music_path| {
                    let music_path = PathBuf::from(music_path);

                    let lyric_path = get_lyric_path(music_path);

                    lyric_path.map(|lyric_path| LyricHint::File(lyric_path))
                }),
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

fn get_lyric_path(music_path: PathBuf) -> Option<PathBuf> {
    if !music_path.is_file() {
        // invalid music file path
        return None;
    }

    let file_name = music_path.iter().last().unwrap().as_encoded_bytes();
    file_name
        .iter()
        .enumerate()
        .rev()
        .find(|&(_, ch)| ch == &b'.')
        .map(|(last_dot_pos, _)| {
            let mut lrc_file_name = file_name.split_at(last_dot_pos + 1).0.to_vec();
            lrc_file_name.extend_from_slice("lrc".as_bytes());
            let lrc_file_name = unsafe { OsStr::from_encoded_bytes_unchecked(&lrc_file_name) };

            music_path
                .parent()
                .map(|music_dir| music_dir.join(lrc_file_name))
        })
        .flatten()
}
