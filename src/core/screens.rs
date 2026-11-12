// SPDX-License-Identifier: GPL-3.0-or-later

use super::geometry::{Point, Rect};

/// Index of the screen the window overlaps the most.
pub fn screen_of_rect(frame: &Rect, screens: &[Rect]) -> Option<usize> {
    screens
        .iter()
        .enumerate()
        .map(|(i, s)| (i, s.intersection_area(frame)))
        .filter(|&(_, area)| area > 0.0)
        .max_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(i, _)| i)
}

/// Index of the screen containing the point.
pub fn screen_of_point(point: Point, screens: &[Rect]) -> Option<usize> {
    screens.iter().position(|s| s.contains(point))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_screens() -> Vec<Rect> {
        vec![Rect::new(0.0, 0.0, 1000.0, 800.0), Rect::new(1000.0, 0.0, 1000.0, 800.0)]
    }

    #[test]
    fn window_spanning_two_screens_belongs_to_larger_part() {
        let frame = Rect::new(900.0, 0.0, 400.0, 300.0);
        assert_eq!(screen_of_rect(&frame, &two_screens()), Some(1));
    }

    #[test]
    fn offscreen_window_has_no_screen() {
        let frame = Rect::new(5000.0, 0.0, 100.0, 100.0);
        assert_eq!(screen_of_rect(&frame, &two_screens()), None);
    }

    #[test]
    fn point_lookup() {
        let p = Point { x: 1500.0, y: 10.0 };
        assert_eq!(screen_of_point(p, &two_screens()), Some(1));
    }
}
