use std::cell::RefCell;

mod sync_task;
pub use sync_task::register_sync_task;

mod hint;
pub use hint::hint_from_player;

use mpris::{Metadata, Player, PlayerFinder};

use crate::log::error;

use crate::sync::interop::PlayerStatus;
use crate::sync::TrackMeta;

use super::PlayerId;

pub fn clean_player() {
    PLAYER.take();
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
            inner_id: p.identity().to_owned(),
        })
        .collect()
}

thread_local! {
    static PLAYER: RefCell<Option<Player>> = const { RefCell::new(None) };
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
        let title = meta.title().map(str::to_string);
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
