use std::{error::Error, time::Duration};

pub fn parse_time(time: &str) -> Result<Duration, Box<dyn Error>> {
    use rust_decimal::prelude::*;
    use rust_decimal_macros::dec;

    let time_ms = if time.ends_with("ms") {
        let sec = time.trim_end_matches("ms");
        Decimal::from_str_exact(sec)?
    } else if time.ends_with('s') {
        let milli_sec = time.trim_end_matches('s');
        Decimal::from_str_exact(milli_sec)? * dec!(1000)
    } else {
        return Err("unsupported time format! should be ended with 's' or 'ms'.".into());
    };

    Ok(Duration::from_millis(time_ms.to_u64().ok_or(
        "could not represent duration more accurate than ms",
    )?))
}
