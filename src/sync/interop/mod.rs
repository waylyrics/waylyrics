use crate::sync::lyric::fetch::LyricHint;

pub(crate) mod common;
pub use common::register_sync_task;

#[cfg(unix)]
mod mpris;
#[cfg(unix)]
pub use mpris::MPRIS as OS;

#[cfg(target_os = "windows")]
mod smtc;
#[cfg(target_os = "windows")]
pub use smtc::GSMTC as OS;

pub trait OsImp {
    fn clean_player();
    fn connect_player_with_id(player_id: impl AsRef<str>);
    fn hint_from_player() -> Option<LyricHint>;
    fn list_players() -> Vec<PlayerId>;
    fn reconnect_player() -> bool;
    /// This function should:
    ///     call `update_lyric` when fetched new metadata
    ///     update window.imp().lyric_start
    ///     update TRACK_PLAYING_STATE
    fn try_sync_track(window: &crate::app::Window) -> Result<(), PlayerStatus>;
}

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
