// SPDX-License-Identifier: GPL-3.0-or-later

use std::sync::OnceLock;

use objc2_foundation::NSProcessInfo;

/// Major macOS version (e.g. 14 = Sonoma).
pub fn major() -> isize {
    static CELL: OnceLock<isize> = OnceLock::new();
    *CELL.get_or_init(|| NSProcessInfo::processInfo().operatingSystemVersion().majorVersion)
}

pub fn at_least(major_version: isize) -> bool {
    major() >= major_version
}
