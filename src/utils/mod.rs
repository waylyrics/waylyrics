use anyhow::Result;
use gtk::{
    gio::SimpleAction, glib::WeakRef, prelude::*, subclass::prelude::ObjectSubclassIsExt,
    Application,
};
use std::time::Duration;

use crate::app::{Window, utils::set_click_pass_through};

pub fn parse_time(time: &str) -> Result<Duration, ParseError> {
    use rust_decimal::prelude::*;
    use rust_decimal_macros::dec;

    let time_ms = if time.ends_with("ms") {
        let sec = time.trim_end_matches("ms");
        Decimal::from_str_exact(sec)?
    } else if time.ends_with('s') {
        let milli_sec = time.trim_end_matches('s');
        Decimal::from_str_exact(milli_sec)? * dec!(1000)
    } else {
        return Err(ParseError::IllFormed);
    };

    Ok(Duration::from_millis(
        time_ms.to_u64().ok_or(ParseError::ExceedsLimits)?,
    ))
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("{0}")]
    InvalidDecimal(#[from] rust_decimal::Error),

    #[error("could not represent duration more accurate than ms")]
    ExceedsLimits,

    #[error("unsupported time format! should be ended with 's' or 'ms'.")]
    IllFormed,
}

pub fn register_sigusr2_decoration(app: WeakRef<Application>) {
    gtk::glib::unix_signal_add_local(libc::SIGUSR2, move || {
        let Some(app) = app.upgrade() else {
            return Continue(false);
        };

        let mut windows = app.windows();
        if windows.is_empty() {
            return Continue(true);
        }
        let window = windows.remove(0);

        let decorated = window.is_decorated();
        window.set_decorated(!decorated);

        Continue(true)
    });
}

pub fn register_action_hide_decoration(wind: &Window) {
    let action = SimpleAction::new("hide-decoration", None);
    let _wind = Window::downgrade(wind);
    action.connect_activate(move |_, _| {
        if let Some(wind) = _wind.upgrade() {
            wind.set_decorated(false);
        }
    });
    wind.add_action(&action);
}

pub fn register_action_switch_passthrough(wind: &Window) {
    let action = SimpleAction::new("switch-passthrough", None);
    let _wind = Window::downgrade(wind);
    action.connect_activate(move |_, _| {
        if let Some(wind) = _wind.upgrade() {
            let clickthrough = !wind.imp().clickthrough.get();
            wind.imp().clickthrough.set(clickthrough);
            set_click_pass_through(&wind, clickthrough);
        }
    });
    wind.add_action(&action);
}
