// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::rc::Retained;
use objc2::{MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSColor, NSLayoutAttribute, NSLayoutConstraintOrientation, NSScrollView, NSStackView, NSStackViewDistribution,
    NSUserInterfaceLayoutOrientation, NSView,
};
use objc2_foundation::{NSArray, NSRect};

use super::controls::header;
use super::flipped_clip::FlippedClipView;

const FIELDSET_INSET: f64 = 12.0;
const COLUMN_GAP: f64 = 16.0;
const HUG_CONTENT: f32 = 750.0;

/// Stacks views vertically; each one takes the full width of the stack.
pub fn vstack(mtm: MainThreadMarker, views: &[&NSView], spacing: f64) -> Retained<NSStackView> {
    let stack = stack(mtm, views, NSUserInterfaceLayoutOrientation::Vertical);
    stack.setAlignment(NSLayoutAttribute::Leading);
    stack.setSpacing(spacing);
    stack.setDistribution(NSStackViewDistribution::Fill);
    stack.setHuggingPriority_forOrientation(HUG_CONTENT, NSLayoutConstraintOrientation::Vertical);
    for view in views {
        pin_horizontally(view, &stack, 0.0);
    }
    stack
}

/// Places views side by side (at their natural widths).
pub fn hstack(mtm: MainThreadMarker, views: &[&NSView]) -> Retained<NSStackView> {
    let stack = stack(mtm, views, NSUserInterfaceLayoutOrientation::Horizontal);
    stack.setSpacing(8.0);
    stack
}

/// Like an HTML fieldset: a titled box with a light background; its items are stacked at full width.
pub fn fieldset(mtm: MainThreadMarker, title: &str, items: &[&NSView]) -> Retained<NSView> {
    let title = header(mtm, title);
    let mut views: Vec<&NSView> = vec![&title];
    views.extend_from_slice(items);
    card(mtm, &vstack(mtm, &views, 10.0))
}

/// Places boxes in two equal columns aligned at the top.
pub fn two_columns(mtm: MainThreadMarker, left: &[&NSView], right: &[&NSView]) -> Retained<NSStackView> {
    let left = vstack(mtm, left, COLUMN_GAP);
    let right = vstack(mtm, right, COLUMN_GAP);
    let row = stack(mtm, &[&left, &right], NSUserInterfaceLayoutOrientation::Horizontal);
    row.setDistribution(NSStackViewDistribution::FillEqually);
    row.setAlignment(NSLayoutAttribute::Top);
    row.setSpacing(COLUMN_GAP);
    row
}

/// A rounded box with a light background that takes its size from its content.
pub fn card(mtm: MainThreadMarker, content: &NSView) -> Retained<NSView> {
    let container = NSView::initWithFrame(NSView::alloc(mtm), NSRect::ZERO);
    container.setWantsLayer(true);
    if let Some(layer) = container.layer() {
        layer.setCornerRadius(8.0);
        layer.setBorderWidth(1.0);
        layer.setBorderColor(Some(&NSColor::separatorColor().CGColor()));
        layer.setBackgroundColor(Some(&NSColor::quaternaryLabelColor().colorWithAlphaComponent(0.12).CGColor()));
    }
    container.addSubview(content);
    content.setTranslatesAutoresizingMaskIntoConstraints(false);
    pin_horizontally(content, &container, FIELDSET_INSET);
    content.topAnchor().constraintEqualToAnchor_constant(&container.topAnchor(), FIELDSET_INSET).setActive(true);
    content.bottomAnchor().constraintEqualToAnchor_constant(&container.bottomAnchor(), -FIELDSET_INSET).setActive(true);
    container
}

/// Scrollable area for long tabs (shortcuts).
pub fn scroll(mtm: MainThreadMarker, content: &NSView) -> Retained<NSScrollView> {
    let scroll = NSScrollView::initWithFrame(NSScrollView::alloc(mtm), NSRect::ZERO);
    scroll.setHasVerticalScroller(true);
    scroll.setDrawsBackground(false);
    let clip = FlippedClipView::new(mtm);
    clip.setDrawsBackground(false);
    scroll.setContentView(&clip);
    scroll.setDocumentView(Some(content));
    content.setTranslatesAutoresizingMaskIntoConstraints(false);
    let clip = scroll.contentView();
    pin_horizontally(content, &clip, 0.0);
    content.topAnchor().constraintEqualToAnchor(&clip.topAnchor()).setActive(true);
    scroll
}

/// Pins the left and right edges of `child` to `parent` (inset by `inset`).
fn pin_horizontally(child: &NSView, parent: &NSView, inset: f64) {
    child.leadingAnchor().constraintEqualToAnchor_constant(&parent.leadingAnchor(), inset).setActive(true);
    child.trailingAnchor().constraintEqualToAnchor_constant(&parent.trailingAnchor(), -inset).setActive(true);
}

fn stack(
    mtm: MainThreadMarker,
    views: &[&NSView],
    orientation: NSUserInterfaceLayoutOrientation,
) -> Retained<NSStackView> {
    let array = NSArray::from_slice(views);
    let stack = NSStackView::stackViewWithViews(&array, mtm);
    stack.setOrientation(orientation);
    stack
}
