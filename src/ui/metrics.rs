// SPDX-License-Identifier: GPL-3.0-or-later

use crate::core::appearance::{Appearance, Orientation};
use crate::core::style::Style;

const GAP: f64 = 6.0;
const ROW_VISUAL_SCALE: f64 = 0.5;

#[derive(Clone, Copy, Debug)]
pub struct ItemMetrics {
    pub width: f64,
    pub height: f64,
    pub icon: Option<(f64, f64, f64)>,
    pub thumbnail: Option<(f64, f64, f64, f64)>,
    pub label: (f64, f64, f64, f64),
    pub label_lines: isize,
    pub label_centered: bool,
    pub label_middle: bool,
}

struct Base {
    scale: f64,
    pad: f64,
    line: f64,
    lines: f64,
    icon_scale: f64,
}

impl Base {
    fn new(a: &Appearance, zoom: f64) -> Self {
        Self {
            scale: a.size.scale() * zoom,
            pad: a.item_padding,
            line: (a.font_size * 1.3).ceil(),
            lines: a.title_lines.clamp(1, 3) as f64,
            icon_scale: a.icon_size.scale(),
        }
    }

    /// The style's default icon size multiplied by the size settings.
    fn icon(&self, base: f64) -> f64 {
        (base * self.scale * self.icon_scale).round()
    }

    fn thumbnail(&self, k: f64) -> (f64, f64) {
        ((204.0 * self.scale * k).round(), (132.0 * self.scale * k).round())
    }

    fn label_height(&self) -> f64 {
        self.lines * self.line
    }
}

/// `zoom` shrinks the items when the panel does not fit on the screen (1.0 = normal).
pub fn metrics(style: Style, a: &Appearance, zoom: f64) -> ItemMetrics {
    let b = Base::new(a, zoom);
    match (style, a.orientation) {
        (Style::List, _) => row(&b, Visual::Icon(b.icon(24.0))),
        (_, Orientation::Vertical) => row(&b, row_visual(style, &b)),
        (Style::Icons, Orientation::Horizontal) => stacked_icon(&b),
        (_, Orientation::Horizontal) => stacked_preview(&b, style == Style::PreviewsWithIcon),
    }
}

enum Visual {
    Icon(f64),
    Thumbnail { width: f64, height: f64, corner_icon: Option<f64> },
}

fn row_visual(style: Style, b: &Base) -> Visual {
    let (width, height) = b.thumbnail(ROW_VISUAL_SCALE);
    match style {
        Style::Previews => Visual::Thumbnail { width, height, corner_icon: None },
        Style::PreviewsWithIcon => {
            let corner = (b.icon(36.0) * ROW_VISUAL_SCALE).min(height * 0.6);
            Visual::Thumbnail { width, height, corner_icon: Some(corner) }
        }
        _ => Visual::Icon((b.icon(64.0) * ROW_VISUAL_SCALE).round()),
    }
}

/// Horizontal, icon style: large icon on top, centered title below.
fn stacked_icon(b: &Base) -> ItemMetrics {
    let icon = b.icon(64.0);
    let label_h = b.label_height();
    let width = icon.max(96.0 * b.scale) + 2.0 * b.pad;
    ItemMetrics {
        width,
        height: b.pad + label_h + GAP + icon + b.pad,
        icon: Some(((width - icon) / 2.0, b.pad + label_h + GAP, icon)),
        thumbnail: None,
        label: (b.pad, b.pad, width - 2.0 * b.pad, label_h),
        label_lines: b.lines as isize,
        label_centered: true,
        label_middle: false,
    }
}

/// Horizontal, preview styles: preview on top, title below, optionally an icon in the preview's bottom-right corner.
fn stacked_preview(b: &Base, corner_icon: bool) -> ItemMetrics {
    let (tw, th) = b.thumbnail(1.0);
    let label_h = b.label_height();
    let thumb_y = b.pad + label_h + GAP;
    let icon = corner_icon.then(|| corner(b.icon(36.0).min(th * 0.6), b.pad, thumb_y, tw));
    ItemMetrics {
        width: tw + 2.0 * b.pad,
        height: thumb_y + th + b.pad,
        icon,
        thumbnail: Some((b.pad, thumb_y, tw, th)),
        label: (b.pad, b.pad, tw, label_h),
        label_lines: b.lines as isize,
        label_centered: false,
        label_middle: false,
    }
}

/// Row layout: visual on the left, vertically centered, long title on its right.
fn row(b: &Base, visual: Visual) -> ItemMetrics {
    let (vw, vh) = match visual {
        Visual::Icon(size) => (size, size),
        Visual::Thumbnail { width, height, .. } => (width, height),
    };
    let label_w = (380.0 * b.scale).round();
    let label_h = b.label_height();
    let height = vh.max(label_h) + 2.0 * b.pad;
    let vy = (height - vh) / 2.0;
    let label_x = b.pad + vw + GAP + 4.0;
    let (icon, thumbnail) = match visual {
        Visual::Icon(size) => (Some((b.pad, vy, size)), None),
        Visual::Thumbnail { corner_icon, .. } => {
            (corner_icon.map(|s| corner(s, b.pad, vy, vw)), Some((b.pad, vy, vw, vh)))
        }
    };
    ItemMetrics {
        width: label_x + label_w + b.pad,
        height,
        icon,
        thumbnail,
        label: (label_x, (height - label_h) / 2.0, label_w, label_h),
        label_lines: b.lines as isize,
        label_centered: false,
        label_middle: true,
    }
}

/// Icon placed slightly inside the bottom-right corner of the preview area.
fn corner(size: f64, thumb_x: f64, thumb_y: f64, thumb_w: f64) -> (f64, f64, f64) {
    (thumb_x + thumb_w - size - 4.0, thumb_y + 4.0, size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::appearance::IconSize;

    fn vertical() -> Appearance {
        Appearance { orientation: Orientation::Vertical, ..Appearance::default() }
    }

    #[test]
    fn corner_icon_sits_inside_thumbnail_bottom_right() {
        let m = metrics(Style::PreviewsWithIcon, &Appearance::default(), 1.0);
        let (ix, iy, size) = m.icon.unwrap();
        let (tx, ty, tw, _) = m.thumbnail.unwrap();
        assert!(
            ix + size <= tx + tw && ix > tx + tw / 2.0,
            "the corner icon sits inside the right half of the thumbnail"
        );
        assert!(iy >= ty, "the corner icon does not start above the thumbnail");
    }

    #[test]
    fn plain_previews_have_no_icon() {
        assert!(
            metrics(Style::Previews, &Appearance::default(), 1.0).icon.is_none(),
            "the previews style draws no icon"
        );
    }

    #[test]
    fn bigger_icons_make_bigger_items() {
        let small = metrics(Style::Icons, &Appearance { icon_size: IconSize::Small, ..Appearance::default() }, 1.0);
        let large =
            metrics(Style::Icons, &Appearance { icon_size: IconSize::ExtraLarge, ..Appearance::default() }, 1.0);
        assert!(large.height > small.height, "a larger icon size gives a taller cell");
    }

    #[test]
    fn padding_grows_the_item() {
        let base = metrics(Style::List, &Appearance::default(), 1.0);
        let padded = metrics(Style::List, &Appearance { item_padding: 20.0, ..Appearance::default() }, 1.0);
        assert_eq!(padded.height - base.height, 24.0);
    }

    #[test]
    fn vertical_icons_are_half_size_with_title_on_the_right() {
        let h = metrics(Style::Icons, &Appearance::default(), 1.0);
        let v = metrics(Style::Icons, &vertical(), 1.0);
        let (h_icon, v_icon) = (h.icon.unwrap().2, v.icon.unwrap().2);
        assert_eq!(v_icon, (h_icon / 2.0).round());
        let (ix, _, size) = v.icon.unwrap();
        assert!(v.label.0 > ix + size, "the title must start right of the icon");
    }

    #[test]
    fn vertical_previews_are_half_size() {
        let h = metrics(Style::Previews, &Appearance::default(), 1.0).thumbnail.unwrap();
        let v = metrics(Style::Previews, &vertical(), 1.0).thumbnail.unwrap();
        assert_eq!(v.2, (h.2 / 2.0).round());
    }

    #[test]
    fn more_title_lines_make_taller_labels() {
        let one = metrics(Style::Icons, &Appearance { title_lines: 1, ..Appearance::default() }, 1.0);
        let three = metrics(Style::Icons, &Appearance { title_lines: 3, ..Appearance::default() }, 1.0);
        assert_eq!(three.label_lines, 3);
        assert!(three.label.3 > one.label.3, "three title lines need a taller label than one");
    }
}
