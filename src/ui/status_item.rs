// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell;

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSImage, NSStatusBar, NSStatusItem};
use objc2_foundation::NSString;

use super::status_menu::{MenuTarget, build_menu};

const VARIABLE_LENGTH: f64 = -1.0;

struct StatusItem {
    item: Retained<NSStatusItem>,
    target: Retained<MenuTarget>,
}

thread_local! {
    static STATUS_ITEM: RefCell<Option<StatusItem>> = const { RefCell::new(None) };
}

pub fn install(mtm: MainThreadMarker) {
    let target = MenuTarget::new(mtm);
    let item = NSStatusBar::systemStatusBar().statusItemWithLength(VARIABLE_LENGTH);
    if let Some(button) = item.button(mtm) {
        button.setImage(status_icon().as_deref());
    }
    item.setMenu(Some(&build_menu(mtm, &target)));
    STATUS_ITEM.set(Some(StatusItem { item, target }));
}

/// Rebuilds the menu with the new texts when the language changes.
pub fn rebuild_menu(mtm: MainThreadMarker) {
    STATUS_ITEM.with_borrow(|s| {
        if let Some(s) = s {
            s.item.setMenu(Some(&build_menu(mtm, &s.target)));
        }
    });
}

pub fn set_visible(visible: bool) {
    STATUS_ITEM.with_borrow(|s| {
        if let Some(s) = s {
            s.item.setVisible(visible);
        }
    });
}

fn status_icon() -> Option<Retained<NSImage>> {
    let image = NSImage::imageWithSystemSymbolName_accessibilityDescription(
        &NSString::from_str("rectangle.stack"),
        Some(&NSString::from_str("alt-tab-mac")),
    )?;
    image.setTemplate(true);
    Some(image)
}
