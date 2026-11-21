use crate::app::{
    column_offset, compute_fallback_vertical_layout, compute_vertical_layout, group_offset,
    group_width, setup_vertical_context,
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

fn test_contexts() -> Option<(pango::Context, pango::Context)> {
    if gtk::init().is_err() {
        return None;
    }
    let label = gtk::Label::new(None);
    let desc = pango::FontDescription::from_string("Sans 16");

    let vctx = label.create_pango_context();
    vctx.set_font_description(&desc);
    setup_vertical_context(&vctx);

    let hctx = label.create_pango_context();
    hctx.set_font_description(&desc);
    hctx.set_base_gravity(pango::Gravity::South);

    Some((vctx, hctx))
}

#[test]
fn pango_vertical_layout() {
    let Some((vctx, hctx)) = test_contexts() else {
        return;
    };

    // empty / whitespace produces no columns
    assert!(compute_vertical_layout(&vctx, &hctx, "", 300).is_none());
    assert!(compute_vertical_layout(&vctx, &hctx, "   \n\t ", 300).is_none());

    // long Japanese line wraps into multiple columns within column_len
    let text = "こんにちは、世界。今日もいい天気ですね。";
    let single = compute_vertical_layout(&vctx, &hctx, text, 2000).unwrap();
    assert_eq!(single.columns.len(), 1);

    let max_len = 160;
    let wrapped = compute_vertical_layout(&vctx, &hctx, text, max_len).unwrap();
    assert!(wrapped.columns.len() >= 2);
    assert!(wrapped.group_width > single.group_width);
    assert!(wrapped.group_height <= max_len);

    // mixed Latin + CJK preserves whole English words and assigns South vs East gravity
    let mixed = compute_vertical_layout(&vctx, &hctx, "Hello 世界", 140).unwrap();
    assert!(
        mixed.columns.iter().any(|c| c.text.contains("Hello")),
        "expected 'Hello' not to split mid-word: {:?}",
        mixed.columns.iter().map(|c| &c.text).collect::<Vec<_>>()
    );

    let mut gravities = Vec::new();
    for col in &mixed.columns {
        for piece in &col.pieces {
            let mut iter = piece.layout.iter();
            loop {
                if let Some(run) = iter.run_readonly() {
                    gravities.push(run.item().analysis().gravity());
                }
                if !iter.next_run() {
                    break;
                }
            }
        }
    }
    assert!(gravities.contains(&pango::Gravity::South));
    assert!(gravities.contains(&pango::Gravity::East));

    // fallback path (for fonts with broken/zero vmtx) stacks CJK upright without collapsing
    let fallback_cjk = compute_fallback_vertical_layout(&hctx, "我们在魔法时刻", 2000).unwrap();
    assert_eq!(fallback_cjk.columns.len(), 1);
    assert_eq!(fallback_cjk.columns[0].pieces.len(), 7);
    assert!(fallback_cjk.columns[0].pieces.iter().all(|p| !p.rotated));
    assert!(fallback_cjk.group_height >= fallback_cjk.column_width * 4);

    // fallback wraps long CJK lines into multiple columns
    let fallback_wrap = compute_fallback_vertical_layout(&hctx, "我们在魔法时刻", 100).unwrap();
    assert!(fallback_wrap.columns.len() >= 2);
    assert!(fallback_wrap.group_height <= 100);

    // fallback rotates Latin and CJK brackets sideways while keeping CJK upright
    let fallback_mixed = compute_fallback_vertical_layout(&hctx, "「Hello」世界", 2000).unwrap();
    assert_eq!(fallback_mixed.columns.len(), 1);
    let pieces = &fallback_mixed.columns[0].pieces;
    assert!(pieces.iter().any(|p| p.rotated));
    assert!(pieces.iter().any(|p| !p.rotated));
}
