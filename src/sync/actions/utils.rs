use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::{
    app::{dialog::show_dialog, Window},
    log::{error, info},
    sync::{lyric::cache::update_lyric_cache, LyricState, TrackState, LYRIC, TRACK_PLAYING_STATE},
};

pub fn update_cache() {
    TRACK_PLAYING_STATE.with_borrow(|TrackState { cache_path, .. }| {
        if let Some(cache_path) = cache_path {
            update_lyric_cache(cache_path);
        }
    });
}

pub async fn import_lyric(window: &Window, is_original: bool) {
    use crate::lyric_providers::{utils::lrc_iter, Lyric};
    use crate::utils::gettext;

    info!("spawned import-lyric: original={is_original}");

    let lrc_file = rfd::AsyncFileDialog::new()
        .set_title(&gettext("Select a lyrics file"))
        .add_filter("Simple LRC", &["lrc"])
        .pick_file()
        .await;

    let Some(lrc_file) = lrc_file else {
        return;
    };
    let lrc = match String::from_utf8(lrc_file.read().await) {
        Ok(lrc) => lrc,
        Err(e) => {
            let error_msg = format!("failed to read LRC in UTF-8: {e}");
            error!(error_msg);
            show_dialog(gtk::Window::NONE, &error_msg, gtk::MessageType::Error);
            return;
        }
    };
    if let Ok(lyric) = lrc_iter(lrc.lines()) {
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
    }
    let cache_lyrics = window.imp().cache_lyrics.get();
    if cache_lyrics {
        update_cache();
    }
}
