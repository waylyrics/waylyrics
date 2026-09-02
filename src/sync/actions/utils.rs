#![allow(unused)]
use std::fmt::{Display, Write};
use std::time::Duration;

use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::app::dialog::show_dialog;
use crate::app::Window;
use crate::log::{error, info, warn};
use crate::lyric_providers::LyricOwned;
use crate::sync::lyric::cache::update_lyric_cache;
use crate::sync::{LyricState, TrackState, LYRIC, TRACK_PLAYING_STATE};
use crate::utils::gettext;

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
    use gtk::gio::prelude::FileExt;
    use gtk::gio::FileCreateFlags;
    use gtk::glib::Priority;
    use gtk::{FileDialog, FileFilter};

use crate::tokio_spawn;

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
        show_dialog(Some(window), &error_msg, gtk::MessageType::Error);
        return;
    };

    let mut output = String::default();

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

    let filters = gtk::gio::ListStore::new::<FileFilter>();
    let filter = FileFilter::new();
    filter.add_mime_type("text/plain");
    filter.add_suffix("lrc");
    filter.set_name(Some("Simple LRC"));
    filters.append(&filter);
    let Some(mut lrc_file) = FileDialog::builder()
        .title(gettext("Export a lyrics file"))
        .filters(&filters)
        .default_filter(&filter)
        .build()
        .save_future(Some(window))
        .await.ok().and_then(|f| f.path())
    else {
        info!("user cancelled selection");
        return;
    };
    lrc_file.set_extension("lrc");

    if let Ok(Err(e)) = tokio_spawn!(tokio::fs::write(lrc_file, output)).await {
        let prompt = gettext("failed to export: ");
        let error_msg = format!("{prompt}{e}");
        error!(error_msg);
        show_dialog(Some(window), &error_msg, gtk::MessageType::Error);
    }
}

#[cfg(feature = "import-lyric")]
pub async fn import_lyric(window: &Window, is_original: bool) {
    use gtk::gio::prelude::{FileExt, IOStreamExt, InputStreamExt, InputStreamExtManual};
    use gtk::glib::Priority;
    use gtk::{FileDialog, FileFilter};

    use crate::lyric_providers::utils::lrc_iter;
    use crate::lyric_providers::Lyric;
use crate::tokio_spawn;

    info!("spawned import-lyric: original={is_original}");

    let filters = gtk::gio::ListStore::new::<FileFilter>();
    let filter = FileFilter::new();
    filter.add_mime_type("text/plain");
    filter.add_suffix("lrc");
    filter.set_name(Some("Simple LRC"));
    filters.append(&filter);
    let Some(lrc_file) = FileDialog::builder()
        .title(gettext("Select a lyrics file"))
        .filters(&filters)
        .default_filter(&filter)
        .build()
        .open_future(Some(window))
        .await.ok().and_then(|f| f.path())
    else {
        info!("user cancelled selection");
        return;
    };

    let lrc = match tokio_spawn!(tokio::fs::read_to_string(lrc_file)).await {
        Ok(Ok(bytes)) => bytes,
        Ok(Err(e)) => {
            let prompt = gettext("failed to read LRC in UTF-8: ");
            let error_msg = format!("{prompt}{e}");
            error!(error_msg);
            show_dialog(Some(window), &error_msg, gtk::MessageType::Error);
            return;
        }
        Err(e) => panic!("tokio join error"),
    };
    let lyric = match lrc_iter(lrc.lines()) {
        Ok(r) => r,
        Err(e) => {
            let prompt = gettext("input LRC in unsupported format: ");
            let error_msg = format!("{prompt}{e}");
            error!(error_msg);
            show_dialog(Some(window), &error_msg, gtk::MessageType::Error);
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
