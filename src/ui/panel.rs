// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSImage, NSPanel, NSView, NSVisualEffectView, NSWindowOrderingMode};
use objc2_foundation::{NSPoint, NSRect, NSSize};

use super::fit::fit;
use super::icons::IconCache;
use super::item_view::{ItemContent, ItemView};
use super::layout::{self, Bounds, Layout};
use super::mouse_view::MouseView;
use super::theme::{self, Look};
use super::thumbnails::ThumbnailCache;
use super::window::{make_background, make_panel};
use crate::core::appearance::Theme;
use crate::core::geometry::{Point, Rect};
use crate::core::model::WindowInfo;
use crate::core::style::Style;
use crate::macos::screens;

pub struct Images<'a> {
    pub icons: &'a IconCache,
    pub thumbnails: &'a ThumbnailCache,
}

pub struct SwitcherPanel {
    mtm: MainThreadMarker,
    panel: Retained<NSPanel>,
    background: Retained<NSVisualEffectView>,
    mouse: Retained<MouseView>,
    items: Vec<ItemView>,
    layout: Option<Layout>,
    selected: Option<usize>,
}

impl SwitcherPanel {
    pub fn new(mtm: MainThreadMarker) -> Self {
        let panel = make_panel(mtm);
        let background = make_background(mtm);
        let mouse = MouseView::new(mtm, background.bounds());
        background.addSubview(&mouse);
        panel.setContentView(Some(&background));
        Self { mtm, panel, background, mouse, items: Vec::new(), layout: None, selected: None }
    }

    /// Shows the panel on `screen` (CG coordinates).
    pub fn show(&mut self, windows: &[&WindowInfo], selected: usize, images: &Images, look: &Look, screen: &Rect) {
        let bounds = Bounds {
            width: screen.width * theme::MAX_SCREEN_FRACTION,
            height: screen.height * theme::MAX_SCREEN_FRACTION,
        };
        let (look, layout) = fit(look, windows.len(), bounds);
        self.ensure_item_views(windows.len());
        self.fill_items(windows, images, &look, &layout);
        self.place_panel(&layout, screen);
        self.layout = Some(layout);
        self.select(selected);
        self.panel.orderFrontRegardless();
    }

    pub fn select(&mut self, index: usize) {
        if let Some(old) = self.selected.and_then(|i| self.items.get(i)) {
            old.set_selected(false);
        }
        if let Some(new) = self.items.get(index) {
            new.set_selected(true);
        }
        self.selected = Some(index);
    }

    /// Updates the preview of an item in the open panel in place.
    pub fn update_thumbnail(&self, index: usize, image: &NSImage) {
        if let Some(item) = self.items.get(index) {
            item.set_thumbnail(image);
        }
    }

    pub fn grid_shape(&self) -> Option<(usize, usize)> {
        self.layout.as_ref().map(|l| (l.columns, l.rows))
    }

    /// Item under the point, relative to the panel's top-left corner.
    pub fn item_at(&self, point: Point) -> Option<usize> {
        self.layout.as_ref()?.items.iter().position(|r| r.contains(point))
    }

    /// When disabled, the panel ignores mouse events and clicks go to the window behind it.
    pub fn set_mouse_enabled(&self, enabled: bool) {
        self.panel.setIgnoresMouseEvents(!enabled);
    }

    pub fn set_theme(&self, theme: Theme) {
        use objc2_app_kit::NSAppearanceCustomization;

        self.panel.setAppearance(theme::appearance(theme).as_deref());
    }

    /// The panel's content, for the developer tool (`--render-panel`).
    pub fn content_view(&self) -> Option<Retained<NSView>> {
        self.panel.contentView()
    }

    pub fn is_visible(&self) -> bool {
        self.selected.is_some()
    }

    pub fn hide(&mut self) {
        self.panel.orderOut(None);
        self.selected = None;
        self.layout = None;
    }

    fn ensure_item_views(&mut self, count: usize) {
        while self.items.len() < count {
            let item = ItemView::new(self.mtm);
            self.background.addSubview_positioned_relativeTo(
                &item.root,
                NSWindowOrderingMode::Below,
                Some(&self.mouse),
            );
            self.items.push(item);
        }
    }

    fn fill_items(&mut self, windows: &[&WindowInfo], images: &Images, look: &Look, layout: &Layout) {
        for (i, view) in self.items.iter_mut().enumerate() {
            let Some(w) = windows.get(i) else {
                view.hide();
                continue;
            };
            view.apply_look(look);
            view.set_selected(false);
            view.set_content(&content_for(w, images, look.style));
            view.place(flip(&layout.items[i], layout.height));
        }
    }

    fn place_panel(&self, layout: &Layout, screen: &Rect) {
        let frame_cg = layout::centered(layout.width, layout.height, screen);
        let frame = screens::to_appkit(&frame_cg, screens::primary_height(self.mtm));
        self.panel.setFrame_display(frame, false);
    }
}

fn content_for<'a>(w: &'a WindowInfo, images: &'a Images, style: Style) -> ItemContent<'a> {
    let thumbnail = if style.uses_previews() { images.thumbnails.get(w.id) } else { None };
    ItemContent { icon: images.icons.get(w.pid), thumbnail, title: display_title(w) }
}

/// Windows without a title show the app name.
fn display_title(w: &WindowInfo) -> &str {
    if w.title.is_empty() { &w.app_name } else { &w.title }
}

/// Converts a top-left-origin layout frame to AppKit's bottom-left origin.
fn flip(rect: &Rect, container_height: f64) -> NSRect {
    let y = container_height - rect.y - rect.height;
    NSRect::new(NSPoint::new(rect.x, y), NSSize::new(rect.width, rect.height))
}
