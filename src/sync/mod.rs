use std::cell::RefCell;
use std::path::PathBuf;
use std::time::Duration;

use crate::lyric_providers::LyricOwned;

mod interop;
pub mod lyric;
mod utils;

pub use interop::list_player_names;
pub use lyric::scroll::register_lyric_display;

/// metadata from connected player
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackMeta {
    pub unique_song_id: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub artists: Option<Vec<String>>,
    pub length: Option<Duration>,
}

#[derive(Clone, Debug, Default)]
pub struct TrackState {
    pub metainfo: Option<TrackMeta>,
    pub paused: bool,
    pub cache_path: Option<PathBuf>,
}

#[derive(Clone, Default)]
pub struct LyricState {
    pub origin: LyricOwned,
    pub translation: LyricOwned,
}

thread_local! {
    static LYRIC: RefCell<LyricState> = const { RefCell::new(LyricState { origin: LyricOwned::None, translation: LyricOwned::None }) };
    /// A global variable that contains current playing state (excluding lyrics)
    /// including: track_id, paused, cache_path
    static TRACK_PLAYING_STATE: RefCell<TrackState> = RefCell::new(Default::default());
}

mod acts;
pub use acts::{
    register_connect, register_disconnect, register_refetch_lyric, register_remove_lyric,
    register_search_lyric,
};

pub use acts::{init_play_action_channel, PlayAction, PLAY_ACTION};
pub use interop::register_sync_task;
pub use interop::PlayerId;
pub use utils::{get_lyric_cache_path, set_current_lyric};
