use std::cell::RefCell;
use std::path::PathBuf;

use mpris::{Player, PlayerFinder};

use crate::lyric_providers::LyricOwned;

mod interop;
pub mod lyric;
mod search_window;
mod utils;

pub(crate) use interop::list_avaliable_players;
pub use lyric::scroll::register_lyric_display;

/// A struct from metadata in mpris::TrackID to avoid track_id and title unwrapping
#[derive(Clone, Debug)]
pub struct TrackMeta {
    pub track_id: mpris::TrackID,
    pub title: String,
    pub meta: mpris::Metadata,
}

#[derive(Clone, Debug, Default)]
pub struct TrackState {
    pub metainfo: Option<TrackMeta>,
    pub paused: bool,
    pub cache_path: Option<PathBuf>,
}

thread_local! {
    static PLAYER: RefCell<Option<Player>> = RefCell::new(None);
    static PLAYER_FINDER: RefCell<PlayerFinder> = RefCell::new(PlayerFinder::new().unwrap());
    static LYRIC: RefCell<(LyricOwned, LyricOwned)> = RefCell::new((LyricOwned::None, LyricOwned::None));
    /// A global variable that contains current playing state (excluding lyrics)
    /// including: track_id, paused, cache_path
    static TRACK_PLAYING_STATE: RefCell<TrackState> = RefCell::new(Default::default());
}

pub use interop::acts::{
    register_action_connect, register_action_disconnect, register_action_reload_lyric,
    register_action_remove_lyric, register_action_search_lyric, register_sigusr1_disconnect,
};

pub use interop::register_mpris_sync;
