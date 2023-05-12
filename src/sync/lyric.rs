use std::time::Duration;

use gtk::{glib, prelude::*};
use gtk::{glib::WeakRef, Application};

use crate::app::get_label;
use crate::lyric::{LyricLineOwned, LyricOwned};

use super::{LYRIC, LYRIC_CURRENT, LYRIC_START, LYRIC_TRANSLATION_CURRENT, TRACK_PLAYING_PAUSED};

pub fn register_lyric_display(app: WeakRef<Application>, interval: Duration) {
    glib::timeout_add_local(interval, move || {
        if let Some(app) = app.upgrade() {
            let windows = app.windows();
            if windows.is_empty() {
                return Continue(true);
            }
            if TRACK_PLAYING_PAUSED.with_borrow(|(play, paused)| *paused || play.is_none()) {
                // no music is playing
                return Continue(true); // skip lyric scrolling
            }

            LYRIC.with_borrow(|(origin, translation)| {
                let elapsed = LYRIC_START.with_borrow(|start| start.elapsed().ok());
                if let Some(elapsed) = elapsed {
                    if let LyricOwned::LineTimestamp(lyric) = origin {
                        let new_text = crate::lyric::utils::find_next_lyric(&elapsed, lyric);
                        set_origin_lyric(&windows[0], new_text);
                    }
                    if let LyricOwned::LineTimestamp(lyric) = translation {
                        let new_text = crate::lyric::utils::find_next_lyric(&elapsed, lyric);
                        set_translation_lyric(&windows[0], new_text);
                    }
                }
            });

            return Continue(true);
        }

        Continue(false)
    });
}

fn set_origin_lyric(window: &gtk::Window, new_text: Option<&LyricLineOwned>) {
    if let Some(LyricLineOwned { text, start_time }) = new_text {
        if let Some(timestamp) = LYRIC_CURRENT.with_borrow(|status| *status) {
            if &timestamp == start_time {
                return;
            }
        }

        LYRIC_CURRENT.set(Some(*start_time));
        get_label(window, false).set_label(text);
    } else {
        LYRIC_CURRENT.set(None);
        get_label(window, false).set_label("");
    }
}

fn set_translation_lyric(window: &gtk::Window, new_text: Option<&LyricLineOwned>) {
    if let Some(LyricLineOwned { text, start_time }) = new_text {
        if let Some(timestamp) = LYRIC_TRANSLATION_CURRENT.with_borrow(|status| *status) {
            if &timestamp == start_time {
                return;
            }
        }

        LYRIC_TRANSLATION_CURRENT.set(Some(*start_time));
        get_label(window, true).set_label(text);
    } else {
        LYRIC_TRANSLATION_CURRENT.set(None);
        get_label(window, true).set_label("");
    }
}
