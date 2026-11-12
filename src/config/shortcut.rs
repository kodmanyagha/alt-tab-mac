// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

use crate::core::hotkey::Modifier;
use crate::core::scope::Scope;
use crate::core::style::Style;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeySpec {
    Named(NamedKey),
    Code(u16),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamedKey {
    Tab,
    AboveTab,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShortcutConfig {
    #[serde(default = "enabled_by_default")]
    pub enabled: bool,
    pub modifiers: Vec<Modifier>,
    pub key: KeySpec,
    #[serde(default)]
    pub style: Style,
    #[serde(default)]
    pub scope: Scope,
}

fn enabled_by_default() -> bool {
    true
}
