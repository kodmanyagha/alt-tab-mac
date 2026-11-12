// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell;

use dispatch2::DispatchQueue;
use objc2::rc::Retained;
use objc2_app_kit::NSButton;
use objc2_foundation::NSString;

use crate::config::shortcut::{KeySpec, NamedKey};
use crate::controller::update_config;
use crate::core::actions::PanelKeys;
use crate::core::hotkey::{Modifier, Modifiers};
use crate::core::input::KeyEvent;
use crate::core::keys;
use crate::i18n::{Key, t};

#[derive(Clone, Copy, Debug)]
pub enum Target {
    Shortcut(usize),
    PanelKey(fn(&mut PanelKeys) -> &mut u16),
}

struct Recording {
    target: Target,
    button: Retained<NSButton>,
    previous_title: Retained<NSString>,
}

thread_local! {
    static RECORDING: RefCell<Option<Recording>> = const { RefCell::new(None) };
}

pub fn start(target: Target, button: &NSButton) {
    use objc2::Message;

    cancel();
    let previous_title = button.title();
    button.setTitle(&NSString::from_str(t(Key::PressKeys)));
    let button = button.retain();
    RECORDING.set(Some(Recording { target, button, previous_title }));
}

pub fn is_recording() -> bool {
    RECORDING.with_borrow(|r| r.is_some())
}

/// Called from the event tap. Returns `true` to swallow the event. Modifier events are not swallowed (see
/// `core::input::should_swallow`).
pub fn capture(event: KeyEvent) -> bool {
    let KeyEvent::Down { code, mods } = event else {
        return matches!(event, KeyEvent::Up { .. });
    };
    if code == keys::ESCAPE && mods == Modifiers::NONE {
        cancel();
        return true;
    }
    let Some(target) = RECORDING.with_borrow(|r| r.as_ref().map(|r| r.target)) else {
        return false;
    };
    if accept(target, code, mods) {
        RECORDING.set(None);
        DispatchQueue::main().exec_async(move || save(target, code, mods));
    }
    true
}

fn cancel() {
    if let Some(r) = RECORDING.take() {
        r.button.setTitle(&r.previous_title);
    }
}

/// Shortcuts need at least one modifier other than Shift; panel keys need none.
fn accept(target: Target, _code: u16, mods: Modifiers) -> bool {
    match target {
        Target::Shortcut(_) => mods.without(Modifier::Shift) != Modifiers::NONE,
        Target::PanelKey(_) => mods == Modifiers::NONE,
    }
}

fn save(target: Target, code: u16, mods: Modifiers) {
    update_config(|c| match target {
        Target::Shortcut(i) => {
            if let Some(s) = c.shortcuts.get_mut(i) {
                s.modifiers = modifier_list(mods);
                s.key = key_spec(code);
            }
        }
        Target::PanelKey(field) => *field(&mut c.panel_keys) = code,
    });
    super::rebuild();
}

fn modifier_list(mods: Modifiers) -> Vec<Modifier> {
    [Modifier::Ctrl, Modifier::Alt, Modifier::Shift, Modifier::Cmd].into_iter().filter(|m| mods.has(*m)).collect()
}

fn key_spec(code: u16) -> KeySpec {
    if code == keys::TAB { KeySpec::Named(NamedKey::Tab) } else { KeySpec::Code(code) }
}
