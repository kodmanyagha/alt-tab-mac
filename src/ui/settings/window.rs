// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::rc::Retained;
use objc2::{MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSAutoresizingMaskOptions, NSBackingStoreType, NSScrollView, NSTabView, NSTabViewItem, NSView, NSWindow,
    NSWindowStyleMask,
};
use objc2_foundation::{NSPoint, NSRect, NSSize};

use super::tabs::TABS;

const WIDTH: f64 = 860.0;
const HEIGHT: f64 = 700.0;
const PADDING: f64 = 20.0;

pub fn make_window(mtm: MainThreadMarker) -> (Retained<NSWindow>, Retained<NSTabView>) {
    let window = new_window(mtm);
    let tabs = new_tab_view(mtm);
    window.setContentView(Some(&tabs));
    window.center();
    (window, tabs)
}

fn new_window(mtm: MainThreadMarker) -> Retained<NSWindow> {
    let style = NSWindowStyleMask::Titled | NSWindowStyleMask::Closable | NSWindowStyleMask::Miniaturizable;
    let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(WIDTH, HEIGHT));
    // SAFETY: The window is freshly allocated on the main thread with a valid frame and style mask.
    let window = unsafe {
        NSWindow::initWithContentRect_styleMask_backing_defer(
            NSWindow::alloc(mtm),
            frame,
            style,
            NSBackingStoreType::Buffered,
            false,
        )
    };
    // SAFETY: The window is owned by a `Retained`, so AppKit must not release it again on close.
    unsafe { window.setReleasedWhenClosed(false) };
    window
}

fn new_tab_view(mtm: MainThreadMarker) -> Retained<NSTabView> {
    let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(WIDTH, HEIGHT));
    let tabs = NSTabView::initWithFrame(NSTabView::alloc(mtm), frame);
    tabs.setAutoresizingMask(
        NSAutoresizingMaskOptions::ViewWidthSizable | NSAutoresizingMaskOptions::ViewHeightSizable,
    );
    for _ in &TABS {
        tabs.addTabViewItem(&NSTabViewItem::new());
    }
    tabs
}

/// Puts the tab content in a padded container. Scrollable content is pinned to all four edges; other content is aligned
/// to the top.
pub fn padded(mtm: MainThreadMarker, content: &NSView) -> Retained<NSView> {
    use objc2::ClassType;
    use objc2_foundation::NSObjectProtocol;

    let container = NSView::initWithFrame(NSView::alloc(mtm), NSRect::ZERO);
    container.addSubview(content);
    content.setTranslatesAutoresizingMaskIntoConstraints(false);
    content.topAnchor().constraintEqualToAnchor_constant(&container.topAnchor(), PADDING).setActive(true);
    content.leadingAnchor().constraintEqualToAnchor_constant(&container.leadingAnchor(), PADDING).setActive(true);
    content.trailingAnchor().constraintEqualToAnchor_constant(&container.trailingAnchor(), -PADDING).setActive(true);
    if content.isKindOfClass(NSScrollView::class()) {
        content.bottomAnchor().constraintEqualToAnchor_constant(&container.bottomAnchor(), -PADDING).setActive(true);
    }
    container
}
