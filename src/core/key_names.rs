// SPDX-License-Identifier: GPL-3.0-or-later

use super::hotkey::{Modifier, Modifiers};
use super::keys;

/// Symbols for keys that do not produce a character. Other keys are read from the keyboard layout.
pub fn special_key_name(code: u16) -> Option<&'static str> {
    Some(match code {
        keys::TAB => "⇥",
        keys::RETURN => "↩",
        keys::KEYPAD_ENTER => "⌤",
        keys::ESCAPE => "⎋",
        keys::LEFT_ARROW => "←",
        keys::RIGHT_ARROW => "→",
        keys::DOWN_ARROW => "↓",
        keys::UP_ARROW => "↑",
        keys::SPACE => "Space",
        keys::DELETE => "⌫",
        _ => return None,
    })
}

/// In the order macOS uses: ⌃ ⌥ ⇧ ⌘
pub fn modifier_symbols(mods: Modifiers) -> String {
    [(Modifier::Ctrl, "⌃"), (Modifier::Alt, "⌥"), (Modifier::Shift, "⇧"), (Modifier::Cmd, "⌘")]
        .into_iter()
        .filter(|(m, _)| mods.has(*m))
        .map(|(_, s)| s)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbols_in_macos_order() {
        let mods = Modifiers::from_list(&[Modifier::Cmd, Modifier::Shift, Modifier::Alt]);
        assert_eq!(modifier_symbols(mods), "⌥⇧⌘");
    }

    #[test]
    fn tab_has_a_symbol() {
        assert_eq!(special_key_name(keys::TAB), Some("⇥"));
        assert_eq!(special_key_name(0), None);
    }
}
