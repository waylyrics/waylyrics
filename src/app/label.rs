//! A widget that renders text as vertical columns (tategaki) for the
//! vertical alignment modes.
//!
//! Text is shaped by Pango with `Gravity::East` so CJK glyphs stay upright,
//! punctuation gets vertical forms and Latin runs are rotated sideways.
//! Every line of the wrapped layout becomes one column; drawing happens via
//! `GtkSnapshot::append_layout` so the text stays GPU-rendered and crisp.

use std::cell::{Cell, RefCell};

use gtk::glib;
use gtk::graphene;
use gtk::pango;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct VerticalLabel(ObjectSubclass<imp::VerticalLabel>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl VerticalLabel {
    pub fn new(name: &str) -> Self {
        let label: Self = glib::Object::builder().build();
        label.set_widget_name(name);
        label.add_css_class("vertical");
        label
    }

    pub fn set_text(&self, text: &str) {
        let imp = self.imp();
        if imp.text.borrow().as_str() == text {
            return;
        }
        *imp.text.borrow_mut() = text.to_owned();
        imp.cache.borrow_mut().take();
        self.queue_resize();
    }

    pub fn text(&self) -> String {
        self.imp().text.borrow().clone()
    }

    pub fn set_rtl(&self, rtl: bool) {
        let imp = self.imp();
        if imp.rtl.get() != rtl {
            imp.rtl.set(rtl);
            self.queue_draw();
        }
    }
}

pub(crate) fn group_width(count: usize, column_width: i32, gap: i32) -> i32 {
    if count == 0 {
        0
    } else {
        count as i32 * column_width + (count as i32 - 1) * gap
    }
}

pub(crate) fn group_offset(content_width: i32, group_width: i32, rtl: bool) -> i32 {
    if rtl {
        (content_width - group_width).max(0)
    } else {
        0
    }
}

pub(crate) fn column_offset(index: usize, count: usize, step: i32, rtl: bool) -> i32 {
    let column = if rtl { count - 1 - index } else { index };
    column as i32 * step
}

/// One shaped column. `layout` is horizontal in its own space and gets
/// rotated by 90° when drawn.
#[derive(Clone)]
struct Column {
    layout: pango::Layout,
}

#[derive(Clone)]
struct VerticalLayout {
    columns: Vec<Column>,
    column_width: i32,
    gap: i32,
    group_width: i32,
    group_height: i32,
}

type CacheKey = (String, i32, String, i32);

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct VerticalLabel {
        pub(super) text: RefCell<String>,
        pub(super) rtl: Cell<bool>,
        pub(super) cache: RefCell<Option<(CacheKey, VerticalLayout)>>,
    }

    impl VerticalLabel {
        fn padding(&self) -> (i32, i32, i32, i32) {
            #[allow(deprecated)]
            let p = self.obj().style_context().padding();
            (p.left() as i32, p.right() as i32, p.top() as i32, p.bottom() as i32)
        }

        /// A Pango context set up for vertical (top-to-bottom) text.
        /// `GravityHint::Natural` keeps CJK upright while letting Latin run sideways.
        fn vertical_context(&self) -> pango::Context {
            let ctx = self.obj().create_pango_context();
            ctx.set_base_gravity(pango::Gravity::East);
            ctx.set_gravity_hint(pango::GravityHint::Natural);
            ctx
        }

        fn new_layout(&self, ctx: &pango::Context, text: &str) -> pango::Layout {
            let layout = pango::Layout::new(ctx);
            layout.set_text(text);
            layout
        }

        /// Length of the text as a single, unwrapped column.
        fn natural_length(&self, text: &str) -> i32 {
            let ctx = self.vertical_context();
            self.new_layout(&ctx, text).pixel_extents().1.width()
        }

        fn font_key(&self) -> String {
            self.obj()
                .pango_context()
                .font_description()
                .map(|f| f.to_string())
                .unwrap_or_default()
        }

        pub(super) fn layout_for(&self, text: &str, column_len: i32) -> Option<VerticalLayout> {
            if text.trim().is_empty() {
                return None;
            }
            let column_len = column_len.max(1);
            let key: CacheKey = (
                text.to_owned(),
                column_len,
                self.font_key(),
                self.obj().scale_factor(),
            );
            if let Some((k, l)) = self.cache.borrow().as_ref() {
                if *k == key {
                    return Some(l.clone());
                }
            }

            let ctx = self.vertical_context();
            let master = self.new_layout(&ctx, text);
            master.set_wrap(pango::WrapMode::WordChar);
            master.set_width(column_len * pango::SCALE);

            let mut columns = Vec::new();
            let mut column_width = 1;
            let mut group_height = 0;
            for line in master.lines_readonly() {
                let start = line.start_index() as usize;
                let end = start + line.length() as usize;
                let Some(part) = text.get(start..end) else { continue };
                let part = part.trim();
                if part.is_empty() {
                    continue;
                }
                let layout = self.new_layout(&ctx, part);
                let (_, logical) = layout.pixel_extents();
                column_width = column_width.max(logical.height());
                group_height = group_height.max(logical.width());
                columns.push(Column { layout });
            }
            if columns.is_empty() {
                return None;
            }

            let gap = (column_width as f64 * 0.25).round() as i32;
            let layout = VerticalLayout {
                group_width: group_width(columns.len(), column_width, gap),
                columns,
                column_width,
                gap,
                group_height,
            };
            *self.cache.borrow_mut() = Some((key, layout.clone()));
            Some(layout)
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for VerticalLabel {
        const NAME: &'static str = "WaylyricsVerticalLabel";
        type Type = super::VerticalLabel;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_css_name("label");
        }
    }

    impl ObjectImpl for VerticalLabel {}

    impl WidgetImpl for VerticalLabel {
        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::WidthForHeight
        }

        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            let (l, r, t, b) = self.padding();
            let text = self.text.borrow().clone();
            if text.trim().is_empty() {
                let p = if orientation == gtk::Orientation::Horizontal { l + r } else { t + b };
                return (p, p, -1, -1);
            }

            match orientation {
                gtk::Orientation::Vertical => {
                    // min: one glyph; natural: everything in one column
                    let min = self.natural_length("国") + t + b;
                    let nat = self.natural_length(&text) + t + b;
                    (min, nat.max(min), -1, -1)
                }
                _ => {
                    let column_len = if for_size > 0 {
                        for_size - t - b
                    } else {
                        self.natural_length(&text)
                    };
                    let w = self
                        .layout_for(&text, column_len)
                        .map(|l| l.group_width)
                        .unwrap_or(0)
                        + l
                        + r;
                    (w, w, -1, -1)
                }
            }
        }

        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let obj = self.obj();
            let (width, height) = (obj.width(), obj.height());
            if width <= 0 || height <= 0 {
                return;
            }
            let (l, r, t, b) = self.padding();
            let content_width = (width - l - r).max(0);
            let content_height = (height - t - b).max(1);
            let text = self.text.borrow().clone();
            let Some(layout) = self.layout_for(&text, content_height) else {
                return;
            };

            let rtl = self.rtl.get();
            let origin_x = l + group_offset(content_width, layout.group_width, rtl);
            let origin_y = t + ((content_height - layout.group_height) / 2).max(0);
            let color = obj.color();
            let count = layout.columns.len();
            let step = layout.column_width + layout.gap;

            for (index, column) in layout.columns.iter().enumerate() {
                let x = origin_x + column_offset(index, count, step, rtl);
                snapshot.save();
                // rotate 90° clockwise: layout's top edge ends up on the right
                snapshot.translate(&graphene::Point::new(
                    (x + layout.column_width) as f32,
                    origin_y as f32,
                ));
                snapshot.rotate(90.0);
                snapshot.append_layout(&column.layout, &color);
                snapshot.restore();
            }
        }
    }
}
