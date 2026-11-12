// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::rc::Retained;
use objc2::{MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSBackingStoreType, NSColor, NSPanel, NSVisualEffectBlendingMode, NSVisualEffectMaterial, NSVisualEffectState,
    NSVisualEffectView, NSWindowCollectionBehavior, NSWindowStyleMask,
};
use objc2_foundation::{NSPoint, NSRect, NSSize};

use super::theme;

const POPUP_MENU_LEVEL: isize = 101;

pub fn make_panel(mtm: MainThreadMarker) -> Retained<NSPanel> {
    let style = NSWindowStyleMask::Borderless | NSWindowStyleMask::NonactivatingPanel;
    let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(100.0, 100.0));
    let panel = NSPanel::initWithContentRect_styleMask_backing_defer(
        NSPanel::alloc(mtm),
        frame,
        style,
        NSBackingStoreType::Buffered,
        false,
    );
    configure(&panel);
    panel
}

fn configure(panel: &NSPanel) {
    panel.setLevel(POPUP_MENU_LEVEL);
    panel.setOpaque(false);
    panel.setBackgroundColor(Some(&NSColor::clearColor()));
    panel.setHasShadow(true);
    panel.setHidesOnDeactivate(false);
    // SAFETY: The panel is owned by a `Retained`, so AppKit must not release it again on close.
    unsafe { panel.setReleasedWhenClosed(false) };
    panel.setCollectionBehavior(
        NSWindowCollectionBehavior::CanJoinAllSpaces
            | NSWindowCollectionBehavior::FullScreenAuxiliary
            | NSWindowCollectionBehavior::Transient
            | NSWindowCollectionBehavior::IgnoresCycle,
    );
}

/// Blurred background with rounded corners.
pub fn make_background(mtm: MainThreadMarker) -> Retained<NSVisualEffectView> {
    let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(100.0, 100.0));
    let view = NSVisualEffectView::initWithFrame(NSVisualEffectView::alloc(mtm), frame);
    view.setMaterial(NSVisualEffectMaterial::HUDWindow);
    view.setBlendingMode(NSVisualEffectBlendingMode::BehindWindow);
    view.setState(NSVisualEffectState::Active);
    view.setWantsLayer(true);
    if let Some(layer) = view.layer() {
        layer.setCornerRadius(theme::PANEL_CORNER_RADIUS);
        layer.setMasksToBounds(true);
    }
    view
}
