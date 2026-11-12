// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Template {
    ScreenName,
    ShortcutTitle,
    Version,
    ConfigFile,
}
