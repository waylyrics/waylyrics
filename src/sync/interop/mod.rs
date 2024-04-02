use crate::sync::lyric::fetch::LyricHint;

pub(crate) mod common;
pub use common::register_sync_task;

#[cfg(unix)]
mod mpris;
#[cfg(unix)]
mod unix {
    use super::{mpris::MPRIS, OsImp, PlayerId, PlayerStatus};
    use crate::sync::lyric::fetch::LyricHint;
    pub fn clean_player() {
        MPRIS::clean_player()
    }
    pub fn connect_player_with_id(player_id: impl AsRef<str>) {
        MPRIS::connect_player_with_id(player_id)
    }
    pub fn hint_from_player() -> Option<LyricHint> {
        MPRIS::hint_from_player()
    }
    pub fn list_players() -> Vec<PlayerId> {
        MPRIS::list_players()
    }
    pub(super) fn reconnect_player() -> bool {
        MPRIS::reconnect_player()
    }
    pub(super) fn try_sync_track(window: &crate::app::Window) -> Result<(), PlayerStatus> {
        MPRIS::try_sync_track(window)
    }
}
#[cfg(unix)]
pub use unix::*;

#[cfg(target_os = "windows")]
mod smtc;
#[cfg(target_os = "windows")]
mod windows {
    use super::smtc::GSMTC;
    use super::{OsImp, PlayerId, PlayerStatus};
    use crate::sync::lyric::fetch::LyricHint;
    pub fn clean_player() {
        GSMTC::clean_player()
    }
    pub fn connect_player_with_id(player_id: impl AsRef<str>) {
        GSMTC::connect_player_with_id(player_id)
    }
    pub fn hint_from_player() -> Option<LyricHint> {
        GSMTC::hint_from_player()
    }
    pub fn list_players() -> Vec<PlayerId> {
        GSMTC::list_players()
    }
    pub(super) fn reconnect_player() -> bool {
        GSMTC::reconnect_player()
    }
    pub(super) fn try_sync_track(window: &crate::app::Window) -> Result<(), PlayerStatus> {
        GSMTC::try_sync_track(window)
    }
}
#[cfg(target_os = "windows")]
pub use windows::*;

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
