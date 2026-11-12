// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2_app_kit::NSImage;

use super::panel::{Images, SwitcherPanel};
use super::theme::Look;
use crate::core::appearance::Theme;
use crate::core::geometry::{Point, Rect};
use crate::core::model::WindowInfo;

pub struct PanelSet {
    mtm: MainThreadMarker,
    panels: Vec<SwitcherPanel>,
    visible: usize,
    theme: Theme,
    mouse_enabled: bool,
}

impl PanelSet {
    pub fn new(mtm: MainThreadMarker) -> Self {
        Self { mtm, panels: vec![SwitcherPanel::new(mtm)], visible: 0, theme: Theme::System, mouse_enabled: true }
    }

    pub fn show(&mut self, windows: &[&WindowInfo], selected: usize, images: &Images, look: &Look, screens: &[Rect]) {
        self.ensure_panels(screens.len());
        for (panel, screen) in self.panels.iter_mut().zip(screens) {
            panel.show(windows, selected, images, look, screen);
        }
        self.hide_from(screens.len());
        self.visible = screens.len();
    }

    pub fn select(&mut self, index: usize) {
        for panel in self.visible_panels() {
            panel.select(index);
        }
    }

    pub fn update_thumbnail(&mut self, index: usize, image: &NSImage) {
        for panel in self.visible_panels() {
            panel.update_thumbnail(index, image);
        }
    }

    /// All panels share the same layout, so checking the first one is enough.
    pub fn item_at(&self, point: Point) -> Option<usize> {
        self.panels.first()?.item_at(point)
    }

    /// (columns, rows) of the open panel.
    pub fn grid_shape(&self) -> Option<(usize, usize)> {
        self.panels.first()?.grid_shape()
    }

    pub fn is_visible(&self) -> bool {
        self.visible > 0
    }

    pub fn hide(&mut self) {
        self.hide_from(0);
        self.visible = 0;
    }

    pub fn set_mouse_enabled(&mut self, enabled: bool) {
        self.mouse_enabled = enabled;
        for panel in &self.panels {
            panel.set_mouse_enabled(enabled);
        }
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
        for panel in &self.panels {
            panel.set_theme(theme);
        }
    }

    fn ensure_panels(&mut self, count: usize) {
        while self.panels.len() < count {
            let panel = SwitcherPanel::new(self.mtm);
            panel.set_theme(self.theme);
            panel.set_mouse_enabled(self.mouse_enabled);
            self.panels.push(panel);
        }
    }

    fn hide_from(&mut self, start: usize) {
        let shown = self.panels.iter_mut().skip(start).filter(|panel| panel.is_visible());
        for panel in shown {
            panel.hide();
        }
    }

    fn visible_panels(&mut self) -> impl Iterator<Item = &mut SwitcherPanel> {
        self.panels.iter_mut().take(self.visible)
    }
}
