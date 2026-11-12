// SPDX-License-Identifier: GPL-3.0-or-later

use std::time::Duration;

use dispatch2::{DispatchQueue, DispatchTime};

use super::app_controller::{AppController, with_controller};
use crate::macos::{event_tap, permissions, secure_input};

const INTERVAL: Duration = Duration::from_secs(1);

pub fn start_health_checks() {
    let Ok(when) = DispatchTime::try_from(INTERVAL) else { return };
    let _ = DispatchQueue::main().after(when, || {
        tick();
        start_health_checks();
    });
}

fn tick() {
    if event_tap::ensure_enabled() {
        tracing::warn!("event tap was disabled, re-enabled");
    }
    let blocked = !permissions::is_trusted() || secure_input::is_enabled();
    with_controller(|c| c.set_takeover_blocked(blocked));
}

impl AppController {
    fn set_takeover_blocked(&mut self, blocked: bool) {
        if self.takeover_blocked == blocked {
            return;
        }
        self.takeover_blocked = blocked;
        tracing::info!(blocked, "system shortcut takeover changed");
        self.apply_system_shortcuts();
    }
}
