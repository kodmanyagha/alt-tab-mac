// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell;

use dispatch2::DispatchQueue;
use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2_app_kit::{NSApplication, NSTabView, NSWindow};

use super::tabs::TABS;
use objc2_foundation::NSString;

use super::window::{make_window, padded};
use crate::i18n::{Key, t};

struct SettingsWindow {
    window: Retained<NSWindow>,
    tabs: Retained<NSTabView>,
}

thread_local! {
    static SETTINGS: RefCell<Option<SettingsWindow>> = const { RefCell::new(None) };
}

/// Opens the settings window (brings it to the front if already open).
pub fn open(mtm: MainThreadMarker) {
    ensure_window(mtm);
    rebuild_now(mtm);
    focus(mtm);
}

/// Brings the window to the front without rebuilding its content.
pub fn focus(mtm: MainThreadMarker) {
    let shown = with_window(|w| {
        #[allow(deprecated)]
        NSApplication::sharedApplication(mtm).activateIgnoringOtherApps(true);
        w.makeKeyAndOrderFront(None);
        super::own_window::report_shown(mtm, w);
    });
    if shown.is_none() {
        open(mtm);
    }
}

/// Access to the settings window if it has been created.
pub fn with_window<R>(f: impl FnOnce(&NSWindow) -> R) -> Option<R> {
    SETTINGS.with_borrow(|s| s.as_ref().map(|s| f(&s.window)))
}

pub fn is_settings_window(object: &AnyObject) -> bool {
    with_window(|w| std::ptr::eq(object as *const AnyObject as *const NSWindow, w)).unwrap_or(false)
}

/// Rebuilds the tabs with the current settings. Deferred to the next run loop turn so it can be called from inside a
/// control's action.
pub fn rebuild() {
    DispatchQueue::main().exec_async(|| {
        if let Some(mtm) = MainThreadMarker::new() {
            rebuild_now(mtm);
        }
    });
}

fn ensure_window(mtm: MainThreadMarker) {
    SETTINGS.with_borrow_mut(|s| {
        if s.is_none() {
            let (window, tabs) = make_window(mtm);
            super::own_window::observe_close();
            *s = Some(SettingsWindow { window, tabs });
        }
    });
}

fn rebuild_now(mtm: MainThreadMarker) {
    SETTINGS.with_borrow(|s| {
        if let Some(s) = s {
            s.window.setTitle(&NSString::from_str(t(Key::SettingsTitle)));
            rebuild_into(mtm, &s.tabs);
        }
    });
}

/// Builds the tab contents from scratch with the current settings.
pub fn rebuild_into(mtm: MainThreadMarker, tabs: &NSTabView) {
    super::binding::reset();
    let config = crate::controller::current_config();
    for (index, tab) in TABS.iter().enumerate() {
        let content = (tab.build)(mtm, &config);
        let item = tabs.tabViewItemAtIndex(index as isize);
        item.setLabel(&NSString::from_str(t(tab.title)));
        item.setView(Some(&padded(mtm, &content)));
    }
}
