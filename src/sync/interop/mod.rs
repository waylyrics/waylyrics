mod mpris;

pub mod acts;

pub use mpris::{
    clean_player,
    connect_player_with_id,
    hint_from_player,
    list_player_names,
    register_sync_task,
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
