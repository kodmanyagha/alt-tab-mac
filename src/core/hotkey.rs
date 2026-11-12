// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Modifier {
    Cmd,
    #[serde(alias = "option")]
    Alt,
    Ctrl,
    Shift,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Modifiers(u8);

impl Modifiers {
    pub const NONE: Self = Self(0);

    pub fn from_list(list: &[Modifier]) -> Self {
        list.iter().fold(Self::NONE, |acc, &m| acc.with(m))
    }

    pub fn with(self, m: Modifier) -> Self {
        Self(self.0 | Self::bit(m))
    }

    pub fn has(self, m: Modifier) -> bool {
        self.0 & Self::bit(m) != 0
    }

    /// Whether all keys in `required` are pressed.
    pub fn contains(self, required: Modifiers) -> bool {
        self.0 & required.0 == required.0
    }

    pub fn without(self, m: Modifier) -> Self {
        Self(self.0 & !Self::bit(m))
    }

    fn bit(m: Modifier) -> u8 {
        match m {
            Modifier::Cmd => 1,
            Modifier::Alt => 2,
            Modifier::Ctrl => 4,
            Modifier::Shift => 8,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Hotkey {
    pub modifiers: Modifiers,
    pub key_code: u16,
}

impl Hotkey {
    /// Whether the key press triggers this shortcut. Shift may be pressed in addition (to go backwards); other
    /// modifiers must match exactly.
    pub fn matches(&self, key_code: u16, pressed: Modifiers) -> bool {
        key_code == self.key_code && pressed.without(Modifier::Shift) == self.modifiers.without(Modifier::Shift)
    }

    /// Whether the keys that keep the panel open are still held.
    pub fn is_held(&self, pressed: Modifiers) -> bool {
        pressed.contains(self.modifiers.without(Modifier::Shift))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cmd_tab() -> Hotkey {
        Hotkey { modifiers: Modifiers::from_list(&[Modifier::Cmd]), key_code: 48 }
    }

    #[test]
    fn matches_with_extra_shift() {
        let pressed = Modifiers::from_list(&[Modifier::Cmd, Modifier::Shift]);
        assert!(cmd_tab().matches(48, pressed), "⌘⇧Tab still matches ⌘Tab to step backwards");
    }

    #[test]
    fn does_not_match_with_extra_alt() {
        let pressed = Modifiers::from_list(&[Modifier::Cmd, Modifier::Alt]);
        assert!(!cmd_tab().matches(48, pressed), "⌘⌥Tab is a different shortcut than ⌘Tab");
    }

    #[test]
    fn released_when_cmd_lifted() {
        assert!(!cmd_tab().is_held(Modifiers::from_list(&[Modifier::Shift])), "lifting ⌘ releases the ⌘Tab switcher");
        assert!(cmd_tab().is_held(Modifiers::from_list(&[Modifier::Cmd])), "holding ⌘ keeps the ⌘Tab switcher open");
    }
}
