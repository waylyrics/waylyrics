use crate::sync::utils::distance;

fn str_distance<A: AsRef<str>, B: AsRef<str>>(a: A, b: B) -> f64 {
    let a = a.as_ref().chars().collect::<Vec<_>>();
    let b = b.as_ref().chars().collect::<Vec<_>>();

    distance(&a, &b)
}

#[test]
fn unicode_distance() {
    assert_eq!(str_distance("你好，世界", "你好，世界"), 1.0);

    // Inserting one character should be closer than replacing multiple characters.
    assert!(str_distance("你好世界", "你好，世界") > str_distance("你好世界", "我的世界"));

    // One substitution should be closer than multiple substitutions.
    assert!(str_distance("你好世界", "你好世人") > str_distance("你好世界", "你我世人"));

    // A single insertion should be closer than multiple insertions.
    assert!(str_distance("abc", "abxc") > str_distance("abc", "axyzc"));

    // A single substitution should be closer than two substitutions.
    assert!(str_distance("hello", "hallo") > str_distance("hello", "hullo!"));

    // Strings with a common prefix should be more similar.
    assert!(
        str_distance("Hello world", "Hello World") > str_distance("Hello world", "Goodbye world")
    );

    // A small modification should be closer than a completely different sentence.
    assert!(
        str_distance("The quick brown fox", "The quick brown box")
            > str_distance("The quick brown fox", "A slow green dog")
    );

    // A single changed character should be closer than a completely different sentence.
    assert!(
        str_distance("今天天气很好", "今天天气很冷") > str_distance("今天天气很好", "昨天我吃苹果")
    );

    // A single changed word should be closer than a completely different phrase.
    assert!(
        str_distance("初音ミクの消失", "初音ミクの暴走")
            > str_distance("初音ミクの消失", "東方紅魔郷")
    );

    // Similar mixed Chinese and Japanese text should be closer than unrelated text.
    assert!(
        str_distance("初音未来的消失", "初音ミクの消失")
            > str_distance("初音未来的消失", "完全不同的歌曲")
    );
}
