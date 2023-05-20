use std::time::Duration;

use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib, prelude::*};
use gtk::{glib::WeakRef, Application};

use crate::app::{self, get_label};
use crate::lyric::{LyricLineOwned, LyricOwned};

use super::{LYRIC, TRACK_PLAYING_PAUSED};

pub fn register_lyric_display(app: WeakRef<Application>, interval: Duration) {
    glib::timeout_add_local(interval, move || {
        let Some(app) = app.upgrade() else {
            return Continue(false);
        };

        let mut windows = app.windows();
        if windows.is_empty() {
            return Continue(true);
        }
        let window: app::Window = windows.remove(0).downcast().unwrap();

        if TRACK_PLAYING_PAUSED.with_borrow(|(play, paused)| *paused || play.is_none()) {
            // no music is playing
            return Continue(true); // skip lyric scrolling
        }

        LYRIC.with_borrow(|(origin, translation)| {
            let system_time = window.imp().lyric_start.get().unwrap();
            let elapsed = system_time.elapsed().ok();
            if let Some(elapsed) = elapsed {
                if let LyricOwned::LineTimestamp(lyric) = origin {
                    let new_text = crate::lyric::utils::find_next_lyric(&elapsed, lyric);
                    set_origin_lyric(&window, new_text);
                }
                if let LyricOwned::LineTimestamp(lyric) = translation {
                    let new_text = crate::lyric::utils::find_next_lyric(&elapsed, lyric);
                    set_translation_lyric(&window, new_text);
                }
            }
        });

        Continue(true)
    });
}

fn set_origin_lyric(window: &app::Window, new_text: Option<&LyricLineOwned>) {
    if let Some(LyricLineOwned { text, start_time }) = new_text {
        if window
            .imp()
            .lyric_playing
            .get()
            .is_some_and(|time| &time == start_time)
        {
            return;
        }

        window.imp().lyric_playing.set(Some(*start_time));
        get_label(window, false).set_label(text);
    } else {
        window.imp().lyric_playing.set(None);
        get_label(window, false).set_label("");
    }
}

fn set_translation_lyric(window: &app::Window, new_text: Option<&LyricLineOwned>) {
    if let Some(LyricLineOwned { text, start_time }) = new_text {
        if window
            .imp()
            .lyric_playing_translation
            .get()
            .is_some_and(|time| &time == start_time)
        {
            return;
        }

        window
            .imp()
            .lyric_playing_translation
            .set(Some(*start_time));
        get_label(window, true).set_label(text);
    } else {
        window.imp().lyric_playing_translation.set(None);
        get_label(window, true).set_label("");
    }
}
