// SPDX-License-Identifier: GPL-3.0-or-later

use std::ffi::c_void;

use super::ffi::keyboard::LMGetKbdType;
use crate::core::hotkey::Modifiers;
use crate::core::key_names::{modifier_symbols, special_key_name};

type TISInputSourceRef = *const c_void;
type CFDataRef = *const c_void;

const KEY_ACTION_DISPLAY: u16 = 3;
const NO_DEAD_KEYS: u32 = 1;

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    static kTISPropertyUnicodeKeyLayoutData: *const c_void;
    fn TISCopyCurrentKeyboardLayoutInputSource() -> TISInputSourceRef;
    fn TISGetInputSourceProperty(source: TISInputSourceRef, key: *const c_void) -> *const c_void;
    fn UCKeyTranslate(
        layout: *const c_void,
        virtual_key_code: u16,
        key_action: u16,
        modifier_key_state: u32,
        keyboard_type: u32,
        options: u32,
        dead_key_state: *mut u32,
        max_length: usize,
        actual_length: *mut usize,
        unicode: *mut u16,
    ) -> i32;
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFDataGetBytePtr(data: CFDataRef) -> *const u8;
    fn CFRelease(cf: *const c_void);
}

/// E.g. `⌘ ⇥`, `⌥ "`
pub fn shortcut_label(mods: Modifiers, code: u16) -> String {
    let symbols = modifier_symbols(mods);
    if symbols.is_empty() { key_name(code) } else { format!("{symbols} {}", key_name(code)) }
}

pub fn key_name(code: u16) -> String {
    special_key_name(code).map(str::to_string).or_else(|| translate(code)).unwrap_or_else(|| format!("#{code}"))
}

fn translate(code: u16) -> Option<String> {
    // SAFETY: The input source is checked for null, used only while retained and released exactly once.
    unsafe {
        let source = TISCopyCurrentKeyboardLayoutInputSource();
        if source.is_null() {
            return None;
        }
        let result = translate_with(source, code);
        CFRelease(source);
        result
    }
}

/// Name of the key with virtual key `code` in the given keyboard layout.
///
/// # Safety
/// `source` must be a valid, retained `TISInputSourceRef` for the whole call.
unsafe fn translate_with(source: TISInputSourceRef, code: u16) -> Option<String> {
    // SAFETY: `source` is a valid input source, as required by the caller.
    let data = unsafe { TISGetInputSourceProperty(source, kTISPropertyUnicodeKeyLayoutData) };
    if data.is_null() {
        return None;
    }
    // SAFETY: `data` was checked for null and is a CFData owned by the input source.
    let layout = unsafe { CFDataGetBytePtr(data) } as *const c_void;
    let mut dead_keys = 0u32;
    let mut buffer = [0u16; 4];
    let mut length = 0usize;
    // SAFETY: `layout` points to the keyboard layout data; every out pointer refers to a live local and `buffer.len()` is its real capacity.
    let status = unsafe {
        UCKeyTranslate(
            layout,
            code,
            KEY_ACTION_DISPLAY,
            0,
            LMGetKbdType() as u32,
            NO_DEAD_KEYS,
            &mut dead_keys,
            buffer.len(),
            &mut length,
            buffer.as_mut_ptr(),
        )
    };
    let text = String::from_utf16_lossy(&buffer[..length]).trim().to_uppercase();
    (status == 0 && !text.is_empty()).then_some(text)
}
