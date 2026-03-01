use std::borrow::Cow;

use crate::lyric_providers::strip_extended_timestamps;

#[test]
fn test_strip_extended_timestamps() {
    let lrc = "[00:12.34]Hello <01:23.456>world!";
    let stripped = strip_extended_timestamps(lrc);
    assert_eq!(stripped, "[00:12.34]Hello world!");

    // 没有尖括号，直接借用
    let lrc2 = "Hello world!";
    let stripped2 = strip_extended_timestamps(lrc2);
    assert!(matches!(stripped2, Cow::Borrowed(_)));
}
