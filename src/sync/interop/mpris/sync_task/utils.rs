use crate::log::*;
use mpris::{PlaybackStatus, Player, PlayerFinder};

use crate::{PLAYER_IDENTITY_BLACKLIST, PLAYER_NAME_BLACKLIST};

/// find a likely active player
/// ignore players in blacklists
pub fn find_next_player(player_finder: &PlayerFinder) -> Option<Player> {
    let identity_blacklisted = |p: &Player| {
        PLAYER_IDENTITY_BLACKLIST.with_borrow(|ids| {
            let identity = p.identity();
            ids.iter().any(|s| s == identity)
        })
    };
    let name_blacklisted = |p: &Player| {
        PLAYER_NAME_BLACKLIST.with_borrow(|names| {
            let name = p.bus_name_player_name_part();
            names.iter().any(|s| s == name)
        })
    };

    let active = player_finder.find_active().ok()?;
    if !name_blacklisted(&active) && !identity_blacklisted(&active) {
        return Some(active);
    }

    let players = player_finder.find_all().ok()?;
    players
        .into_iter()
        .filter(|p| {
            p.track_progress(0)
                .is_ok_and(|mut t| t.tick().progress.playback_status() == PlaybackStatus::Playing)
        })
        .filter(|p| !identity_blacklisted(p))
        .find(|p| !name_blacklisted(p))
}
