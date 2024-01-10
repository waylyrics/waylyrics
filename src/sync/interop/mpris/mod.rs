use std::{cell::RefCell, sync::OnceLock};

pub mod sync_task;
pub use sync_task::register_sync_task;

pub mod hint;
pub use hint::hint_from_player;

use anyhow::Error;
use gtk::subclass::prelude::*;
use mpris::{Metadata, Player, PlayerFinder};
use tokio::sync::Mutex;

use crate::log::error;

use crate::{
    app,
    sync::{
        lyric::{cache, fetch},
        TrackMeta,
    },
};

use crate::sync::interop::PlayerStatus;

use super::PlayerId;

pub fn clean_player() {
    PLAYER.take();
}

pub async fn update_lyric(
    track_meta: &TrackMeta,
    window: &app::Window,
    ignore_cache: bool,
) -> Result<(), Error> {
    static UPDATE_LYRIC_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let lock = UPDATE_LYRIC_LOCK.get_or_init(|| Mutex::new(()));
    let Ok(_gaurd) = lock.try_lock() else {
        return Err(anyhow::anyhow!("update_lyric already in queue"));
    };

    crate::sync::utils::clean_lyric(window);

    if window.imp().cache_lyrics.get() {
        cache::fetch_lyric_cached(track_meta, ignore_cache, window).await?
    } else {
        fetch::fetch_lyric(track_meta, window).await?
    };

    drop(_gaurd);
    Ok(())
}

pub fn connect_player_with_id(player_id: impl AsRef<str>) {
    let player_id = player_id.as_ref();

    PLAYER_FINDER.with_borrow(|player_finder| {
        if let Ok(player) = player_finder.find_by_name(player_id) {
            PLAYER.set(Some(player));
        } else {
            error!("cannot connect to: {player_id}");
        }
    });
}

pub fn list_player_names() -> Vec<PlayerId> {
    find_players()
        .iter()
        .map(|p| PlayerId {
            player_name: p.identity().to_owned(),
            inner_id: p.bus_name().to_owned(),
        })
        .collect()
}

thread_local! {
    static PLAYER: RefCell<Option<Player>> = RefCell::new(None);
    static PLAYER_FINDER: RefCell<PlayerFinder> = RefCell::new(PlayerFinder::new().unwrap());
}

fn find_players() -> Vec<Player> {
    PLAYER_FINDER.with_borrow(|player_finder| match player_finder.find_all() {
        Ok(players) => players,
        Err(e) => {
            error!("cannot find players!: {e}");
            panic!("please check your d-bus connection!")
        }
    })
}

impl TryFrom<Metadata> for TrackMeta {
    type Error = PlayerStatus;

    fn try_from(meta: Metadata) -> Result<Self, Self::Error> {
        let track_id = meta.track_id();
        let title = meta
            .title()
            .ok_or(())
            .map_err(|_| PlayerStatus::Unsupported("cannot get title"))?
            .to_string();
        let album = meta.album_name().map(ToOwned::to_owned);
        let artists: Option<Vec<_>> = meta
            .artists()
            .map(|v| v.iter().map(ToString::to_string).collect());
        let length = meta.length();

        Ok(Self {
            unique_song_id: track_id.map(|id| id.to_string()),
            title,
            album,
            artists,
            length,
        })
    }
}
