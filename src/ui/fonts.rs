// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSFont, NSFontManager, NSFontTraitMask};
use objc2_foundation::NSString;

const REGULAR_WEIGHT: isize = 5;

/// The system font if the family name is empty or not found.
pub fn font(family: &str, size: f64) -> Retained<NSFont> {
    family_font(family, size).unwrap_or_else(|| NSFont::systemFontOfSize(size))
}

fn family_font(family: &str, size: f64) -> Option<Retained<NSFont>> {
    if family.is_empty() {
        return None;
    }
    let mtm = MainThreadMarker::new()?;
    NSFontManager::sharedFontManager(mtm).fontWithFamily_traits_weight_size(
        &NSString::from_str(family),
        NSFontTraitMask::empty(),
        REGULAR_WEIGHT,
        size,
    )
}

/// Font families installed on the system (alphabetical).
pub fn families(mtm: MainThreadMarker) -> Vec<String> {
    let mut names: Vec<String> =
        NSFontManager::sharedFontManager(mtm).availableFontFamilies().iter().map(|n| n.to_string()).collect();
    names.sort_by_key(|n| n.to_lowercase());
    names
}
