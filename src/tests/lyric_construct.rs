#[test]
#[cfg(feature = "export-lyric")]
fn make_lrc_line() {
    use std::time::Duration;

    use crate::sync::actions::utils::make_lrc_line;

    assert_eq!(
        make_lrc_line("你好", Duration::from_secs(61)),
        "[01:01.000]你好".to_string()
    );
    assert_eq!(
        make_lrc_line("世界", Duration::from_millis(63570)),
        "[01:03.570]世界".to_string()
    );
    assert_eq!(
        make_lrc_line("遗忘我", Duration::from_millis(123570)),
        "[02:03.570]遗忘我".to_string()
    );
}
