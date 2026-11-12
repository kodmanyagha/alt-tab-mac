// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

use super::defaults;
use super::general::General;
use super::shortcut::ShortcutConfig;
use crate::core::actions::PanelKeys;
use crate::core::appearance::Appearance;
use crate::core::window_kind::WindowKinds;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub general: General,
    pub appearance: Appearance,
    pub window_kinds: WindowKinds,
    pub panel_keys: PanelKeys,
    pub blacklist: Vec<String>,
    pub shortcuts: Vec<ShortcutConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: General::default(),
            appearance: Appearance::default(),
            window_kinds: WindowKinds::default(),
            panel_keys: PanelKeys::default(),
            blacklist: Vec::new(),
            shortcuts: defaults::default_shortcuts(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::shortcut::KeySpec;
    use crate::core::panel_screen::PanelScreen;

    #[test]
    fn default_config_round_trips_through_toml() {
        let text = toml::to_string_pretty(&Config::default()).unwrap();
        let parsed: Config = toml::from_str(&text).unwrap();
        assert_eq!(parsed, Config::default());
    }

    #[test]
    fn named_panel_screen_round_trips() {
        let mut config = Config::default();
        config.general.panel_screen = PanelScreen::Named("DELL U2720Q".into());
        let text = toml::to_string_pretty(&config).unwrap();
        assert_eq!(toml::from_str::<Config>(&text).unwrap(), config);
    }

    #[test]
    fn key_code_can_be_a_number() {
        let text = "[[shortcuts]]\nmodifiers = [\"ctrl\"]\nkey = 49\n";
        let parsed: Config = toml::from_str(text).unwrap();
        assert_eq!(parsed.shortcuts[0].key, KeySpec::Code(49));
    }
}
