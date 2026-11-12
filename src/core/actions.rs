// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

use super::keys;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowAction {
    Close,
    Minimize,
    QuitApp,
    HideApp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct PanelKeys {
    pub close: u16,
    pub minimize: u16,
    pub quit: u16,
    pub hide: u16,
}

impl Default for PanelKeys {
    fn default() -> Self {
        Self { close: keys::W, minimize: keys::M, quit: keys::Q, hide: keys::H }
    }
}

impl PanelKeys {
    pub fn action_for(&self, code: u16) -> Option<WindowAction> {
        [
            (self.close, WindowAction::Close),
            (self.minimize, WindowAction::Minimize),
            (self.quit, WindowAction::QuitApp),
            (self.hide, WindowAction::HideApp),
        ]
        .into_iter()
        .find(|&(key, _)| key == code)
        .map(|(_, action)| action)
    }
}

impl WindowAction {
    /// Whether the window should leave the list after the action.
    pub fn removes_window(self) -> bool {
        matches!(self, Self::Close | Self::QuitApp)
    }
}
