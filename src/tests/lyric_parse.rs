#[cfg(test)]
mod lrc {
    use std::time::Duration;

    use crate::{
        lyric_providers::{utils::lrc_iter, LyricLine, LyricLineOwned},
        sync::{extract_translated_lyric, filter_original_lyric},
    };
    use anyhow::Result;

    #[test]
    fn simple_lrc_sorted_with_time() -> Result<()> {
        let lrc = r#"
[00:01.05] Hi friend...
[45:05.64] Can you hear me?...
"#;
        let mut lyrics = lrc_iter(lrc.lines())?.into_iter();

        assert_eq!(
            lyrics.next().map(|l| LyricLine::<'_>::into_owned(l)),
            Some(LyricLineOwned {
                text: "Hi friend...".into(),
                start_time: Duration::from_millis(1000 + 50),
            })
        );
        assert_eq!(
            lyrics.next().map(|l| LyricLine::<'_>::into_owned(l)),
            Some(LyricLineOwned {
                text: "Can you hear me?...".into(),
                start_time: Duration::from_millis(640 + (45 * 60 + 5) * 1000),
            })
        );

        Ok(())
    }

    #[test]
    fn simple_lrc_unsorted() -> Result<()> {
        let lrc = r#"
[45:05.64] Can you hear me?...
[00:01.05] Hi friend...
"#;
        let mut lyrics = lrc_iter(lrc.lines())?.into_iter();

        assert_eq!(
            lyrics.next().map(|l| LyricLine::<'_>::into_owned(l)),
            Some(LyricLineOwned {
                text: "Hi friend...".into(),
                start_time: Duration::from_millis(1000 + 50),
            })
        );
        assert_eq!(
            lyrics.next().map(|l| LyricLine::<'_>::into_owned(l)),
            Some(LyricLineOwned {
                text: "Can you hear me?...".into(),
                start_time: Duration::from_millis(640 + (45 * 60 + 5) * 1000),
            })
        );

        Ok(())
    }

    #[test]
    fn ill_formatted_time() -> Result<()> {
        let lrc = r#"
[00:01:05] Hi friend...
"#;
        let mut lyrics = lrc_iter(lrc.lines())?.into_iter();

        assert_eq!(
            lyrics.next().map(|l| LyricLine::<'_>::into_owned(l)),
            Some(LyricLineOwned {
                text: "Hi friend...".into(),
                start_time: Duration::from_millis(1000 + 50),
            })
        );
        Ok(())
    }

    #[test]
    fn ill_formed_unexpected_line() -> Result<()> {
        let lrc = r#"
My first LRC Lyric!
Author: poly000
[00:01.05] Hi friend...
"#;
        let mut lyrics = lrc_iter(lrc.lines())?.into_iter();

        assert_eq!(
            lyrics.next().map(|l| LyricLine::<'_>::into_owned(l)),
            Some(LyricLineOwned {
                text: "Hi friend...".into(),
                start_time: Duration::from_millis(1000 + 50),
            })
        );
        Ok(())
    }

    #[test]
    fn ill_formed_tag() -> Result<()> {
        let lrc = r#"
[SAYONARA]
"#;
        let lyrics = lrc_iter(lrc.lines())?;
        assert!(lyrics.is_empty());
        Ok(())
    }

    #[test]
    fn elided_text() -> Result<()> {
        let lrc = r#"
[00:01.014][00:02.062] Hi friend...
"#;
        let mut lyrics = lrc_iter(lrc.lines())?.into_iter();

        assert_eq!(
            lyrics.next().map(|l| LyricLine::<'_>::into_owned(l)),
            Some(LyricLineOwned {
                text: "Hi friend...".into(),
                start_time: Duration::from_millis(1014),
            })
        );

        assert_eq!(
            lyrics.next().map(|l| LyricLine::<'_>::into_owned(l)),
            Some(LyricLineOwned {
                text: "Hi friend...".into(),
                start_time: Duration::from_millis(2062),
            })
        );

        Ok(())
    }

    #[test]
    fn empty_texts() -> Result<()> {
        let lrc = r#"
[00:01.014] Hi friend...
[00:02.062]
"#;
        let mut lyrics = lrc_iter(lrc.lines())?.into_iter();

        assert_eq!(
            lyrics.next().map(|l| LyricLine::<'_>::into_owned(l)),
            Some(LyricLineOwned {
                text: "Hi friend...".into(),
                start_time: Duration::from_millis(1014),
            })
        );

        assert_eq!(
            lyrics.next().map(|l| LyricLine::<'_>::into_owned(l)),
            Some(LyricLineOwned {
                text: "".into(),
                start_time: Duration::from_millis(2062),
            })
        );

        Ok(())
    }

    #[test]
    fn extract_lyric() -> Result<()> {
        let lrc = r#"
[00:01.77]Please don't say "You are lazy"
[00:01.77]请不要说“你很懒”
[00:04.38]だって本当はcrazy"#;
        let lyrics = lrc_iter(lrc.lines())?
            .into_iter()
            .map(LyricLine::into_owned)
            .collect::<Vec<_>>();
        let tlyric = extract_translated_lyric(&lyrics);
        let olyric = filter_original_lyric(lyrics, &tlyric);
        assert_eq!(
            tlyric,
            vec![LyricLineOwned {
                text: "请不要说“你很懒”".into(),
                start_time: Duration::from_millis(1770)
            },]
        );
        assert_eq!(
            olyric,
            vec![
                LyricLineOwned {
                    text: "Please don't say \"You are lazy\"".into(),
                    start_time: Duration::from_millis(1770)
                },
                LyricLineOwned {
                    text: "だって本当はcrazy".into(),
                    start_time: Duration::from_millis(4380)
                }
            ]
        );
        Ok(())
    }
}
