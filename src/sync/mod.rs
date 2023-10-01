use std::cell::RefCell;
use std::path::PathBuf;

use mpris::{Player, PlayerFinder, TrackID};

use crate::lyric::LyricOwned;


mod utils;
mod lyric;
mod player;
mod cache;
mod search_window;

pub use lyric::register_lyric_display;
pub use player::register_mpris_sync;
pub use player::register_sigusr1_disconnect;
pub use player::register_action_disconnect;
pub use player::register_action_connect;
pub use player::register_action_search_lyric;
pub use player::register_action_remove_lyric;
pub use player::register_action_reload_lyric;

pub(crate) use player::list_avaliable_players;

thread_local! {
    static PLAYER: RefCell<Option<Player>> = RefCell::new(None);
    static PLAYER_FINDER: RefCell<PlayerFinder> = RefCell::new(PlayerFinder::new().unwrap());
    static LYRIC: RefCell<(LyricOwned, LyricOwned)> = RefCell::new((LyricOwned::None, LyricOwned::None));
    /// A global variable that contains current playing state (excluding lyrics)
    /// including: track_id, paused, cache_path
    static TRACK_PLAYING_STATE: RefCell<(Option<TrackID>, bool, Option<PathBuf>)> = RefCell::new((None, false, None));
}
