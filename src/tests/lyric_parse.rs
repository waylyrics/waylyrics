#[cfg(test)]
mod lrc {
    use std::time::Duration;

    use crate::lyric_providers::{utils::lrc_iter, LyricLine, LyricLineOwned};
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
}
