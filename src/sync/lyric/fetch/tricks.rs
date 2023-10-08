use crate::lyric_providers::netease::Netease;
use crate::lyric_providers::qqmusic::QQMusic;

use crate::sync::PLAYER;

use crate::app;
use crate::lyric_providers::LyricProvider;
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
        let Some((song_id, provider)): Option<(String, Box<dyn LyricProvider>)> = (match player_name
        {
            "mpv" => {
                tracing::warn!("local lyric files are still unsupported");
                None
            }
            "ElectronNCM" | "Qcm" => get_song_id_from_player(player, |meta| {
                meta.get("mpris:trackid")
                    .and_then(mpris::MetadataValue::as_str)
                    .and_then(|s| s.split('/').last())
            })
            .map(|song_id| (song_id, Box::new(Netease) as _)),
            "feeluown" => get_song_id_from_player(player, |meta| {
                meta.url()?.strip_prefix("fuo://netease/songs/")
            })
            .map(|song_id| (song_id, Box::new(Netease) as _))
            .or_else(|| {
                get_song_id_from_player(player, |meta| {
                    meta.url()?.strip_prefix("fuo://qqmusic/songs/")
                })
                .map(|song_id| (song_id, Box::new(QQMusic) as _))
            }),
            "YesPlayMusic" => {
                get_song_id_from_player(player, |meta| meta.url()?.strip_prefix("/trackid/"))
                    .map(|song_id| (song_id, Box::new(Netease) as _))
            }

            _ => None,
        }) else {
            return None;
        };

        gidle_future::spawn(async move {
            let Ok(lyric) = provider.query_lyric(&song_id).await else {
                return;
            };
            let olyric = provider.get_lyric(&lyric);
            let tlyric = provider.get_translated_lyric(&lyric);
            set_lyric(olyric, tlyric, title, artists, window);
        });

        Some(Ok(()))
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
