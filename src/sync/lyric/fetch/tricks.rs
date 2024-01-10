use crate::lyric_providers::LyricProvider;
use crate::sync::interop::hint_from_player;
use crate::sync::lyric::fetch::set_lyric;
use crate::{app, glib_spawn};
use anyhow::Result;
use gtk::glib::clone::Downgrade;

pub enum LyricHint {
    SongId {
        song_id: String,
        provider: Box<dyn LyricProvider>,
    },
}

pub fn get_lyric_hint_from_player(
    title: &str,
    artists: &str,
    window: &app::Window,
) -> Option<Result<(), anyhow::Error>> {
    let hint_from_player: Option<LyricHint> = hint_from_player();
    match hint_from_player {
        Some(LyricHint::SongId { song_id, provider }) => {
            let title = title.to_owned();
            let artists = artists.to_owned();
            let window = window.downgrade();
            crate::log::debug!("spawned query from get_accurate_lyric");
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

            return Some(Ok(()));
        }

        _ => None,
    }
}
