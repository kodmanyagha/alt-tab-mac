// SPDX-License-Identifier: GPL-3.0-or-later

use crate::config::shortcut::ShortcutConfig;
use crate::core::hotkey::{Hotkey, Modifier, Modifiers};
use crate::core::scope::Scope;
use crate::core::style::Style;
use crate::macos::keyboard::resolve_key;

#[derive(Default)]
pub struct Bindings {
    pub hotkeys: Vec<Hotkey>,
    pub scopes: Vec<Scope>,
    pub styles: Vec<Style>,
}

pub fn build(shortcuts: &[ShortcutConfig]) -> Bindings {
    let mut bindings = Bindings::default();
    for shortcut in shortcuts.iter().filter(|s| s.enabled) {
        match to_hotkey(shortcut) {
            Some(hotkey) => {
                bindings.hotkeys.push(hotkey);
                bindings.scopes.push(shortcut.scope);
                bindings.styles.push(shortcut.style);
            }
            None => tracing::warn!(shortcut = ?shortcut, "shortcut ignored, no modifier other than Shift"),
        }
    }
    bindings
}

/// A shortcut without modifiers would swallow even the plain Tab key, so it is not allowed.
fn to_hotkey(shortcut: &ShortcutConfig) -> Option<Hotkey> {
    let modifiers = Modifiers::from_list(&shortcut.modifiers);
    if modifiers.without(Modifier::Shift) == Modifiers::NONE {
        return None;
    }
    Some(Hotkey { modifiers, key_code: resolve_key(shortcut.key) })
}
