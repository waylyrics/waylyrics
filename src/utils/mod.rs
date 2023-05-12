use std::time::Duration;

pub fn parse_time(time: &str) -> Duration {
    use rust_decimal::prelude::*;
    use rust_decimal_macros::dec;

    let time_ms = if time.ends_with("ms") {
        let sec = time.trim_end_matches("ms");
        Decimal::from_str_exact(sec).unwrap()
    } else if time.ends_with('s') {
        let milli_sec = time.trim_end_matches('s');
        Decimal::from_str_exact(milli_sec).unwrap() * dec!(1000)
    } else {
        panic!("unsupported time format! should be ended with 's' or 'ms'.")
    };
    Duration::from_millis(
        time_ms
            .to_u64()
            .expect("could not represent duration more accurate than ms"),
    )
}
