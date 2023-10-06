use crate::lyric_providers::netease::NeteaseLyricProvider;
use crate::lyric_providers::qqmusic::QQMusicLyricProvider;

use crate::sync::PLAYER;

use crate::app;
use crate::lyric_providers::{LyricParse, LyricProvider};
use anyhow::Result;
use mpris::{Metadata, Player};

use crate::sync::lyric::fetch::set_lyric;

pub fn get_accurate_lyric(
    title: &str,
    artists: &str,
    window: &app::Window,
) -> Option<Result<(), anyhow::Error>> {
    PLAYER.with_borrow(|player| {
        let player = player
            .as_ref()
            .expect("player not exists in lyric fetching");
        let player_name = player.identity();
        let handle = tokio::runtime::Handle::current();
        match player_name {
            "mpv" => {
                tracing::warn!("local lyric files are still unsupported");
                None
            }
            "ElectronNCM" | "Qcm" => get_song_id_from_player(player, |meta| {
                meta.get("mpris:trackid")
                    .and_then(mpris::MetadataValue::as_str)
                    .and_then(|s| s.split('/').last())
            })
            .map(|song_id| {
                let provider = NeteaseLyricProvider;
                let lyric = handle.block_on(provider.query_lyric(&song_id))?;
                let olyric = provider.get_lyric(&lyric);
                let tlyric = provider.get_translated_lyric(&lyric);
                set_lyric(olyric, tlyric, title, artists, window)
            }),
            "feeluown" => get_song_id_from_player(player, |meta| {
                meta.url()?.strip_prefix("fuo://netease/songs/")
            })
            .map(|song_id| {
                let provider = NeteaseLyricProvider;
                let lyric = handle.block_on(provider.query_lyric(&song_id))?;
                let olyric = provider.get_lyric(&lyric);
                let tlyric = provider.get_translated_lyric(&lyric);
                set_lyric(olyric, tlyric, title, artists, window)
            })
            .or_else(|| {
                get_song_id_from_player(player, |meta| {
                    meta.url()?.strip_prefix("fuo://qqmusic/songs/")
                })
                .map(|song_id| {
                    let provider = QQMusicLyricProvider;
                    let lyric = handle.block_on(provider.query_lyric(&song_id))?;
                    let olyric = provider.get_lyric(&lyric);
                    let tlyric = provider.get_translated_lyric(&lyric);
                    set_lyric(olyric, tlyric, title, artists, window)
                })
            }),
            "YesPlayMusic" => {
                get_song_id_from_player(player, |meta| meta.url()?.strip_prefix("/trackid/")).map(
                    |song_id| {
                        let provider = NeteaseLyricProvider;
                        let lyric = handle.block_on(provider.query_lyric(&song_id))?;
                        let olyric = provider.get_lyric(&lyric);
                        let tlyric = provider.get_translated_lyric(&lyric);
                        set_lyric(olyric, tlyric, title, artists, window)
                    },
                )
            }

            _ => None,
        }
    })
}

fn get_song_id_from_player(
    player: &Player,
    extract_field: impl for<'a> FnOnce(&'a Metadata) -> Option<&'a str>,
) -> Option<String> {
    player
        .get_metadata()
        .ok()
        .and_then(|metadata| extract_field(&metadata).map(|s| s.to_owned()))
}
