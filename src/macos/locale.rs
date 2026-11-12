// SPDX-License-Identifier: GPL-3.0-or-later

use objc2_foundation::NSLocale;

/// E.g. "tr-TR", "en-US": the first language in the preference list.
pub fn preferred_language() -> Option<String> {
    NSLocale::preferredLanguages().iter().next().map(|l| l.to_string())
}
