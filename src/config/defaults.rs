// SPDX-License-Identifier: GPL-3.0-or-later

use super::shortcut::{KeySpec, NamedKey, ShortcutConfig};
use crate::core::hotkey::Modifier;
use crate::core::scope::{AppScope, Scope, ScreenScope, SpaceScope};
use crate::core::style::Style;

pub fn default_shortcuts() -> Vec<ShortcutConfig> {
    vec![
        shortcut(Modifier::Cmd, NamedKey::Tab, AppScope::All, SpaceScope::All, ScreenScope::All),
        shortcut(Modifier::Cmd, NamedKey::AboveTab, AppScope::Current, SpaceScope::All, ScreenScope::All),
        shortcut(Modifier::Alt, NamedKey::Tab, AppScope::All, SpaceScope::Current, ScreenScope::Current),
        shortcut(Modifier::Alt, NamedKey::AboveTab, AppScope::Current, SpaceScope::Current, ScreenScope::Current),
    ]
}

fn shortcut(
    modifier: Modifier,
    key: NamedKey,
    apps: AppScope,
    spaces: SpaceScope,
    screens: ScreenScope,
) -> ShortcutConfig {
    ShortcutConfig {
        enabled: true,
        modifiers: vec![modifier],
        key: KeySpec::Named(key),
        style: Style::default(),
        scope: Scope { apps, spaces, screens, ..Scope::default() },
    }
}
