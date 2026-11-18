use crate::app::{column_offset, group_offset, group_width};
use crate::config::Align;

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
