#![allow(unused)]
use std::{
    fmt::{Display, Write},
    time::Duration,
};

use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::{
    app::{dialog::show_dialog, Window},
    log::{error, info, warn},
    lyric_providers::LyricOwned,
    sync::{lyric::cache::update_lyric_cache, LyricState, TrackState, LYRIC, TRACK_PLAYING_STATE},
    utils::gettext,
};

pub fn update_cache() {
    TRACK_PLAYING_STATE.with_borrow(|TrackState { cache_path, .. }| {
        if let Some(cache_path) = cache_path {
            update_lyric_cache(cache_path);
        }
    });
}

#[cfg(feature = "export-lyric")]
pub fn make_lrc_line(text: impl Display, start_time: Duration) -> String {
    let mut ms = start_time.as_millis() as u64;
    let mut sec = ms / 1000;
    let min = sec / 60;
    sec %= 60;
    ms %= 1000;

    format!("[{min:02}:{sec:02}.{ms:03}]{text}")
}

#[cfg(feature = "export-lyric")]
pub async fn export_lyric(window: &Window, is_original: bool) {
    info!("spawned export-lyric: original={is_original}");

    let meta = TRACK_PLAYING_STATE.with_borrow(|meta| meta.metainfo.clone());
    let current_lyrics = LYRIC.with_borrow(|l| {
        if is_original {
            l.origin.clone()
        } else {
            l.translation.clone()
        }
    });
    let offset = window.imp().lyric_offset_ms.get();

    let LyricOwned::LineTimestamp(lines) = current_lyrics else {
        let error_msg = gettext("lyric not exising!");
        error!(error_msg);
        show_dialog(gtk::Window::NONE, &error_msg, gtk::MessageType::Error);
        return;
    };

    let mut output = String::new();

    let _ = output.write_str("[re:waylyrics]\n");
    let _ = output.write_str(concat!("[ve:", env!("CARGO_PKG_VERSION"), "]\n"));
    if let Some(meta) = meta {
        if let Some(value) = meta.title {
            let _ = output.write_fmt(format_args!("[ti:{value}]\n"));
        }
        if let Some(value) = meta.artists {
            let value = value.join(", ");
            let _ = output.write_fmt(format_args!("[ar:{value}]\n"));
        }
        if let Some(album) = meta.album {
            let _ = output.write_fmt(format_args!("[al:{album}]\n"));
        }
    } else {
        warn!("metainfo not found! will not generate");
    }

    let _ = output.write_fmt(format_args!("[offset:{offset}]\n"));
    let _ = output.write_char('\n');

    for line in lines {
        output += &make_lrc_line(&line.text, line.start_time);
        output += "\n";
    }

    let Some(lrc_file) = rfd::AsyncFileDialog::new()
        .set_title(gettext("Export a lyrics file"))
        .add_filter("Simple LRC", &["lrc"])
        .save_file()
        .await
    else {
        info!("user canceled selection");
        return;
    };

    if let Err(e) = lrc_file.write(output.as_bytes()).await {
        let prompt = gettext("failed to export: ");
        let error_msg = format!("{prompt}{e}");
        error!(error_msg);
        show_dialog(gtk::Window::NONE, &error_msg, gtk::MessageType::Error);
    }
}

#[cfg(feature = "import-lyric")]
pub async fn import_lyric(window: &Window, is_original: bool) {
    use crate::lyric_providers::{utils::lrc_iter, Lyric};

    info!("spawned import-lyric: original={is_original}");

    let lrc_file = rfd::AsyncFileDialog::new()
        .set_title(gettext("Select a lyrics file"))
        .add_filter("Simple LRC", &["lrc"])
        .pick_file()
        .await;

    let Some(lrc_file) = lrc_file else {
        info!("user canceled selection");
        return;
    };
    let lrc = match String::from_utf8(lrc_file.read().await) {
        Ok(lrc) => lrc,
        Err(e) => {
            let prompt = gettext("failed to read LRC in UTF-8: ");
            let error_msg = format!("{prompt}{e}");
            error!(error_msg);
            show_dialog(gtk::Window::NONE, &error_msg, gtk::MessageType::Error);
            return;
        }
    };
    let lyric = match lrc_iter(lrc.lines()) {
        Ok(r) => r,
        Err(e) => {
            let prompt = gettext("input LRC in unsupported format: ");
            let error_msg = format!("{prompt}{e}");
            error!(error_msg);
            show_dialog(gtk::Window::NONE, &error_msg, gtk::MessageType::Error);
            return;
        }
    };
    LYRIC.with_borrow_mut(
        |LyricState {
             origin,
             translation,
         }| {
            if is_original {
                *origin = Lyric::LineTimestamp(lyric).into_owned();
            } else {
                *translation = Lyric::LineTimestamp(lyric).into_owned()
            }
        },
    );
    let cache_lyrics = window.imp().cache_lyrics.get();
    if cache_lyrics {
        update_cache();
    }
}
