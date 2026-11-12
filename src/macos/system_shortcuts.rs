// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::atomic::{AtomicBool, Ordering};

use super::ffi::symbolic_hotkeys::{
    COMMAND_KEY_ABOVE_TAB, COMMAND_SHIFT_TAB, COMMAND_TAB, set_symbolic_hotkey_enabled,
};
use crate::core::system_shortcuts::{ALL, SystemShortcut};

static DISABLED_ANY: AtomicBool = AtomicBool::new(false);

/// Disables only the shortcuts in `disabled` and enables the others.
pub fn apply(disabled: &[SystemShortcut]) {
    for shortcut in ALL {
        set_enabled(shortcut, !disabled.contains(&shortcut));
    }
    DISABLED_ANY.store(!disabled.is_empty(), Ordering::SeqCst);
    if !disabled.is_empty() {
        tracing::info!(shortcuts = ?disabled, "took over system shortcuts");
    }
}

/// Re-enables all system shortcuts.
pub fn restore() {
    if DISABLED_ANY.swap(false, Ordering::SeqCst) {
        for shortcut in ALL {
            set_enabled(shortcut, true);
        }
    }
}

fn set_enabled(shortcut: SystemShortcut, enabled: bool) {
    let Some(set) = set_symbolic_hotkey_enabled() else {
        tracing::warn!("cannot change system shortcut, API not found");
        return;
    };
    for id in ids(shortcut) {
        // SAFETY: `set` was resolved from SkyLight with the matching `(i32, bool) -> i32` signature.
        let err = unsafe { set(*id, enabled) };
        if err != 0 {
            tracing::warn!(hotkey = id, error = err, "could not set symbolic hotkey");
        }
    }
}

fn ids(shortcut: SystemShortcut) -> &'static [i32] {
    match shortcut {
        SystemShortcut::CommandTab => &[COMMAND_TAB, COMMAND_SHIFT_TAB],
        SystemShortcut::CommandKeyAboveTab => &[COMMAND_KEY_ABOVE_TAB],
    }
}
