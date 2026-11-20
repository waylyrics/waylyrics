//! A widget that renders text as vertical columns (tategaki) for the
//! vertical alignment modes.
//!
//! Normally text is shaped by Pango with `Gravity::East` so CJK glyphs stay
//! upright, punctuation gets vertical forms and Latin runs are rotated
//! sideways. If the active font has broken/zero vertical advances (`vmtx`),
//! the widget automatically falls back to stacking upright CJK glyphs with
//! `Gravity::South` while rotating Latin and bracket/dash runs 90° clockwise.
//! Drawing happens via `GtkSnapshot::append_layout` so text stays GPU-rendered.

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
        self.queue_draw();
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

/// One positioned piece inside a vertical column.
#[derive(Clone)]
pub(crate) struct Piece {
    pub(crate) layout: pango::Layout,
    /// Vertical offset from the top of the column (px).
    pub(crate) y: i32,
    /// Visual horizontal width of this piece after orientation (px).
    pub(crate) width: i32,
    /// If true, rotate 90° clockwise when drawing.
    pub(crate) rotated: bool,
}

/// One vertical column made of one or more positioned Pango layouts.
#[derive(Clone)]
pub(crate) struct Column {
    pub(crate) pieces: Vec<Piece>,
    #[cfg(test)]
    pub(crate) text: String,
}

#[derive(Clone)]
pub(crate) struct VerticalLayout {
    pub(crate) columns: Vec<Column>,
    pub(crate) column_width: i32,
    pub(crate) gap: i32,
    pub(crate) group_width: i32,
    pub(crate) group_height: i32,
}

pub(crate) fn setup_vertical_context(ctx: &pango::Context) {
    ctx.set_base_gravity(pango::Gravity::East);
    ctx.set_gravity_hint(pango::GravityHint::Natural);
}

fn is_rotatable_cjk_punct(ch: char) -> bool {
    matches!(
        ch,
        'ー' | '—'
            | '–'
            | '―'
            | '…'
            | '‥'
            | '～'
            | '〜'
            | '「'
            | '」'
            | '『'
            | '』'
            | '（'
            | '）'
            | '【'
            | '】'
            | '〈'
            | '〉'
            | '《'
            | '》'
            | '〔'
            | '〕'
            | '［'
            | '］'
            | '｛'
            | '｝'
            | '＝'
    )
}

fn is_upright_cjk(ch: char) -> bool {
    if is_rotatable_cjk_punct(ch) {
        return false;
    }
    matches!(
        ch as u32,
        0x3000..=0x303F
            | 0x3040..=0x309F
            | 0x30A0..=0x30FF
            | 0x31F0..=0x31FF
            | 0x3400..=0x4DBF
            | 0x4E00..=0x9FFF
            | 0xAC00..=0xD7AF
            | 0xF900..=0xFAFF
            | 0xFF01..=0xFF60
            | 0x20000..=0x2FA1F
    )
}

/// Detect whether `Gravity::East` produces valid vertical advances on this context.
/// Some CJK fonts ship broken/zero `vmtx` tables, causing all glyphs to stack
/// at the same coordinate.
fn vertical_metrics_working(vertical_ctx: &pango::Context) -> bool {
    let single = pango::Layout::new(vertical_ctx);
    single.set_text("国");
    let (_, single_rect) = single.pixel_extents();

    let sample = pango::Layout::new(vertical_ctx);
    sample.set_text("我们ああ");
    let (_, sample_rect) = sample.pixel_extents();

    let glyph_em = single_rect.height().max(1);
    single_rect.width() > 0 && sample_rect.width() >= glyph_em * 2
}

fn column_has_collapsed_cjk(text: &str, logical_width: i32, logical_height: i32) -> bool {
    let cjk_count = text.matches(is_upright_cjk).count();
    cjk_count >= 2 && logical_width < (cjk_count as i32) * (logical_height / 2).max(1)
}

/// Safe fallback layout using `Gravity::South` (horizontal metrics only):
/// CJK characters are stacked upright one by one, while Latin/bracket runs are
/// rotated 90° clockwise.
pub(crate) fn compute_fallback_vertical_layout(
    horiz_ctx: &pango::Context,
    text: &str,
    column_len: i32,
) -> Option<VerticalLayout> {
    if text.trim().is_empty() {
        return None;
    }

    let column_len = column_len.max(1);
    // Tokenize into upright CJK chars, explicit newlines, and sideways runs.
    enum Token {
        Newline,
        Upright(String),
        Sideways(String),
    }

    let mut tokens = Vec::new();
    let mut sideways = String::with_capacity(text.len());
    let flush_sideways = |buf: &mut String, out: &mut Vec<Token>| {
        if !buf.is_empty() {
            out.push(Token::Sideways(std::mem::take(buf)));
        }
    };

    for ch in text.chars() {
        if ch == '\n' {
            flush_sideways(&mut sideways, &mut tokens);
            tokens.push(Token::Newline);
        } else if is_upright_cjk(ch) {
            flush_sideways(&mut sideways, &mut tokens);
            tokens.push(Token::Upright(ch.to_string()));
        } else {
            sideways.push(ch);
        }
    }
    flush_sideways(&mut sideways, &mut tokens);

    let mut columns: Vec<Column> = Vec::new();
    let mut current_pieces: Vec<Piece> = Vec::new();
    #[cfg(test)]
    let mut current_text = String::with_capacity(text.len());
    let mut cur_y = 0;
    let mut column_width = 1;
    let mut group_height = 0;

    let push_column = |columns: &mut Vec<Column>,
                       pieces: &mut Vec<Piece>,
                       #[cfg(test)] col_text: &mut String,
                       cur_y: &mut i32,
                       group_height: &mut i32| {
        if !pieces.is_empty() {
            *group_height = (*group_height).max(*cur_y);
            columns.push(Column {
                pieces: std::mem::take(pieces),
                #[cfg(test)]
                text: std::mem::take(col_text),
            });
            *cur_y = 0;
        }
    };

    for token in tokens {
        match token {
            Token::Newline => {
                push_column(
                    &mut columns,
                    &mut current_pieces,
                    #[cfg(test)]
                    &mut current_text,
                    &mut cur_y,
                    &mut group_height,
                );
            }
            Token::Upright(ch) => {
                let layout = pango::Layout::new(horiz_ctx);
                layout.set_text(&ch);
                let (_, logical) = layout.pixel_extents();
                let advance = logical.height().max(1);
                let width = logical.width().max(1);
                if cur_y > 0 && cur_y + advance > column_len {
                    push_column(
                        &mut columns,
                        &mut current_pieces,
                        #[cfg(test)]
                        &mut current_text,
                        &mut cur_y,
                        &mut group_height,
                    );
                }
                column_width = column_width.max(width);
                current_pieces.push(Piece {
                    layout,
                    y: cur_y,
                    width,
                    rotated: false,
                });
                #[cfg(test)]
                current_text.push_str(&ch);
                cur_y += advance;
            }
            Token::Sideways(run) => {
                let mut remaining = run.as_str();
                while !remaining.is_empty() {
                    if cur_y == 0 {
                        remaining = remaining.trim_start();
                        if remaining.is_empty() {
                            break;
                        }
                    }
                    let available = (column_len - cur_y).max(1);
                    let probe = pango::Layout::new(horiz_ctx);
                    probe.set_text(remaining);
                    probe.set_wrap(pango::WrapMode::WordChar);
                    probe.set_width(available * pango::SCALE);

                    let first_line = probe.lines_readonly().into_iter().next();
                    let Some(line) = first_line else {
                        break;
                    };
                    let len = line.length() as usize;
                    let (chunk, rest) = if len == 0 {
                        // Force progress by taking at least one char
                        let first_len = remaining.chars().next().map_or(0, char::len_utf8);
                        (&remaining[..first_len], &remaining[first_len..])
                    } else {
                        (&remaining[..len], &remaining[len..])
                    };

                    let chunk_trimmed = if rest.is_empty() {
                        chunk
                    } else {
                        chunk.trim_end()
                    };
                    let layout = pango::Layout::new(horiz_ctx);
                    layout.set_text(chunk_trimmed);
                    let (_, logical) = layout.pixel_extents();
                    let advance = logical.width();
                    let width = logical.height().max(1);

                    // If this piece doesn't fit in the remaining space of a non-empty column,
                    // start a fresh column and retry.
                    if cur_y > 0 && cur_y + advance > column_len {
                        push_column(
                            &mut columns,
                            &mut current_pieces,
                            #[cfg(test)]
                            &mut current_text,
                            &mut cur_y,
                            &mut group_height,
                        );
                        continue;
                    }

                    if !chunk_trimmed.is_empty() {
                        column_width = column_width.max(width);
                        current_pieces.push(Piece {
                            layout,
                            y: cur_y,
                            width,
                            rotated: true,
                        });
                        #[cfg(test)]
                        current_text.push_str(chunk_trimmed);
                        cur_y += advance;
                    }
                    remaining = rest;
                    if !remaining.is_empty() {
                        push_column(
                            &mut columns,
                            &mut current_pieces,
                            #[cfg(test)]
                            &mut current_text,
                            &mut cur_y,
                            &mut group_height,
                        );
                    }
                }
            }
        }
    }

    push_column(
        &mut columns,
        &mut current_pieces,
        #[cfg(test)]
        &mut current_text,
        &mut cur_y,
        &mut group_height,
    );

    if columns.is_empty() {
        return None;
    }

    let gap = (column_width as f64 * 0.25).round() as i32;
    Some(VerticalLayout {
        group_width: group_width(columns.len(), column_width, gap),
        columns,
        column_width,
        gap,
        group_height,
    })
}

pub(crate) fn compute_vertical_layout(
    vertical_ctx: &pango::Context,
    horiz_ctx: &pango::Context,
    text: &str,
    column_len: i32,
) -> Option<VerticalLayout> {
    if text.trim().is_empty() {
        return None;
    }

    if !vertical_metrics_working(vertical_ctx) {
        return compute_fallback_vertical_layout(horiz_ctx, text, column_len);
    }

    let column_len = column_len.max(1);
    let master = pango::Layout::new(vertical_ctx);
    master.set_text(text);
    master.set_wrap(pango::WrapMode::WordChar);
    master.set_width(column_len * pango::SCALE);

    let mut columns = Vec::new();
    let mut column_width = 1;
    let mut group_height = 0;
    for line in master.lines_readonly() {
        let start = line.start_index() as usize;
        let end = start + line.length() as usize;
        let Some(part) = text.get(start..end) else {
            continue;
        };
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        let layout = pango::Layout::new(vertical_ctx);
        layout.set_text(part);
        let (_, logical) = layout.pixel_extents();
        if column_has_collapsed_cjk(part, logical.width(), logical.height()) {
            return compute_fallback_vertical_layout(horiz_ctx, text, column_len);
        }
        column_width = column_width.max(logical.height());
        group_height = group_height.max(logical.width());
        columns.push(Column {
            pieces: vec![Piece {
                layout,
                y: 0,
                width: logical.height(),
                rotated: true,
            }],
            #[cfg(test)]
            text: part.to_owned(),
        });
    }
    if columns.is_empty() {
        return None;
    }

    let gap = (column_width as f64 * 0.25).round() as i32;
    Some(VerticalLayout {
        group_width: group_width(columns.len(), column_width, gap),
        columns,
        column_width,
        gap,
        group_height,
    })
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
            (
                p.left() as i32,
                p.right() as i32,
                p.top() as i32,
                p.bottom() as i32,
            )
        }

        fn horizontal_context(&self) -> pango::Context {
            let ctx = self.obj().create_pango_context();
            ctx.set_base_gravity(pango::Gravity::South);
            ctx
        }

        /// A Pango context set up for vertical (top-to-bottom) text.
        /// `GravityHint::Natural` keeps CJK upright while letting Latin run sideways.
        fn vertical_context(&self) -> pango::Context {
            let ctx = self.obj().create_pango_context();
            setup_vertical_context(&ctx);
            ctx
        }

        /// Length of the text as a single, unwrapped column.
        fn natural_length(&self, text: &str) -> i32 {
            let vctx = self.vertical_context();
            let hctx = self.horizontal_context();
            compute_vertical_layout(&vctx, &hctx, text, 100_000)
                .map_or(0, |layout| layout.group_height)
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

            let vctx = self.vertical_context();
            let hctx = self.horizontal_context();
            let layout = compute_vertical_layout(&vctx, &hctx, text, column_len)?;
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
                let p = if orientation == gtk::Orientation::Horizontal {
                    l + r
                } else {
                    t + b
                };
                return (p, p, -1, -1);
            }

            if orientation == gtk::Orientation::Vertical {
                // min: one glyph; natural: everything in one column
                let min = self.natural_length("国") + t + b;
                let nat = self.natural_length(&text) + t + b;
                (min, nat.max(min), -1, -1)
            } else {
                let column_len = if for_size > 0 {
                    for_size - t - b
                } else {
                    self.natural_length(&text)
                };
                let w = self
                    .layout_for(&text, column_len)
                    .map_or(0, |l| l.group_width)
                    + l
                    + r;
                (w, w, -1, -1)
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
                for piece in &column.pieces {
                    snapshot.save();
                    if piece.rotated {
                        // rotate 90° clockwise: layout's top edge ends up on the right
                        let tx = x + (layout.column_width + piece.width) / 2;
                        snapshot.translate(&graphene::Point::new(
                            tx as f32,
                            (origin_y + piece.y) as f32,
                        ));
                        snapshot.rotate(90.0);
                    } else {
                        let tx = x + (layout.column_width - piece.width) / 2;
                        snapshot.translate(&graphene::Point::new(
                            tx as f32,
                            (origin_y + piece.y) as f32,
                        ));
                    }
                    snapshot.append_layout(&piece.layout, &color);
                    snapshot.restore();
                }
            }
        }
    }
}
