// SPDX-License-Identifier: GPL-3.0-or-later

use super::actions::{PanelKeys, WindowAction};
use super::grid_nav::Direction;
use super::hotkey::{Hotkey, Modifier, Modifiers};
use super::keys;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyEvent {
    Down { code: u16, mods: Modifiers },
    Up { code: u16 },
    FlagsChanged { mods: Modifiers },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intent {
    PassThrough,
    Swallow,
    Trigger { hotkey: usize, reverse: bool },
    Step(isize),
    Move(Direction),
    Commit,
    Cancel,
    Act(WindowAction),
}

impl Intent {
    pub fn swallows(self) -> bool {
        self != Intent::PassThrough
    }
}

/// Whether the event should be kept from reaching applications.
///
/// Modifier changes are **never** swallowed: the ⌘ press already reached the app; swallowing its release would make the
/// app think ⌘ is still held, turning the next click into a ⌘-click.
pub fn should_swallow(event: KeyEvent, intent: Intent) -> bool {
    !matches!(event, KeyEvent::FlagsChanged { .. }) && intent.swallows()
}

#[derive(Debug, Default)]
pub struct KeyRouter {
    swallowed_downs: Vec<u16>,
    panel_keys: PanelKeys,
}

impl KeyRouter {
    pub fn set_panel_keys(&mut self, keys: PanelKeys) {
        self.panel_keys = keys;
    }

    pub fn route(&mut self, event: KeyEvent, hotkeys: &[Hotkey], active: Option<usize>) -> Intent {
        let intent = interpret(event, hotkeys, &self.panel_keys, active);
        self.track_swallowed(event, intent)
    }

    fn track_swallowed(&mut self, event: KeyEvent, intent: Intent) -> Intent {
        match event {
            KeyEvent::Down { code, .. } if intent.swallows() => {
                if !self.swallowed_downs.contains(&code) {
                    self.swallowed_downs.push(code);
                }
                intent
            }
            KeyEvent::Up { code } if self.swallowed_downs.contains(&code) => {
                self.swallowed_downs.retain(|&c| c != code);
                Intent::Swallow
            }
            _ => intent,
        }
    }
}

fn interpret(event: KeyEvent, hotkeys: &[Hotkey], panel: &PanelKeys, active: Option<usize>) -> Intent {
    match (event, active) {
        (KeyEvent::Down { code, mods }, None) => trigger_for(code, mods, hotkeys),
        (KeyEvent::Down { code, mods }, Some(current)) => while_active(code, mods, hotkeys, panel, current),
        (KeyEvent::Up { .. }, Some(_)) => Intent::Swallow,
        (KeyEvent::Up { .. }, None) => Intent::PassThrough,
        (KeyEvent::FlagsChanged { mods }, Some(current)) => on_flags(mods, &hotkeys[current]),
        (KeyEvent::FlagsChanged { .. }, None) => Intent::PassThrough,
    }
}

fn trigger_for(code: u16, mods: Modifiers, hotkeys: &[Hotkey]) -> Intent {
    match hotkeys.iter().position(|h| h.matches(code, mods)) {
        Some(hotkey) => Intent::Trigger { hotkey, reverse: mods.has(Modifier::Shift) },
        None => Intent::PassThrough,
    }
}

fn while_active(code: u16, mods: Modifiers, hotkeys: &[Hotkey], panel: &PanelKeys, current: usize) -> Intent {
    if hotkeys[current].matches(code, mods) {
        return Intent::Step(if mods.has(Modifier::Shift) { -1 } else { 1 });
    }
    if let Intent::Trigger { hotkey, reverse } = trigger_for(code, mods, hotkeys) {
        return Intent::Trigger { hotkey, reverse };
    }
    navigation_key(code, panel)
}

/// Other keys while the panel is open. Unknown keys are swallowed too, so they are not typed into the app behind the
/// panel.
fn navigation_key(code: u16, panel: &PanelKeys) -> Intent {
    match code {
        keys::RIGHT_ARROW => Intent::Move(Direction::Right),
        keys::LEFT_ARROW => Intent::Move(Direction::Left),
        keys::DOWN_ARROW => Intent::Move(Direction::Down),
        keys::UP_ARROW => Intent::Move(Direction::Up),
        keys::RETURN | keys::KEYPAD_ENTER => Intent::Commit,
        keys::ESCAPE => Intent::Cancel,
        _ => panel.action_for(code).map_or(Intent::Swallow, Intent::Act),
    }
}

fn on_flags(mods: Modifiers, hotkey: &Hotkey) -> Intent {
    if hotkey.is_held(mods) { Intent::Swallow } else { Intent::Commit }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GRAVE: u16 = keys::ANSI_GRAVE;

    fn mods(list: &[Modifier]) -> Modifiers {
        Modifiers::from_list(list)
    }

    fn hotkeys() -> Vec<Hotkey> {
        vec![
            Hotkey { modifiers: mods(&[Modifier::Cmd]), key_code: keys::TAB },
            Hotkey { modifiers: mods(&[Modifier::Cmd]), key_code: GRAVE },
            Hotkey { modifiers: mods(&[Modifier::Alt]), key_code: keys::TAB },
        ]
    }

    fn down(code: u16, list: &[Modifier]) -> KeyEvent {
        KeyEvent::Down { code, mods: mods(list) }
    }

    #[test]
    fn unrelated_key_passes_through_when_idle() {
        let mut r = KeyRouter::default();
        assert_eq!(r.route(down(0, &[Modifier::Cmd]), &hotkeys(), None), Intent::PassThrough);
    }

    #[test]
    fn cmd_tab_triggers_first_hotkey() {
        let mut r = KeyRouter::default();
        let intent = r.route(down(keys::TAB, &[Modifier::Cmd]), &hotkeys(), None);
        assert_eq!(intent, Intent::Trigger { hotkey: 0, reverse: false });
    }

    #[test]
    fn alt_shift_tab_triggers_reverse() {
        let mut r = KeyRouter::default();
        let intent = r.route(down(keys::TAB, &[Modifier::Alt, Modifier::Shift]), &hotkeys(), None);
        assert_eq!(intent, Intent::Trigger { hotkey: 2, reverse: true });
    }

    #[test]
    fn repeated_tab_steps_and_shift_goes_back() {
        let mut r = KeyRouter::default();
        let hk = hotkeys();
        assert_eq!(r.route(down(keys::TAB, &[Modifier::Cmd]), &hk, Some(0)), Intent::Step(1));
        assert_eq!(r.route(down(keys::TAB, &[Modifier::Cmd, Modifier::Shift]), &hk, Some(0)), Intent::Step(-1));
    }

    #[test]
    fn switching_to_other_hotkey_while_active() {
        let mut r = KeyRouter::default();
        let intent = r.route(down(GRAVE, &[Modifier::Cmd]), &hotkeys(), Some(0));
        assert_eq!(intent, Intent::Trigger { hotkey: 1, reverse: false });
    }

    #[test]
    fn releasing_modifier_commits() {
        let mut r = KeyRouter::default();
        let event = KeyEvent::FlagsChanged { mods: Modifiers::NONE };
        assert_eq!(r.route(event, &hotkeys(), Some(0)), Intent::Commit);
    }

    #[test]
    fn letter_keys_trigger_window_actions_while_active() {
        let mut r = KeyRouter::default();
        let intent = r.route(down(keys::W, &[Modifier::Cmd]), &hotkeys(), Some(0));
        assert_eq!(intent, Intent::Act(WindowAction::Close));
    }

    #[test]
    fn modifier_events_are_never_swallowed() {
        let event = KeyEvent::FlagsChanged { mods: Modifiers::NONE };
        assert!(!should_swallow(event, Intent::Commit), "modifier-only events always reach other apps");
        assert!(
            should_swallow(down(keys::TAB, &[Modifier::Cmd]), Intent::Step(1)),
            "a ⌘Tab that steps the selection is swallowed"
        );
    }

    #[test]
    fn releasing_only_shift_keeps_panel_open() {
        let mut r = KeyRouter::default();
        let event = KeyEvent::FlagsChanged { mods: mods(&[Modifier::Cmd]) };
        assert_eq!(r.route(event, &hotkeys(), Some(0)), Intent::Swallow);
    }

    #[test]
    fn arrows_move_in_the_grid() {
        let mut r = KeyRouter::default();
        let intent = r.route(down(keys::DOWN_ARROW, &[Modifier::Cmd]), &hotkeys(), Some(0));
        assert_eq!(intent, Intent::Move(Direction::Down));
    }

    #[test]
    fn escape_cancels_and_other_keys_are_swallowed() {
        let mut r = KeyRouter::default();
        let hk = hotkeys();
        assert_eq!(r.route(down(keys::ESCAPE, &[Modifier::Cmd]), &hk, Some(0)), Intent::Cancel);
        assert_eq!(r.route(down(0, &[Modifier::Cmd]), &hk, Some(0)), Intent::Swallow);
    }

    #[test]
    fn key_up_of_swallowed_key_is_swallowed_after_session_ends() {
        let mut r = KeyRouter::default();
        let hk = hotkeys();
        r.route(down(keys::TAB, &[Modifier::Cmd]), &hk, None);
        assert_eq!(r.route(KeyEvent::Up { code: keys::TAB }, &hk, None), Intent::Swallow);
        assert_eq!(r.route(KeyEvent::Up { code: keys::TAB }, &hk, None), Intent::PassThrough);
    }
}
