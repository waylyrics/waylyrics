use std::borrow::Cow;
use std::error::Error;
use std::time::{Duration, SystemTime};

use gtk::glib::WeakRef;
use gtk::{glib, Application};
use gtk::{prelude::*, Window};
use mpris::{Metadata, PlaybackStatus, Player, ProgressTracker};
use tracing::{debug, error, info};

use crate::app::get_label;
use crate::lyric::netease::NeteaseLyricProvider;
use crate::lyric::{LyricOwned, LyricProvider, LyricStore, SongInfo};
use crate::sync::LYRIC_OFFSET_MILLISEC;

use super::{
    utils, CACHE_LYRICS, DEFAULT_TEXT, LYRIC, LYRIC_START, PLAYER, PLAYER_FINDER,
    TOKIO_RUNTIME_HANDLE, TRACK_PLAYING_PAUSED,
};

enum PlayerStatus {
    Missing,
    Paused,
    Unsupported(&'static str),
}

fn try_sync_player(window: &gtk::Window) -> Result<(), PlayerStatus> {
    PLAYER.with_borrow(|player| {
        let player = player.as_ref().ok_or(PlayerStatus::Missing)?;

        if !player.is_running() {
            info!("disconnected from player: {}", player.identity());
            return Err(PlayerStatus::Missing);
        }

        let mut progress_tracker = ProgressTracker::new(player, 0)
            .map_err(|_| PlayerStatus::Unsupported("cannot fetch progress"))?;

        let progress_tick = progress_tracker.tick();
        if progress_tick.progress.playback_status() != PlaybackStatus::Playing {
            return Err(PlayerStatus::Paused);
        }
        let track_meta = player
            .get_metadata()
            .map_err(|_| PlayerStatus::Unsupported("cannot get metadata of track playing"))?;
        let need_update_lyric =
            TRACK_PLAYING_PAUSED.with_borrow_mut(|(track_id_playing, paused)| {
                if let Some(track_id) = track_meta.track_id() {
                    let need = track_id_playing.is_none()
                        || track_id_playing.as_ref().is_some_and(|p| p != &track_id)
                            && !(*paused
                                && track_id_playing.as_ref().is_some_and(|p| p == &track_id));

                    *track_id_playing = Some(track_id);
                    *paused = false;
                    need
                } else {
                    *track_id_playing = None;
                    *paused = false;
                    false
                }
            });

        if need_update_lyric {
            utils::clear_lyric();

            let title = track_meta
                .title()
                .ok_or(PlayerStatus::Unsupported("cannot get song title"))?;
            let album = track_meta.album_name();
            let artists = track_meta.artists();

            let length = track_meta.length();

            let cache = CACHE_LYRICS.with_borrow(|cache| *cache);
            let fetch_result = if cache {
                super::cache::fetch_lyric_cached(title, album, artists.as_deref(), length, window)
            } else {
                fetch_lyric(title, album, artists.as_deref(), length, window)
            };

            if let Err(e) = fetch_result {
                error!("lyric fetch error: {e}");
            }

            get_label(window, false).set_label(DEFAULT_TEXT);
            get_label(window, true).set_label("");
        }

        // sync play position
        let position = player
            .get_position()
            .map_err(|_| PlayerStatus::Unsupported("cannot get playback position"))?;
        let mut start =
            SystemTime::now()
                .checked_sub(position)
                .ok_or(PlayerStatus::Unsupported(
                    "Position is greater than SystemTime",
                ))?;

        let offset = LYRIC_OFFSET_MILLISEC.with_borrow(|offset| *offset);
        if offset.is_negative() {
            start = start
                .checked_sub(Duration::from_millis(offset.unsigned_abs()))
                .expect("infinite offset time");
        } else {
            start = start
                .checked_add(Duration::from_millis(offset as _))
                .expect("infinite offset time");
        }

        LYRIC_START.set(start);

        Ok(())
    })
}

pub fn register_mpris_sync(app: WeakRef<Application>, interval: Duration) {
    glib::timeout_add_local(interval, move || {
        if let Some(app) = app.upgrade() {
            let windows = app.windows();
            if windows.is_empty() {
                return Continue(true);
            }

            let sync_status = try_sync_player(&windows[0]);

            match sync_status {
                Err(PlayerStatus::Missing) => {
                    PLAYER_FINDER.with_borrow(|player_finder| {
                        if let Ok(player) = player_finder.find_active() {
                            info!("connected to player: {}", player.identity());
                            PLAYER.set(Some(player));
                        } else {
                            PLAYER.set(None);
                        }
                    });
                    get_label(&windows[0], true).set_label(DEFAULT_TEXT);
                    get_label(&windows[0], false).set_label("");
                    TRACK_PLAYING_PAUSED.set((None, false));
                }
                Err(PlayerStatus::Unsupported(kind)) => {
                    get_label(&windows[0], true).set_label("Unsupported Player");
                    get_label(&windows[0], false).set_label("");

                    utils::clear_lyric();
                    error!(kind);
                }
                Err(PlayerStatus::Paused) => {
                    TRACK_PLAYING_PAUSED.with_borrow_mut(|(_, paused)| *paused = true)
                }
                _ => (),
            }
        }
        Continue(true)
    });
}

pub fn fetch_lyric(
    title: &str,
    album: Option<&str>,
    _artists: Option<&[&str]>,
    length: Option<Duration>,
    window: &gtk::Window,
) -> Result<(), Box<dyn std::error::Error>> {
    let artists = _artists
        .map(|s| Cow::Owned(s.join(",")))
        .unwrap_or(Cow::Borrowed("Unknown"));

    if let Some(result) = set_lyric_with_songid_or_file(title, &artists, window) {
        info!("fetched lyric directly");
        return result;
    }

    let provider = NeteaseLyricProvider::new().unwrap();

    let search_result = search_song(
        provider.as_ref(),
        album.as_deref().unwrap_or_default(),
        _artists.unwrap_or(&[]),
        title,
    )?;

    if let Some(&song_id) =
        utils::match_likely_lyric(album.zip(Some(title)), length, &search_result)
    {
        info!("matched songid: {song_id}");
        set_lyric(provider.as_ref(), song_id, title, &artists, window)?;
        Ok(())
    } else {
        info!("Failed searching for {artists} - {title}",);
        utils::clear_lyric();
        Err("No lyric found".into())
    }
}

fn fetch_lyric_by_id<P: LyricProvider>(
    provider: &P,
    id: P::Id,
) -> Result<P::LStore, Box<dyn std::error::Error>> {
    TOKIO_RUNTIME_HANDLE.with_borrow(|handle| provider.query_lyric(handle, id))
}

fn search_song<P: LyricProvider>(
    provider: &P,
    album: &str,
    artists: &[&str],
    title: &str,
) -> Result<Vec<SongInfo<P::Id>>, Box<dyn std::error::Error>> {
    TOKIO_RUNTIME_HANDLE.with_borrow(|handle| provider.search_song(handle, album, artists, title))
}

fn set_lyric<P: LyricProvider>(
    provider: &P,
    song_id: P::Id,
    title: &str,
    artists: &str,
    window: &gtk::Window,
) -> Result<(), Box<dyn std::error::Error>> {
    let lyric = fetch_lyric_by_id(provider, song_id).unwrap();
    let olyric = lyric.get_lyric().into_owned();
    let tlyric = lyric.get_translated_lyric().into_owned();
    debug!("original lyric: {olyric:?}");
    debug!("translated lyric: {tlyric:?}");

    // show info to user if original lyric is empty or no timestamp
    match &olyric {
        LyricOwned::LineTimestamp(_) => (),
        _ => {
            info!("No lyric for {} - {title}", artists,);
        }
    }

    if let LyricOwned::LineTimestamp(_) = &tlyric {
    } else {
        info!("No translated lyric for {} - {title}", artists,);
        get_label(window, true).set_visible(false);
    }
    LYRIC.set((olyric, tlyric));

    Ok(())
}

fn set_lyric_with_songid_or_file(
    title: &str,
    artists: &str,
    window: &Window,
) -> Option<Result<(), Box<(dyn Error + 'static)>>> {
    PLAYER.with_borrow(|player| {
        let player = player
            .as_ref()
            .expect("player not exists in lyric fetching");
        let player_name = player.identity();
        match player_name {
            "mpv" => {
                tracing::warn!("local lyric files are still not supported");
                None
            }
            "Qcm" => {
                let provider = NeteaseLyricProvider::new().unwrap();
                const PREFIX: &str = "/trackid/";

                set_lyric_with_player_songid(
                    provider.as_ref(),
                    player,
                    title,
                    artists,
                    window,
                    |meta| {
                        meta.track_id()
                            .unwrap()
                            .as_str()
                            .strip_prefix(PREFIX)
                            .map(|id| id.parse().unwrap())
                    },
                )
            }
            "feeluown" => {
                let provider = NeteaseLyricProvider::new().unwrap();
                const PREFIX: &str = "fuo://netease/songs/";

                set_lyric_with_player_songid(
                    provider.as_ref(),
                    player,
                    title,
                    artists,
                    window,
                    |meta| {
                        meta.url()
                            .unwrap()
                            .strip_prefix(PREFIX)
                            .map(|id| id.parse().unwrap())
                    },
                )
            }
            "ElectronNCM" => {
                let provider = NeteaseLyricProvider::new().unwrap();
                const PREFIX: &str = "/org/mpris/MediaPlayer2/";

                set_lyric_with_player_songid(
                    provider.as_ref(),
                    player,
                    title,
                    artists,
                    window,
                    |meta| {
                        meta.track_id()
                            .unwrap()
                            .as_str()
                            .strip_prefix(PREFIX)
                            .map(|id| id.parse().unwrap())
                    },
                )
            }

            _ => None,
        }
    })
}

fn set_lyric_with_player_songid<P: LyricProvider>(
    provider: &P,
    player: &Player,
    title: &str,
    artists: &str,
    window: &Window,
    parse_id: impl Fn(&Metadata) -> Option<P::Id>,
) -> Option<Result<(), Box<dyn std::error::Error>>> {
    if let Ok(metadata) = player.get_metadata() {
        return parse_id(&metadata)
            .map(|song_id| set_lyric(provider, song_id, title, artists, window));
    }
    None
}
