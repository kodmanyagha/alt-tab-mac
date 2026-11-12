// SPDX-License-Identifier: GPL-3.0-or-later

use super::{launch_agent, service};
use crate::macos::{bundle, os_version};

pub fn is_enabled() -> bool {
    if os_version::at_least(13) { service::is_enabled() } else { launch_agent::is_enabled() }
}

pub fn set_enabled(enabled: bool) -> Result<(), String> {
    if bundle::app_path().is_none() {
        return Err("the app is not running from an .app bundle".into());
    }
    if os_version::at_least(13) { service::set_enabled(enabled) } else { launch_agent::set_enabled(enabled) }
}
