// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt::Display;

use super::key::Key;
use super::language::{Lang, current};
use super::template::Template;
use super::{en, tr};

pub fn t(key: Key) -> &'static str {
    match current() {
        Lang::Turkish => tr::text(key),
        Lang::English => en::text(key),
    }
}

/// Fills a template text with `value` in the current language.
pub fn tf(template: Template, value: impl Display) -> String {
    let value = value.to_string();
    match current() {
        Lang::Turkish => tr::template(template, &value),
        Lang::English => en::template(template, &value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn templates_are_filled_in_each_language() {
        assert_eq!(en::template(Template::ShortcutTitle, "3"), "Shortcut 3");
        assert_eq!(tr::template(Template::ShortcutTitle, "3"), "Kısayol 3");
    }
}
