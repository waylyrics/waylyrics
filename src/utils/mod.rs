use anyhow::Result;
use gtk::{Application, glib::WeakRef, prelude::*};
use std::time::Duration;

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
