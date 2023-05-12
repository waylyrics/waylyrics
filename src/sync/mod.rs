use std::cell::RefCell;
use std::time::{SystemTime, Duration};

use mpris::{Player, PlayerFinder, TrackID};
use tokio::runtime::Handle;

use crate::lyric::{LyricOwned};

const DEFAULT_TEXT: &str = "Waylyrics";

mod utils;
mod lyric;
mod player;

pub use lyric::register_lyric_display;
pub use player::register_mpris_sync;

thread_local! {
    static PLAYER: RefCell<Option<Player>> = RefCell::new(None);
    static PLAYER_FINDER: RefCell<PlayerFinder> = RefCell::new(PlayerFinder::new().unwrap());

    static LYRIC: RefCell<(LyricOwned, LyricOwned)> = RefCell::new((LyricOwned::None, LyricOwned::None));
    static LYRIC_START: RefCell<SystemTime> = RefCell::new(SystemTime::now());
    static LYRIC_CURRENT: RefCell<Option<Duration>> = RefCell::new(None);
    static LYRIC_TRANSLATION_CURRENT: RefCell<Option<Duration>> = RefCell::new(None);

    static TRACK_PLAYING_PAUSED: RefCell<(Option<TrackID>, bool)> = RefCell::new((None, false));

    static TOKIO_RUNTIME_HANDLE: RefCell<Handle> = RefCell::new(Handle::current());

    pub static CACHE_LYRICS: RefCell<bool> = RefCell::new(false);
    pub static LYRIC_OFFSET_MILLISEC: RefCell<i64> = RefCell::new(0);
}
