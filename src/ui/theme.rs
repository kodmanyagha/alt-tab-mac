// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::rc::Retained;
use objc2_app_kit::{NSAppearance, NSAppearanceNameAqua, NSAppearanceNameDarkAqua, NSColor, NSFont};

use super::fonts;
use super::layout::GridSpec;
use super::metrics::{ItemMetrics, metrics};
use crate::core::appearance::{Appearance, Theme};
use crate::core::style::Style;

pub const PANEL_PADDING: f64 = 18.0;
pub const PANEL_CORNER_RADIUS: f64 = 16.0;
pub const ITEM_CORNER_RADIUS: f64 = 10.0;
pub const MAX_SCREEN_FRACTION: f64 = 0.9;

#[derive(Clone, Debug, PartialEq)]
pub struct Look {
    pub style: Style,
    pub appearance: Appearance,
    pub zoom: f64,
}

impl Look {
    pub fn metrics(&self) -> ItemMetrics {
        metrics(self.style, &self.appearance, self.zoom)
    }

    pub fn grid_spec(&self) -> GridSpec {
        let m = self.metrics();
        GridSpec {
            item_width: m.width,
            item_height: m.height,
            spacing: self.appearance.item_spacing,
            padding: PANEL_PADDING,
            orientation: self.appearance.orientation,
            max_columns: (self.style == Style::List).then_some(1),
        }
    }

    pub fn new(style: Style, appearance: Appearance) -> Self {
        Self { style, appearance, zoom: 1.0 }
    }

    pub fn zoomed(&self, zoom: f64) -> Self {
        Self { zoom, ..self.clone() }
    }

    pub fn font(&self) -> Retained<NSFont> {
        fonts::font(&self.appearance.font_family, self.appearance.font_size)
    }
}

pub fn selection_color() -> Retained<NSColor> {
    NSColor::controlAccentColor().colorWithAlphaComponent(0.45)
}

/// `None` means: follow the system appearance.
pub fn appearance(theme: Theme) -> Option<Retained<NSAppearance>> {
    let name = match theme {
        Theme::System => return None,
        // SAFETY: Appearance name constants are immutable statics exported by AppKit.
        Theme::Light => unsafe { NSAppearanceNameAqua },
        // SAFETY: Appearance name constants are immutable statics exported by AppKit.
        Theme::Dark => unsafe { NSAppearanceNameDarkAqua },
    };
    NSAppearance::appearanceNamed(name)
}
