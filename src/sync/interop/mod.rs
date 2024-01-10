pub mod acts;

#[cfg(unix)]
mod mpris;
#[cfg(unix)]
pub use mpris::{
    clean_player, connect_player_with_id, hint_from_player, list_player_names, register_sync_task,
    update_lyric,
};

#[cfg(target_os = "windows")]
mod smtc;
#[cfg(target_os = "windows")]
pub use smtc::{
    clean_player, connect_player_with_id, hint_from_player, list_player_names, register_sync_task,
    update_lyric,
};

#[derive(Debug)]
pub enum PlayerStatus {
    Missing,
    Paused,
    Unsupported(&'static str),
}

#[derive(Debug)]
pub struct PlayerId {
    pub player_name: String,
    pub inner_id: String,
}
