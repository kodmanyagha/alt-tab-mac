// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self { x, y, width, height }
    }

    pub fn max_x(&self) -> f64 {
        self.x + self.width
    }

    pub fn max_y(&self) -> f64 {
        self.y + self.height
    }

    pub fn center(&self) -> Point {
        Point { x: self.x + self.width / 2.0, y: self.y + self.height / 2.0 }
    }

    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.x && p.x < self.max_x() && p.y >= self.y && p.y < self.max_y()
    }

    /// Intersection area of two rectangles (0 if they do not intersect).
    pub fn intersection_area(&self, other: &Rect) -> f64 {
        let w = self.max_x().min(other.max_x()) - self.x.max(other.x);
        let h = self.max_y().min(other.max_y()) - self.y.max(other.y);
        if w <= 0.0 || h <= 0.0 { 0.0 } else { w * h }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_of_overlapping_rects() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(5.0, 5.0, 10.0, 10.0);
        assert_eq!(a.intersection_area(&b), 25.0);
    }

    #[test]
    fn intersection_of_disjoint_rects_is_zero() {
        let a = Rect::new(0.0, 0.0, 10.0, 10.0);
        let b = Rect::new(20.0, 0.0, 10.0, 10.0);
        assert_eq!(a.intersection_area(&b), 0.0);
    }

    #[test]
    fn contains_is_half_open() {
        let r = Rect::new(0.0, 0.0, 10.0, 10.0);
        assert!(r.contains(Point { x: 0.0, y: 0.0 }), "the origin is inside the rect");
        assert!(!r.contains(Point { x: 10.0, y: 5.0 }), "the far corner is outside the half-open rect");
    }
}
