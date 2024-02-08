use std::path::PathBuf;

use crate::sync::lyric::fetch::tricks::get_lrc_path;

#[test]
fn shorter_extension() {
    let result = get_lrc_path(PathBuf::from("/xx/yy/test.ts"));
    assert_eq!(result, Some("/xx/yy/test.lrc".into()))
}

#[test]
fn longer_extension() {
    let result = get_lrc_path(PathBuf::from("/xx/yy/test.m3u8"));
    assert_eq!(result, Some("/xx/yy/test.lrc".into()))
}

#[test]
fn relative_path() {
    let result = get_lrc_path(PathBuf::from("../test.mp3"));
    assert_eq!(result, Some("../test.lrc".into()))
}

#[test]
fn non_ascii() {
    let result = get_lrc_path(PathBuf::from("/测试/中文/你好.诶木披散"));
    assert_eq!(result, Some("/测试/中文/你好.lrc".into()))
}

#[test]
fn empty_string() {
    let result = get_lrc_path(PathBuf::new());
    assert_eq!(result, None)
}
