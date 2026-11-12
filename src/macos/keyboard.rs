// SPDX-License-Identifier: GPL-3.0-or-later

use super::ffi::keyboard::{KBGetLayoutType, LAYOUT_ISO, LMGetKbdType};
use crate::config::shortcut::{KeySpec, NamedKey};
use crate::core::keys;

pub fn resolve_key(spec: KeySpec) -> u16 {
    match spec {
        KeySpec::Code(code) => code,
        KeySpec::Named(NamedKey::Tab) => keys::TAB,
        KeySpec::Named(NamedKey::AboveTab) => above_tab_key_code(),
    }
}

/// The key above Tab has a different code on ISO and ANSI keyboards.
fn above_tab_key_code() -> u16 {
    if is_iso_keyboard() { keys::ISO_SECTION } else { keys::ANSI_GRAVE }
}

fn is_iso_keyboard() -> bool {
    // SAFETY: Both functions only read the current keyboard type and have no preconditions.
    let layout = unsafe { KBGetLayoutType(LMGetKbdType() as i16) };
    layout == LAYOUT_ISO
}
