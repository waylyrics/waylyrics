use std::time::Duration;

use gtk::glib::ControlFlow;
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::{glib, prelude::*};
use gtk::{glib::WeakRef, Application};

use crate::app::{self, get_label};
use crate::config::LyricDisplayMode;
use crate::lyric_providers::{LyricLineOwned, LyricOwned};

use crate::sync::{LyricState, TrackState, LYRIC, TRACK_PLAYING_STATE};

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
    translation: Option<&LyricLineOwned>,
    origin: Option<&LyricLineOwned>,
) {
    match window.imp().lyric_display_mode.get() {
        LyricDisplayMode::ShowBoth => {
            set_lyric(
                window,
                translation,
                "above",
                #[cfg(feature = "set-label-class")]
                &["translation"],
            );
            set_lyric(
                window,
                origin,
                "below",
                #[cfg(feature = "set-label-class")]
                &["origin"],
            );
        }
        LyricDisplayMode::ShowBothRev => {
            set_lyric(
                window,
                origin,
                "above",
                #[cfg(feature = "set-label-class")]
                &["origin"],
            );
            set_lyric(
                window,
                translation,
                "below",
                #[cfg(feature = "set-label-class")]
                &["translation"],
            );
        }
        LyricDisplayMode::Origin => {
            set_lyric(
                window,
                origin,
                "above",
                #[cfg(feature = "set-label-class")]
                &["origin"],
            );
            set_lyric(
                window,
                None,
                "below",
                #[cfg(feature = "set-label-class")]
                &[],
            );
        }
        LyricDisplayMode::PreferTranslation => {
            #[cfg(feature = "set-label-class")]
            let class = if translation.is_some() {
                "translation"
            } else {
                "origin"
            };
            let text = translation.or(origin);
            set_lyric(
                window,
                text,
                "above",
                #[cfg(feature = "set-label-class")]
                &[class],
            );
            set_lyric(
                window,
                None,
                "below",
                #[cfg(feature = "set-label-class")]
                &[],
            );
        }
    }
}

fn set_lyric(
    window: &app::Window,
    text: Option<&LyricLineOwned>,
    position: &str,
    #[cfg(feature = "set-label-class")] classes: &[&str],
) {
    let text = text
        .map(|LyricLineOwned { text, .. }| text.as_str().trim())
        .unwrap_or_default();

    let label = get_label(window, position);

    label.set_label(text);
    #[cfg(feature = "set-label-class")]
    label.set_css_classes(classes);
}

pub fn refresh_lyric(window: &app::Window) {
    LYRIC.with_borrow(
        |LyricState {
             origin,
             translation,
         }| {
            let system_time = window.imp().lyric_start.get().unwrap();
            let elapsed = system_time.elapsed().ok();
            let Some(elapsed) = elapsed else {
                return;
            };
            match (origin, translation) {
                (
                    LyricOwned::LineTimestamp(origin_lyric),
                    LyricOwned::LineTimestamp(translation_lyric),
                ) => {
                    let translation =
                        crate::lyric_providers::utils::find_next_lyric(&elapsed, translation_lyric);
                    let origin =
                        crate::lyric_providers::utils::find_next_lyric(&elapsed, origin_lyric);
                    set_lyric_with_mode(window, translation, origin);
                }
                (LyricOwned::LineTimestamp(origin_lyric), _) => {
                    let origin =
                        crate::lyric_providers::utils::find_next_lyric(&elapsed, origin_lyric);
                    set_lyric_with_mode(window, None, origin);
                }
                _ => (),
            }
        },
    );
}
