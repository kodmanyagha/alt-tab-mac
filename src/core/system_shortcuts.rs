// SPDX-License-Identifier: GPL-3.0-or-later

use super::hotkey::{Hotkey, Modifier, Modifiers};
use super::keys;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SystemShortcut {
    CommandTab,
    CommandKeyAboveTab,
}

pub const ALL: [SystemShortcut; 2] = [SystemShortcut::CommandTab, SystemShortcut::CommandKeyAboveTab];

/// System shortcuts that must be disabled to take them over.
pub fn conflicts(hotkeys: &[Hotkey]) -> Vec<SystemShortcut> {
    let mut result = Vec::new();
    for shortcut in hotkeys.iter().filter_map(conflict_of) {
        if !result.contains(&shortcut) {
            result.push(shortcut);
        }
    }
    result
}

fn conflict_of(hotkey: &Hotkey) -> Option<SystemShortcut> {
    let cmd_only = Modifiers::NONE.with(Modifier::Cmd);
    if hotkey.modifiers.without(Modifier::Shift) != cmd_only {
        return None;
    }
    match hotkey.key_code {
        keys::TAB => Some(SystemShortcut::CommandTab),
        keys::ANSI_GRAVE | keys::ISO_SECTION => Some(SystemShortcut::CommandKeyAboveTab),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hotkey(mods: &[Modifier], key_code: u16) -> Hotkey {
        Hotkey { modifiers: Modifiers::from_list(mods), key_code }
    }

    #[test]
    fn cmd_tab_and_cmd_grave_conflict() {
        let hotkeys = [hotkey(&[Modifier::Cmd], keys::TAB), hotkey(&[Modifier::Cmd], keys::ISO_SECTION)];
        assert_eq!(conflicts(&hotkeys), vec![SystemShortcut::CommandTab, SystemShortcut::CommandKeyAboveTab]);
    }

    #[test]
    fn alt_tab_does_not_conflict() {
        assert!(conflicts(&[hotkey(&[Modifier::Alt], keys::TAB)]).is_empty(), "⌥Tab is not a system shortcut");
    }

    #[test]
    fn cmd_alt_tab_does_not_conflict() {
        assert!(
            conflicts(&[hotkey(&[Modifier::Cmd, Modifier::Alt], keys::TAB)]).is_empty(),
            "⌘⌥Tab is not a system shortcut"
        );
    }
}
