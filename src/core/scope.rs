// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppScope {
    All,
    Current,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpaceScope {
    All,
    Current,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScreenScope {
    All,
    Current,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScreenReference {
    ActiveWindow,
    Cursor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Show,
    Hide,
    ShowAtEnd,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Scope {
    pub apps: AppScope,
    pub spaces: SpaceScope,
    pub screens: ScreenScope,
    pub minimized: Visibility,
    pub hidden_apps: Visibility,
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            apps: AppScope::All,
            spaces: SpaceScope::All,
            screens: ScreenScope::All,
            minimized: Visibility::ShowAtEnd,
            hidden_apps: Visibility::ShowAtEnd,
        }
    }
}
