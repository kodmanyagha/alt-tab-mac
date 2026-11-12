// SPDX-License-Identifier: GPL-3.0-or-later

use objc2_core_foundation::{CFBoolean, CFDictionary, CFString, CFType};

use super::ffi::ax::{AXIsProcessTrusted, AXIsProcessTrustedWithOptions, kAXTrustedCheckOptionPrompt};

pub fn is_trusted() -> bool {
    // SAFETY: `AXIsProcessTrusted` has no preconditions.
    unsafe { AXIsProcessTrusted() }
}

/// Opens the system "grant Accessibility permission" prompt if permission is missing.
pub fn request() -> bool {
    // SAFETY: `kAXTrustedCheckOptionPrompt` is a non-null CFStringRef constant with the layout of `CFString`.
    let key: &CFString = unsafe { &*(kAXTrustedCheckOptionPrompt as *const CFString) };
    let value: &CFType = CFBoolean::new(true);
    let options = CFDictionary::<CFString, CFType>::from_slices(&[key], &[value]);
    // SAFETY: `options` is a live CFDictionary for the duration of the call.
    unsafe { AXIsProcessTrustedWithOptions(&*options as *const _ as *const std::ffi::c_void) }
}
