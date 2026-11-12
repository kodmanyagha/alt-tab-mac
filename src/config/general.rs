// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

use crate::core::panel_screen::PanelScreen;
use crate::core::scope::ScreenReference;
use crate::i18n::LanguageSetting;
use crate::logging::LogLevel;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct General {
    pub show_delay_ms: u64,
    pub screen_reference: ScreenReference,
    pub panel_screen: PanelScreen,
    pub show_status_item: bool,
    pub mouse_selection: bool,
    pub language: LanguageSetting,
    pub refresh_interval_ms: u64,
    pub log_level: LogLevel,
}

impl Default for General {
    fn default() -> Self {
        Self {
            show_delay_ms: 0,
            screen_reference: ScreenReference::ActiveWindow,
            panel_screen: PanelScreen::default(),
            show_status_item: true,
            mouse_selection: true,
            language: LanguageSetting::default(),
            refresh_interval_ms: 3000,
            log_level: LogLevel::default(),
        }
    }
}
