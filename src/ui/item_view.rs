// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::rc::Retained;
use objc2::{MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{NSImage, NSImageScaling, NSImageView, NSLineBreakMode, NSTextAlignment, NSTextField, NSView};
use objc2_foundation::{NSPoint, NSRect, NSSize, NSString};

use super::metrics::ItemMetrics;
use super::theme::{self, Look};

pub struct ItemView {
    pub root: Retained<NSView>,
    thumbnail: Retained<NSImageView>,
    icon: Retained<NSImageView>,
    label: Retained<NSTextField>,
    look: Option<Look>,
}

pub struct ItemContent<'a> {
    pub icon: Option<&'a NSImage>,
    pub thumbnail: Option<&'a NSImage>,
    pub title: &'a str,
}

impl ItemView {
    pub fn new(mtm: MainThreadMarker) -> Self {
        let root = make_root(mtm);
        let thumbnail = make_image_view(mtm);
        let icon = make_image_view(mtm);
        let label = make_label(mtm);
        for view in [&*thumbnail as &NSView, &icon, &label] {
            root.addSubview(view);
        }
        Self { root, thumbnail, icon, label, look: None }
    }

    /// Updates the inner layout if the style or size changed.
    pub fn apply_look(&mut self, look: &Look) {
        if self.look.as_ref() != Some(look) {
            self.layout_subviews(&look.metrics());
            self.label.setFont(Some(&look.font()));
            self.look = Some(look.clone());
        }
    }

    pub fn set_content(&self, content: &ItemContent) {
        self.icon.setImage(content.icon);
        self.thumbnail.setImage(content.thumbnail.or(content.icon));
        self.label.setStringValue(&NSString::from_str(content.title));
        self.center_label_vertically();
    }

    /// In row layout a short title should not stick to the top of its box: shrinks the label to the text's real height
    /// and centers it in the box.
    fn center_label_vertically(&self) {
        let Some(m) = self.look.as_ref().map(|l| l.metrics()) else { return };
        if !m.label_middle {
            return;
        }
        let (x, y, w, h) = m.label;
        let fitted = self.label.sizeThatFits(NSSize::new(w, h)).height.min(h);
        self.label.setFrame(rect(x, y + (h - fitted) / 2.0, w, fitted));
    }

    pub fn set_thumbnail(&self, image: &NSImage) {
        self.thumbnail.setImage(Some(image));
    }

    pub fn set_selected(&self, selected: bool) {
        let Some(layer) = self.root.layer() else {
            return;
        };
        let color = selected.then(|| theme::selection_color().CGColor());
        layer.setBackgroundColor(color.as_deref());
    }

    /// `frame` is in the parent view's (AppKit) coordinates.
    pub fn place(&self, frame: NSRect) {
        self.root.setFrame(frame);
        self.root.setHidden(false);
    }

    pub fn hide(&self) {
        self.root.setHidden(true);
    }

    fn layout_subviews(&self, m: &ItemMetrics) {
        let icon = m.icon.map_or(NSRect::ZERO, |(x, y, size)| rect(x, y, size, size));
        self.icon.setFrame(icon);
        self.icon.setHidden(m.icon.is_none());
        let thumb = m.thumbnail.map_or(NSRect::ZERO, |(x, y, w, h)| rect(x, y, w, h));
        self.thumbnail.setFrame(thumb);
        self.thumbnail.setHidden(m.thumbnail.is_none());
        let (x, y, w, h) = m.label;
        self.label.setFrame(rect(x, y, w, h));
        self.label.setMaximumNumberOfLines(m.label_lines);
        let align = if m.label_centered { NSTextAlignment::Center } else { NSTextAlignment::Left };
        self.label.setAlignment(align);
    }
}

fn make_root(mtm: MainThreadMarker) -> Retained<NSView> {
    let root = NSView::initWithFrame(NSView::alloc(mtm), NSRect::ZERO);
    root.setWantsLayer(true);
    if let Some(layer) = root.layer() {
        layer.setCornerRadius(theme::ITEM_CORNER_RADIUS);
    }
    root
}

fn make_image_view(mtm: MainThreadMarker) -> Retained<NSImageView> {
    let view = NSImageView::initWithFrame(NSImageView::alloc(mtm), NSRect::ZERO);
    view.setImageScaling(NSImageScaling::ScaleProportionallyUpOrDown);
    view
}

/// A wrapping label; when the line limit is reached the last line is truncated with "…".
fn make_label(mtm: MainThreadMarker) -> Retained<NSTextField> {
    let label = NSTextField::wrappingLabelWithString(&NSString::from_str(""), mtm);
    label.setLineBreakMode(NSLineBreakMode::ByWordWrapping);
    if let Some(cell) = label.cell() {
        cell.setTruncatesLastVisibleLine(true);
    }
    label
}

fn rect(x: f64, y: f64, w: f64, h: f64) -> NSRect {
    NSRect::new(NSPoint::new(x, y), NSSize::new(w, h))
}
