use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::time::Duration;

use crate::lyric_providers::LyricOwned;

mod interop;
pub mod lyric;
mod utils;

pub use interop::list_players;
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
    pub static TRACK_PLAYING_STATE: RefCell<TrackState> = RefCell::new(Default::default());
}

pub static ENABLE_LOCAL_LYRIC: OnceLock<bool> = OnceLock::new();

mod actions;
pub use actions::{
    register_connect, register_disconnect, register_refetch_lyric, register_reload_lyric,
    register_remove_lyric, register_search_lyric,
};
#[cfg(feature = "import-lyric")]
pub use actions::{register_import_original_lyric, register_import_translated_lyric};

#[cfg(feature = "action-event")]
pub use actions::{init_play_action_channel, PlayAction, PLAY_ACTION};

pub use interop::register_sync_task;
pub use interop::PlayerId;
pub use utils::{
    extract_translated_lyric, filter_original_lyric, fuzzy_match_song, get_lyric_cache_path,
    set_current_lyric,
};
