// SPDX-License-Identifier: GPL-3.0-or-later

use objc2_app_kit::NSRunningApplication;

use super::ax::AxElement;
use crate::core::actions::WindowAction;
use crate::core::model::Pid;

pub fn perform(action: WindowAction, pid: Pid, window: &AxElement, app: &AxElement) {
    let ok = match action {
        WindowAction::Close => window.close(),
        WindowAction::Minimize => window.set_minimized(true),
        WindowAction::HideApp => app.set_hidden(true),
        WindowAction::QuitApp => quit(pid),
    };
    if !ok {
        tracing::warn!(action = ?action, pid = pid.0, "window action failed");
    }
}

fn quit(pid: Pid) -> bool {
    NSRunningApplication::runningApplicationWithProcessIdentifier(pid.0).is_some_and(|app| app.terminate())
}
