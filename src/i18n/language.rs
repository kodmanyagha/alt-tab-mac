// SPDX-License-Identifier: GPL-3.0-or-later

use std::env;
use std::sync::atomic::{AtomicU8, Ordering};

use serde::{Deserialize, Serialize};

use crate::macos::locale;

const TOOLS_LANGUAGE_VARIABLE: &str = "ALT_TAB_LANG";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lang {
    Turkish,
    English,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguageSetting {
    Turkish,
    English,
    #[default]
    System,
}

static CURRENT: AtomicU8 = AtomicU8::new(Lang::Turkish as u8);

pub fn set(lang: Lang) {
    CURRENT.store(lang as u8, Ordering::Relaxed);
}

pub fn current() -> Lang {
    if CURRENT.load(Ordering::Relaxed) == Lang::English as u8 { Lang::English } else { Lang::Turkish }
}

/// Activates the language for the setting. Returns `true` if the language changed.
pub fn apply(setting: LanguageSetting) -> bool {
    let lang = resolve(setting, locale::preferred_language().as_deref());
    let changed = lang != current();
    set(lang);
    changed
}

/// For developer tools: uses `ALT_TAB_LANG=tr|en` if set, otherwise the setting.
pub fn apply_for_tools(setting: LanguageSetting) {
    match env::var(TOOLS_LANGUAGE_VARIABLE).as_deref() {
        Ok("tr") => set(Lang::Turkish),
        Ok("en") => set(Lang::English),
        _ => {
            apply(setting);
        }
    }
}

/// Resolves the setting against the system's preferred language (e.g. "tr-TR").
pub fn resolve(setting: LanguageSetting, system_language: Option<&str>) -> Lang {
    match setting {
        LanguageSetting::Turkish => Lang::Turkish,
        LanguageSetting::English => Lang::English,
        LanguageSetting::System if system_language.is_some_and(|language| language.starts_with("tr")) => Lang::Turkish,
        LanguageSetting::System => Lang::English,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_turkish_resolves_to_turkish() {
        assert_eq!(resolve(LanguageSetting::System, Some("tr-TR")), Lang::Turkish);
        assert_eq!(resolve(LanguageSetting::System, Some("de-DE")), Lang::English);
        assert_eq!(resolve(LanguageSetting::System, None), Lang::English);
    }

    #[test]
    fn explicit_setting_wins_over_system_language() {
        assert_eq!(resolve(LanguageSetting::English, Some("tr-TR")), Lang::English);
        assert_eq!(resolve(LanguageSetting::Turkish, Some("en-US")), Lang::Turkish);
    }
}
