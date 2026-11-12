// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2_app_kit::NSScreen;
use objc2_foundation::{NSPoint, NSRect, NSSize};

use crate::core::geometry::Rect;
use crate::core::model::ScreenInfo;

/// The first screen is always the main screen (the one with the menu bar).
pub fn screen_infos(mtm: MainThreadMarker) -> Vec<ScreenInfo> {
    let screens = NSScreen::screens(mtm);
    let primary_height = primary_height(mtm);
    screens
        .iter()
        .map(|s| ScreenInfo { frame: to_cg(&s.frame(), primary_height), name: s.localizedName().to_string() })
        .collect()
}

/// AppKit (bottom-left origin, y up) → CG (top-left origin, y down).
pub fn to_cg(frame: &NSRect, primary_height: f64) -> Rect {
    let y = primary_height - (frame.origin.y + frame.size.height);
    Rect::new(frame.origin.x, y, frame.size.width, frame.size.height)
}

/// CG → AppKit conversion (for positioning the panel).
pub fn to_appkit(rect: &Rect, primary_height: f64) -> NSRect {
    let y = primary_height - (rect.y + rect.height);
    NSRect::new(NSPoint::new(rect.x, y), NSSize::new(rect.width, rect.height))
}

pub fn primary_height(mtm: MainThreadMarker) -> f64 {
    NSScreen::screens(mtm).iter().next().map(|s| s.frame().size.height).unwrap_or(0.0)
}
