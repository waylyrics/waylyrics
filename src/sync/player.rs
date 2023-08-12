use anyhow::Result;
use gtk::gio::SimpleAction;
use std::borrow::Cow;
use std::time::{Duration, SystemTime};

use gtk::glib::{VariantTy, WeakRef};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib, Application};
use gtk::{prelude::*, NamedAction, Shortcut, ShortcutController, ShortcutTrigger};
use mpris::{Metadata, PlaybackStatus, Player, ProgressTracker};
use tracing::{debug, error, info, trace, warn};

use crate::app;
use crate::lyric::netease::NeteaseLyricProvider;
use crate::lyric::{LyricOwned, LyricProvider, LyricStore, SongInfo};
use crate::sync::LYRIC;

use super::{utils, DEFAULT_TEXT, PLAYER, PLAYER_FINDER, TRACK_PLAYING_PAUSED};

enum PlayerStatus {
    Missing,
    Paused,
    Unsupported(&'static str),
}

fn try_sync_player(window: &crate::app::Window) -> Result<(), PlayerStatus> {
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
                let Some(track_id) = track_meta.track_id() else {
                    *track_id_playing = None;
                    *paused = false;
                    return false;
                };

                trace!("got track_id: {track_id}");

                let need = track_id_playing.is_none()
                    || track_id_playing.as_ref().is_some_and(|p| p != &track_id)
                        && !(*paused && track_id_playing.as_ref().is_some_and(|p| p == &track_id));

                *track_id_playing = Some(track_id);
                *paused = false;
                need
            });

        if need_update_lyric {
            utils::clear_lyric(&window);

            let title = track_meta
                .title()
                .ok_or(PlayerStatus::Unsupported("cannot get song title"))?;
            let album = track_meta.album_name();
            let artists = track_meta.artists();

            let length = track_meta.length();

            let fetch_result = if window.imp().cache_lyrics.get() {
                super::cache::fetch_lyric_cached(title, album, artists.as_deref(), length, window)
            } else {
                fetch_lyric(title, album, artists.as_deref(), length, window)
            };

            if let Err(e) = fetch_result {
                error!("lyric fetch error: {e}");
            }

            app::get_label(window, "above").set_label(DEFAULT_TEXT);
            app::get_label(window, "below").set_label("");
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

        let offset = window.imp().lyric_offset_ms.get();
        if offset.is_negative() {
            start = start
                .checked_sub(Duration::from_millis(offset.unsigned_abs()))
                .expect("infinite offset time");
        } else {
            start = start
                .checked_add(Duration::from_millis(offset as _))
                .expect("infinite offset time");
        }

        window.imp().lyric_start.set(Some(start));

        Ok(())
    })
}

pub fn register_action_disconnect(app: &Application) {
    let action = SimpleAction::new("disconnect", None);
    action.connect_activate(|_, _| {
        PLAYER.set(None);
    });
    app.add_action(&action);
}

pub fn register_sigusr1_disconnect() {
    glib::unix_signal_add_local(libc::SIGUSR1, move || {
        PLAYER.set(None);
        Continue(true)
    });
}

pub fn register_action_reload_lyric(app: &Application, wind: &app::Window, trigger: &str) {
    let action = SimpleAction::new("reload-lyric", None);
    action.connect_activate(move |_, _| {
        TRACK_PLAYING_PAUSED.set((None, false));
        info!("cleaned lyric");
    });
    app.add_action(&action);

    let shortcut = Shortcut::builder()
        .action(&NamedAction::new("app.reload-lyric"))
        .trigger(&ShortcutTrigger::parse_string(trigger).unwrap())
        .build();
    let controller = ShortcutController::new();
    controller.set_scope(gtk::ShortcutScope::Global);
    controller.add_shortcut(shortcut);
    wind.add_controller(controller);
}

pub fn register_action_connect(app: &Application) {
    let connect = SimpleAction::new("connect", Some(&VariantTy::STRING));
    connect.connect_activate(|_, player_id| {
        let Some(player_id) = player_id.and_then(|p| p.str()) else {
            warn!("did not received string paramter for action \'app.connect\'");
            return;
        };
        PLAYER_FINDER.with_borrow(|player_finder| {
            if let Ok(player) = player_finder.find_by_name(&player_id) {
                PLAYER.set(Some(player));
            } else {
                error!("cannot connect to: {player_id}");
            }
        });
    });
    app.add_action(&connect);
}

pub fn list_avaliable_players() -> Vec<Player> {
    PLAYER_FINDER.with_borrow(|player_finder| match player_finder.find_all() {
        Ok(players) => players,
        Err(e) => {
            error!("cannot find players!: {e}");
            panic!("please check your d-bus connection!")
        }
    })
}

pub fn register_mpris_sync(app: WeakRef<Application>, interval: Duration) {
    glib::timeout_add_local(interval, move || {
        let Some(app) = app.upgrade() else {
            return Continue(false);
        };

        let mut windows = app.windows();
        if windows.is_empty() {
            return Continue(true);
        }
        let window: app::Window = windows.remove(0).downcast().unwrap();

        let sync_status = try_sync_player(&window);

        match sync_status {
            Err(PlayerStatus::Missing) => {
                PLAYER_FINDER.with_borrow(|player_finder| {
                    let Ok(player) = player_finder.find_active() else {
                        PLAYER.set(None);
                        return;
                    };

                    info!("connected to player: {}", player.identity());
                    PLAYER.set(Some(player));
                });
                app::get_label(&window, "above").set_label(DEFAULT_TEXT);
                app::get_label(&window, "below").set_label("");
                utils::clear_lyric(&window);
                TRACK_PLAYING_PAUSED.set((None, false));
            }
            Err(PlayerStatus::Unsupported(kind)) => {
                app::get_label(&window, "above").set_label("Unsupported Player");
                app::get_label(&window, "below").set_label("");

                utils::clear_lyric(&window);
                error!(kind);
            }
            Err(PlayerStatus::Paused) => {
                TRACK_PLAYING_PAUSED.with_borrow_mut(|(_, paused)| *paused = true)
            }
            _ => (),
        }

        Continue(true)
    });
}

pub fn fetch_lyric(
    title: &str,
    album: Option<&str>,
    _artists: Option<&[&str]>,
    length: Option<Duration>,
    window: &app::Window,
) -> Result<()> {
    let artists = _artists
        .map(|s| Cow::Owned(s.join(",")))
        .unwrap_or(Cow::Borrowed("Unknown"));

    if let Some(result) = set_lyric_with_songid_or_file(title, &artists, window) {
        info!("fetched lyric directly");
        return result;
    }

    let provider = NeteaseLyricProvider::new().unwrap();

    let search_result = search_song(
        &provider,
        album.as_deref().unwrap_or_default(),
        _artists.unwrap_or(&[]),
        title,
    )?;

    let length_toleration_ms = window.imp().length_toleration_ms.get();
    let Some(&song_id) = utils::match_likely_lyric(
        album.zip(Some(title)),
        length,
        &search_result,
        length_toleration_ms,
    ) else {
        info!("Failed searching for {artists} - {title}",);
        utils::clear_lyric(&window);
        return Err(crate::lyric::Error::NoLyric)?;
    };

    info!("matched songid: {song_id}");
    set_lyric(&provider, song_id, title, &artists, window)?;
    Ok(())
}

fn fetch_lyric_by_id<P: LyricProvider>(provider: &P, id: P::Id) -> Result<P::LStore> {
    provider.query_lyric(id)
}

fn search_song<P: LyricProvider>(
    provider: &P,
    album: &str,
    artists: &[&str],
    title: &str,
) -> Result<Vec<SongInfo<P::Id>>> {
    provider.search_song(album, artists, title)
}

fn set_lyric<P: LyricProvider>(
    provider: &P,
    song_id: P::Id,
    title: &str,
    artists: &str,
    window: &app::Window,
) -> Result<()> {
    let lyric = fetch_lyric_by_id(provider, song_id)?;
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

    if !matches!(tlyric, LyricOwned::LineTimestamp(_)) {
        info!("No translated lyric for {} - {title}", artists,);
        app::get_label(window, "below").set_visible(false);
    }
    LYRIC.set((olyric, tlyric));

    Ok(())
}

fn set_lyric_with_songid_or_file(
    title: &str,
    artists: &str,
    window: &app::Window,
) -> Option<Result<(), anyhow::Error>> {
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
            "ElectronNCM" | "Qcm" => {
                let provider = NeteaseLyricProvider::new().unwrap();

                set_lyric_with_player_songid::<NeteaseLyricProvider>(
                    player,
                    |meta| {
                        meta.get("mpris:trackid")
                            .and_then(mpris::MetadataValue::as_str)
                            .and_then(|s| s.split('/').last())
                    },
                    |songid| songid.parse().ok(),
                )
                .map(|song_id| set_lyric(&provider, song_id, title, artists, window))
            }
            "feeluown" => {
                let provider = NeteaseLyricProvider::new().unwrap();

                set_lyric_with_player_songid::<NeteaseLyricProvider>(
                    player,
                    |meta| meta.url()?.strip_prefix("fuo://netease/songs/"),
                    |songid| songid.parse().ok(),
                )
                .map(|song_id| set_lyric(&provider, song_id, title, artists, window))
            }
            "YesPlayMusic" => {
                let provider = NeteaseLyricProvider::new().unwrap();

                set_lyric_with_player_songid::<NeteaseLyricProvider>(
                    player,
                    |meta| meta.url()?.strip_prefix("/trackid/"),
                    |songid| songid.parse().ok(),
                )
                .map(|song_id| set_lyric(&provider, song_id, title, artists, window))
            }
            _ => None,
        }
    })
}

fn set_lyric_with_player_songid<P: LyricProvider>(
    player: &Player,
    extract_field: impl for<'a> FnOnce(&'a Metadata) -> Option<&'a str>,
    parse_id: impl FnOnce(&str) -> Option<P::Id>,
) -> Option<P::Id> {
    player
        .get_metadata()
        .ok()
        .and_then(|metadata| extract_field(&metadata).and_then(parse_id))
}
