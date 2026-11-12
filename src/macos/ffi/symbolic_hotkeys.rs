// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::OnceLock;

use super::dl;

pub type SetSymbolicHotKeyEnabledFn = unsafe extern "C" fn(hotkey: i32, enabled: bool) -> i32;

pub const COMMAND_TAB: i32 = 1;
pub const COMMAND_SHIFT_TAB: i32 = 2;
pub const COMMAND_KEY_ABOVE_TAB: i32 = 6;

pub fn set_symbolic_hotkey_enabled() -> Option<SetSymbolicHotKeyEnabledFn> {
    static CELL: OnceLock<Option<SetSymbolicHotKeyEnabledFn>> = OnceLock::new();
    *CELL.get_or_init(load)
}

fn load() -> Option<SetSymbolicHotKeyEnabledFn> {
    let skylight = dl::open(dl::SKYLIGHT)?;
    // SAFETY: `SetSymbolicHotKeyEnabledFn` matches the C signature of both symbols.
    unsafe {
        dl::function(skylight, c"SLSSetSymbolicHotKeyEnabled")
            .or_else(|| dl::function(skylight, c"CGSSetSymbolicHotKeyEnabled"))
    }
}
