// SPDX-License-Identifier: GPL-3.0-or-later

pub const LAYOUT_ISO: u32 = u32::from_be_bytes(*b"ISO ");

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {
    pub fn LMGetKbdType() -> u8;
    pub fn KBGetLayoutType(keyboard_type: i16) -> u32;
}
