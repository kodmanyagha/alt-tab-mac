// SPDX-License-Identifier: GPL-3.0-or-later

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    fn IsSecureEventInputEnabled() -> u8;
}

pub fn is_enabled() -> bool {
    // SAFETY: `IsSecureEventInputEnabled` has no preconditions.
    unsafe { IsSecureEventInputEnabled() != 0 }
}
