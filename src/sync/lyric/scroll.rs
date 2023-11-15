use std::time::Duration;

use gtk::glib::ControlFlow;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib, prelude::*};
use gtk::{glib::WeakRef, Application};

use crate::app::{self, get_label};
use crate::config::LyricDisplay;
use crate::lyric_providers::{LyricLineOwned, LyricOwned};

use crate::sync::{TrackState, LYRIC, TRACK_PLAYING_STATE};

pub fn register_lyric_display(app: WeakRef<Application>, interval: Duration) {
    glib::timeout_add_local(interval, move || {
        let Some(app) = app.upgrade() else {
            return ControlFlow::Break;
        };

        let mut windows = app.windows();
        if windows.is_empty() {
            return ControlFlow::Continue;
        }
        let window: app::Window = windows.remove(0).downcast().unwrap();

        if TRACK_PLAYING_STATE.with_borrow(
            |TrackState {
                 metainfo, paused, ..
             }| *paused || metainfo.is_none(),
        ) {
            // no music is playing
            return ControlFlow::Continue;
        }

        refresh_lyric(&window);

        ControlFlow::Continue
    });
}

fn set_lyric_with_mode(
    window: &app::Window,
    next_translation: Option<&LyricLineOwned>,
    next_origin: Option<&LyricLineOwned>,
) {
    match window.imp().lyric_display_mode.get() {
        LyricDisplay::ShowBoth => {
            if next_translation.is_some() {
                set_lyric(window, next_translation, "above", true);
                set_lyric(window, next_origin, "below", false);
            } else {
                set_lyric(window, next_origin, "above", false);
            }
        }
        LyricDisplay::Origin => {
            set_lyric(window, next_origin, "above", false);
        }
        LyricDisplay::PreferTranslation => {
            if next_translation.is_none() {
                set_lyric(window, next_origin, "above", false);
            } else {
                set_lyric(window, next_translation, "above", true);
            }
        }
    }
}

fn set_lyric(
    window: &app::Window,
    next_text: Option<&LyricLineOwned>,
    position: &str,
    translation: bool,
) {
    let status = &window.imp().lyric_playing[translation as usize];
    if let Some(LyricLineOwned { text, start_time }) = next_text {
        let text = text.trim();
        if status.get().is_some_and(|time| &time == start_time) {
            return;
        }

        status.set(Some(*start_time));
        get_label(window, position).set_label(text);
    } else {
        status.set(None);
        get_label(window, position).set_label("");
    }
}

pub fn refresh_lyric(window: &app::Window) {
    LYRIC.with_borrow(|(origin, translation)| {
        let system_time = window.imp().lyric_start.get().unwrap();
        let elapsed = system_time.elapsed().ok();
        if let Some(elapsed) = elapsed {
            match (origin, translation) {
                (
                    LyricOwned::LineTimestamp(origin_lyric),
                    LyricOwned::LineTimestamp(translation_lyric),
                ) => {
                    let next_translation =
                        crate::lyric_providers::utils::find_next_lyric(&elapsed, translation_lyric);
                    let next_origin =
                        crate::lyric_providers::utils::find_next_lyric(&elapsed, origin_lyric);
                    set_lyric_with_mode(window, next_translation, next_origin);
                }
                (LyricOwned::LineTimestamp(origin_lyric), _) => {
                    let next_origin =
                        crate::lyric_providers::utils::find_next_lyric(&elapsed, origin_lyric);
                    set_lyric_with_mode(window, None, next_origin);
                }
                _ => (),
            }
        }
    });
}
