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
                match (origin, translation) {
                    (
                        LyricOwned::LineTimestamp(origin_lyric),
                        LyricOwned::LineTimestamp(translation_lyric),
                    ) => {
                        let next_translation = crate::lyric::utils::find_next_lyric(&elapsed, translation_lyric);
                        let next_origin = crate::lyric::utils::find_next_lyric(&elapsed, origin_lyric);
                        set_lyric(&window, next_translation, "above");
                        set_lyric(&window, next_origin, "below");
                    }
                    (
                        LyricOwned::LineTimestamp(origin_lyric),
                        _
                    ) => {
                        let next_origin = crate::lyric::utils::find_next_lyric(&elapsed, origin_lyric);
                        set_lyric(&window, next_origin, "above");
                    }
                    _ => (),
                }
            }
        });

        Continue(true)
    });
}

fn set_lyric(window: &app::Window, new_text: Option<&LyricLineOwned>, position: &str) {
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
        get_label(window, position).set_label(text);
    } else {
        window.imp().lyric_playing.set(None);
        get_label(window, position).set_label("");
    }
}
