// SPDX-License-Identifier: GPL-3.0-or-later

use super::layout::{self, Bounds, Layout};
use super::theme::Look;

const LARGER_ZOOMS: [f64; 6] = [1.0, 0.85, 0.72, 0.6, 0.5, 0.42];
const SMALLEST_ZOOM: f64 = 0.35;

/// The first look and layout that fit on the screen; the smallest one if none fits.
pub fn fit(look: &Look, count: usize, bounds: Bounds) -> (Look, Layout) {
    for zoom in LARGER_ZOOMS {
        let (candidate, layout) = zoomed_layout(look, zoom, count, bounds);
        if fits(&layout, bounds) {
            return (candidate, layout);
        }
    }
    zoomed_layout(look, SMALLEST_ZOOM, count, bounds)
}

fn zoomed_layout(look: &Look, zoom: f64, count: usize, bounds: Bounds) -> (Look, Layout) {
    let candidate = look.zoomed(zoom);
    let layout = layout::grid(count, bounds, &candidate.grid_spec());
    (candidate, layout)
}

fn fits(layout: &Layout, bounds: Bounds) -> bool {
    layout.width <= bounds.width && layout.height <= bounds.height
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::appearance::Appearance;
    use crate::core::style::Style;

    const SCREEN: Bounds = Bounds { width: 1300.0, height: 800.0 };

    #[test]
    fn few_windows_keep_full_size() {
        let (look, _) = fit(&Look::new(Style::Icons, Appearance::default()), 5, SCREEN);
        assert_eq!(look.zoom, 1.0);
    }

    #[test]
    fn many_windows_shrink_until_they_fit() {
        let (look, layout) = fit(&Look::new(Style::Previews, Appearance::default()), 60, SCREEN);
        assert!(look.zoom < 1.0, "60 previews cannot fit at full size, zoom was {}", look.zoom);
        assert!(layout.height <= SCREEN.height, "the fitted layout must fit on the screen");
    }
}
