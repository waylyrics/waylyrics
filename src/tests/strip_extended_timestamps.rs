use std::borrow::Cow;

use crate::lyric_providers::strip_extended_timestamps;

#[test]
fn test_direct_borrow() {
    // No angle brackets, should be borrowed directly
    let lrc = "[00:00.00]Hello world!";
    let stripped = strip_extended_timestamps(lrc);
    assert!(matches!(stripped, Cow::Borrowed(_)));
}

#[test]
fn test_multiple_tag() {
    let lrc = "[00:12.34]I <01:23.456>love <01:24.123>Rust!";
    let stripped = strip_extended_timestamps(lrc);
    assert_eq!(stripped, "[00:12.34]I love Rust!");
}

#[test]
fn test_ill_form_ignored() {
    let lrc = "[00:12.34]Hello <01:23:456>world!";
    let stripped = strip_extended_timestamps(lrc);
    assert_eq!(stripped, "[00:12.34]Hello <01:23:456>world!");
}

#[test]
fn test_not_tag_ignored() {
    let lrc = "[00:12.34]<FM> 纯音乐";
    let stripped = strip_extended_timestamps(lrc);
    assert_eq!(stripped, "[00:12.34]<FM> 纯音乐");
}

#[test]
fn test_all_possible_case() {
    let answer = "[00:00.00]你好";
    for min in (0..100)
        .map(|num| {
            if num < 10 {
                vec![format!("{num}"), format!("{num:02}")]
            } else {
                vec![format!("{num}")]
            }
        })
        .flatten()
    {
        for sec in (0..60)
            .map(|num| {
                if num < 10 {
                    vec![format!("{num}"), format!("{num:02}")]
                } else {
                    vec![format!("{num}")]
                }
            })
            .flatten()
        {
            for subsec in 1..1000 {
                if subsec < 10 {
                    let lrc_line1 = format!("[00:00.00]你<{min}:{sec}.{subsec}>好");
                    assert_eq!(strip_extended_timestamps(&lrc_line1), answer,);
                }
                if subsec < 100 {
                    let lrc_line2 = format!("[00:00.00]你<{min}:{sec}.{subsec:02}>好");
                    assert_eq!(strip_extended_timestamps(&lrc_line2), answer,);
                }
                let lrc_line3 = format!("[00:00.00]你<{min}:{sec}.{subsec:03}>好");
                assert_eq!(strip_extended_timestamps(&lrc_line3), answer,);
            }
        }
    }
}
