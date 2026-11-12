// SPDX-License-Identifier: GPL-3.0-or-later

use crate::core::appearance::Orientation;
use crate::core::geometry::Rect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridSpec {
    pub item_width: f64,
    pub item_height: f64,
    pub spacing: f64,
    pub padding: f64,
    pub orientation: Orientation,
    pub max_columns: Option<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Layout {
    pub columns: usize,
    pub rows: usize,
    pub width: f64,
    pub height: f64,
    pub items: Vec<Rect>,
}

#[derive(Clone, Copy, Debug)]
pub struct Bounds {
    pub width: f64,
    pub height: f64,
}

pub fn grid(count: usize, bounds: Bounds, spec: &GridSpec) -> Layout {
    let (columns, rows) = shape(count, bounds, spec);
    let items = (0..count).map(|i| item_rect(cell(i, columns, rows, spec.orientation), spec)).collect();
    Layout {
        columns,
        rows,
        width: span(columns, spec.item_width, spec),
        height: span(rows, spec.item_height, spec),
        items,
    }
}

/// Number of columns and rows.
fn shape(count: usize, bounds: Bounds, spec: &GridSpec) -> (usize, usize) {
    match spec.orientation {
        Orientation::Horizontal => {
            let limit = spec.max_columns.unwrap_or(usize::MAX);
            let columns = fits(count, bounds.width, spec.item_width, spec).min(limit);
            (columns, count.div_ceil(columns))
        }
        Orientation::Vertical => {
            let rows = fits(count, bounds.height, spec.item_height, spec);
            (count.div_ceil(rows), rows)
        }
    }
}

/// How many items fit in `available` length (at least 1, at most `count`).
fn fits(count: usize, available: f64, item: f64, spec: &GridSpec) -> usize {
    let usable = available - 2.0 * spec.padding + spec.spacing;
    let fit = (usable / (item + spec.spacing)).floor().max(1.0) as usize;
    fit.min(count.max(1))
}

/// (column, row) position of the item at `index`.
fn cell(index: usize, columns: usize, rows: usize, orientation: Orientation) -> (usize, usize) {
    match orientation {
        Orientation::Horizontal => (index % columns.max(1), index / columns.max(1)),
        Orientation::Vertical => (index / rows.max(1), index % rows.max(1)),
    }
}

/// Length taken by `n` items including the padding.
fn span(n: usize, item: f64, spec: &GridSpec) -> f64 {
    let n = n.max(1) as f64;
    2.0 * spec.padding + n * item + (n - 1.0) * spec.spacing
}

fn item_rect((col, row): (usize, usize), spec: &GridSpec) -> Rect {
    let x = spec.padding + col as f64 * (spec.item_width + spec.spacing);
    let y = spec.padding + row as f64 * (spec.item_height + spec.spacing);
    Rect::new(x, y, spec.item_width, spec.item_height)
}

/// Centers the panel on the screen (CG coordinates).
pub fn centered(width: f64, height: f64, screen: &Rect) -> Rect {
    let c = screen.center();
    Rect::new((c.x - width / 2.0).round(), (c.y - height / 2.0).round(), width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEC: GridSpec = GridSpec {
        item_width: 100.0,
        item_height: 100.0,
        spacing: 10.0,
        padding: 20.0,
        orientation: Orientation::Horizontal,
        max_columns: None,
    };

    const WIDE: Bounds = Bounds { width: 2000.0, height: 2000.0 };

    #[test]
    fn single_row_when_it_fits() {
        let l = grid(3, WIDE, &SPEC);
        assert_eq!((l.columns, l.rows), (3, 1));
        assert_eq!(l.width, 20.0 * 2.0 + 300.0 + 20.0);
    }

    #[test]
    fn wraps_to_multiple_rows() {
        let l = grid(5, Bounds { width: 260.0, height: 2000.0 }, &SPEC);
        assert_eq!((l.columns, l.rows), (2, 3));
        assert_eq!(l.items[2], Rect::new(20.0, 130.0, 100.0, 100.0));
    }

    #[test]
    fn vertical_fills_columns_top_to_bottom() {
        let spec = GridSpec { orientation: Orientation::Vertical, ..SPEC };
        let l = grid(5, Bounds { width: 2000.0, height: 260.0 }, &spec);
        assert_eq!((l.columns, l.rows), (3, 2));
        assert_eq!(l.items[2], Rect::new(130.0, 20.0, 100.0, 100.0));
    }

    #[test]
    fn vertical_single_column_when_it_fits() {
        let spec = GridSpec { orientation: Orientation::Vertical, ..SPEC };
        let l = grid(3, WIDE, &spec);
        assert_eq!((l.columns, l.rows), (1, 3));
    }

    #[test]
    fn empty_list_still_has_a_size() {
        let l = grid(0, WIDE, &SPEC);
        assert_eq!(l.rows, 0);
        assert!(l.width > 0.0, "an empty list still gets a visible panel width");
    }

    #[test]
    fn max_columns_forces_single_column() {
        let spec = GridSpec { max_columns: Some(1), ..SPEC };
        let l = grid(3, WIDE, &spec);
        assert_eq!((l.columns, l.rows), (1, 3));
    }

    #[test]
    fn centering() {
        let r = centered(100.0, 50.0, &Rect::new(0.0, 0.0, 1000.0, 800.0));
        assert_eq!(r, Rect::new(450.0, 375.0, 100.0, 50.0));
    }
}
