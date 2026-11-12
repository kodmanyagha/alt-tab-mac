// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2_app_kit::{NSApplication, NSWindow, NSWindowWillCloseNotification};
use objc2_foundation::NSNotificationCenter;

use super::settings_window;
use crate::activator::Command;
use crate::core::actions::WindowAction;
use crate::core::model::{Pid, WindowId, WindowInfo};
use crate::core::window_kind::WindowKind;
use crate::macos::notifications::add_observer;
use crate::macos::{bundle, screens};

/// The window was opened or brought to the front: add it to the list.
pub fn report_shown(mtm: MainThreadMarker, window: &NSWindow) {
    crate::controller::report_self_window(Some(describe(mtm, window)));
}

/// Installed once so the window leaves the list when it closes.
pub fn observe_close() {
    // SAFETY: Notification name constants are immutable statics exported by AppKit.
    let name = unsafe { NSWindowWillCloseNotification };
    add_observer(&NSNotificationCenter::defaultCenter(), name, |n| {
        if n.object().is_some_and(|o| settings_window::is_settings_window(&o)) {
            crate::controller::report_self_window(None);
        }
    });
}

/// A command for this window from the switcher list (from the activator, on the main thread).
pub fn on_command(command: Command) {
    let Some(mtm) = MainThreadMarker::new() else { return };
    match command {
        Command::Focus(_) => settings_window::focus(mtm),
        Command::Act(_, action) => act(mtm, action),
    }
}

fn act(mtm: MainThreadMarker, action: WindowAction) {
    let app = NSApplication::sharedApplication(mtm);
    match action {
        WindowAction::Close => {
            settings_window::with_window(|w| w.performClose(None));
        }
        WindowAction::Minimize => {
            settings_window::with_window(|w| w.miniaturize(None));
        }
        WindowAction::HideApp => app.hide(None),
        WindowAction::QuitApp => app.terminate(None),
    }
}

fn describe(mtm: MainThreadMarker, window: &NSWindow) -> WindowInfo {
    WindowInfo {
        id: WindowId(window.windowNumber() as u32),
        pid: Pid(std::process::id() as i32),
        app_name: "alt-tab-mac".into(),
        bundle_id: bundle::identifier(),
        title: window.title().to_string(),
        frame: screens::to_cg(&window.frame(), screens::primary_height(mtm)),
        is_minimized: false,
        is_app_hidden: false,
        is_on_active_space: true,
        kind: WindowKind::Standard,
    }
}
