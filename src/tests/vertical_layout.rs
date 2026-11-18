use crate::app::{
    column_offset, compute_vertical_layout, group_offset, group_width, setup_vertical_context,
};
use crate::config::Align;
use gtk::pango;
use gtk::prelude::*;

#[test]
fn column_group_width() {
    assert_eq!(group_width(0, 40, 8), 0);
    assert_eq!(group_width(1, 40, 8), 40);
    assert_eq!(group_width(3, 40, 8), 136);
}

#[test]
fn ltr_column_offsets() {
    let step = 48;
    assert_eq!(column_offset(0, 3, step, false), 0);
    assert_eq!(column_offset(1, 3, step, false), 48);
    assert_eq!(column_offset(2, 3, step, false), 96);
}

#[test]
fn rtl_column_offsets() {
    let step = 48;
    assert_eq!(column_offset(0, 3, step, true), 96);
    assert_eq!(column_offset(1, 3, step, true), 48);
    assert_eq!(column_offset(2, 3, step, true), 0);
}

#[test]
fn ltr_group_starts_left() {
    assert_eq!(group_offset(300, 136, false), 0);
}

#[test]
fn rtl_group_ends_right() {
    assert_eq!(group_offset(300, 136, true), 164);
}

#[test]
fn vertical_align_storage_values() {
    assert_eq!(Align::VerticalLtr.to_string(), "vertical-ltr");
    assert_eq!(Align::VerticalRtl.to_string(), "vertical-rtl");
    assert_eq!(
        "vertical-ltr".parse::<Align>().unwrap(),
        Align::VerticalLtr
    );
    assert_eq!(
        "vertical-rtl".parse::<Align>().unwrap(),
        Align::VerticalRtl
    );
}

#[test]
fn vertical_align_menu_labels() {
    assert_eq!(Align::VerticalLtr.display_name(), "Vertical LTR");
    assert_eq!(Align::VerticalRtl.display_name(), "Vertical RTL");
}

#[test]
fn vertical_align_flags() {
    assert!(Align::VerticalLtr.is_vertical());
    assert!(Align::VerticalRtl.is_vertical());
    assert!(Align::VerticalRtl.is_rtl());
    assert!(!Align::VerticalLtr.is_rtl());
    assert!(!Align::Start.is_vertical());
}

fn test_vertical_context() -> Option<pango::Context> {
    if gtk::init().is_err() {
        return None;
    }
    let ctx = gtk::Label::new(None).create_pango_context();
    ctx.set_font_description(&pango::FontDescription::from_string("Sans 16"));
    setup_vertical_context(&ctx);
    Some(ctx)
}

#[test]
fn pango_vertical_layout() {
    let Some(ctx) = test_vertical_context() else {
        return;
    };

    // empty / whitespace produces no columns
    assert!(compute_vertical_layout(&ctx, "", 300).is_none());
    assert!(compute_vertical_layout(&ctx, "   \n\t ", 300).is_none());

    // long Japanese line wraps into multiple columns within column_len
    let text = "こんにちは、世界。今日もいい天気ですね。";
    let single = compute_vertical_layout(&ctx, text, 2000).unwrap();
    assert_eq!(single.columns.len(), 1);

    let max_len = 160;
    let wrapped = compute_vertical_layout(&ctx, text, max_len).unwrap();
    assert!(wrapped.columns.len() >= 2);
    assert!(wrapped.group_width > single.group_width);
    assert!(wrapped.group_height <= max_len);

    // mixed Latin + CJK preserves whole English words and assigns South vs East gravity
    let mixed = compute_vertical_layout(&ctx, "Hello 世界", 140).unwrap();
    assert!(
        mixed.columns.iter().any(|c| c.text.contains("Hello")),
        "expected 'Hello' not to split mid-word: {:?}",
        mixed.columns.iter().map(|c| &c.text).collect::<Vec<_>>()
    );

    let mut gravities = Vec::new();
    for col in &mixed.columns {
        let mut iter = col.layout.iter();
        loop {
            if let Some(run) = iter.run_readonly() {
                gravities.push(run.item().analysis().gravity());
            }
            if !iter.next_run() {
                break;
            }
        }
    }
    assert!(gravities.contains(&pango::Gravity::South));
    assert!(gravities.contains(&pango::Gravity::East));
}
