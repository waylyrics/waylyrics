use crate::lyric_providers::netease::Netease;
use crate::lyric_providers::qqmusic::QQMusic;

use crate::sync::PLAYER;

use crate::lyric_providers::LyricProvider;
use crate::{app, glib_spawn};
use anyhow::Result;
use gtk::glib::clone::Downgrade;
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
        let player_bus_name = player.bus_name_player_name_part();
        let Some((song_id, provider)): Option<(String, Box<dyn LyricProvider>)> =
            (match (player_name, player_bus_name) {
                ("mpv", _) => {
                    tracing::warn!("local lyric files are still unsupported");
                    None
                }
                ("ElectronNCM" | "Qcm", _) | (_, "NeteaseCloudMusicGtk") => {
                    get_song_id_from_player(player, |meta| {
                        meta.get("mpris:trackid")
                            .and_then(mpris::MetadataValue::as_str)
                            .and_then(|s| s.split('/').last())
                    })
                    .map(|song_id| (song_id, Box::new(Netease) as _))
                }
                ("feeluown", _) => get_song_id_from_player(player, |meta| {
                    meta.url()?.strip_prefix("fuo://netease/songs/")
                })
                .map(|song_id| (song_id, Box::new(Netease) as _))
                .or_else(|| {
                    get_song_id_from_player(player, |meta| {
                        meta.url()?.strip_prefix("fuo://qqmusic/songs/")
                    })
                    .map(|song_id| (song_id, Box::new(QQMusic) as _))
                }),
                ("YesPlayMusic", _) => {
                    get_song_id_from_player(player, |meta| meta.url()?.strip_prefix("/trackid/"))
                        .map(|song_id| (song_id, Box::new(Netease) as _))
                }

                _ => None,
            })
        else {
            return None;
        };

        let title = title.to_owned();
        let artists = artists.to_owned();
        let window = window.downgrade();
        tracing::debug!("spawned query from get_accurate_lyric");
        glib_spawn!(async move {
            let Ok(lyric) = provider.query_lyric(&song_id).await else {
                return;
            };
            let olyric = provider.get_lyric(&lyric);
            let tlyric = provider.get_translated_lyric(&lyric);
            let Some(window) = window.upgrade() else {
                return;
            };

            set_lyric(olyric, tlyric, &title, &artists, &window);
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
