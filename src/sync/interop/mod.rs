pub(crate) mod common;
pub use common::register_sync_task;

#[cfg(unix)]
mod mpris;
#[cfg(unix)]
use mpris::try_sync_track;
#[cfg(unix)]
pub use mpris::{
    clean_player, connect_player_with_id, hint_from_player, list_players,
    reconnect_player,
};

#[cfg(target_os = "windows")]
mod smtc;
#[cfg(target_os = "windows")]
pub use smtc::{
    clean_player, connect_player_with_id, hint_from_player, list_player_names, register_sync_task,
};

#[derive(Debug)]
pub enum PlayerStatus {
    Missing,
    Paused,
    Stopped,
    Unsupported(&'static str),
}

#[derive(Debug, Clone)]
pub struct PlayerId {
    pub player_name: String,
    pub inner_id: String,
}
