// SPDX-License-Identifier: GPL-3.0-or-later

use super::app_controller::{AppController, with_controller};
use super::bindings;
use crate::config::{self, Config};
use crate::core::model::WindowInfo;
use crate::core::system_shortcuts::conflicts;
use crate::macos::system_shortcuts;
use crate::tracker::messages::Msg;
use crate::ui::status_item;
use objc2::MainThreadMarker;

/// "Reload config file" from the menu (after the file was edited by hand).
pub fn reload_config() {
    with_controller(|c| {
        c.config = config::load_or_create();
        c.apply_config();
        tracing::info!(shortcut_count = c.bindings.hotkeys.len(), "settings reloaded");
    });
}

/// From the settings window: apply the change, write it to disk and activate it immediately.
pub fn update_config(change: impl FnOnce(&mut Config)) {
    with_controller(|c| {
        change(&mut c.config);
        config::save(&c.config);
        c.apply_config();
    });
}

/// Current settings shown by the settings window.
pub fn current_config() -> Config {
    with_controller(|c| c.config.clone()).unwrap_or_default()
}

/// Our settings window was opened or closed (see `ui::settings::own_window`).
pub fn report_self_window(window: Option<WindowInfo>) {
    with_controller(|c| c.tracker.send(Msg::SelfWindow(window)));
}

/// Called after the event tap is installed. Disabling the system ⌘Tab without a working tap would leave the user with
/// no switcher at all.
pub fn take_over_system_shortcuts() {
    with_controller(|c| {
        c.owns_system_shortcuts = true;
        c.apply_system_shortcuts();
    });
}

/// When the tracker publishes a new list: preload icons, drop previews of dead windows, and request a preview if the
/// focus changed.
pub fn on_snapshot_published() {
    with_controller(|c| {
        let snapshot = c.store.current();
        c.icons.sync(snapshot.windows.iter().map(|w| w.pid));
        c.thumbnails.retain_only(snapshot.windows.iter().map(|w| w.id));
        c.capture_previous_focus(snapshot.focused_window);
    });
}

impl AppController {
    /// Applies every setting to the running system.
    pub(super) fn apply_config(&mut self) {
        if crate::i18n::apply(self.config.general.language)
            && let Some(mtm) = MainThreadMarker::new()
        {
            status_item::rebuild_menu(mtm);
        }
        crate::logging::set_level(self.config.general.log_level);
        self.bindings = bindings::build(&self.config.shortcuts);
        self.router.set_panel_keys(self.config.panel_keys);
        self.panel.set_theme(self.config.appearance.theme);
        self.panel.set_mouse_enabled(self.config.general.mouse_selection);
        status_item::set_visible(self.config.general.show_status_item);
        self.apply_system_shortcuts();
    }

    /// Takes over the system ⌘Tab while our shortcuts can work, and gives it back when they cannot (no permission,
    /// password field), so the user is never left without a switcher.
    pub(super) fn apply_system_shortcuts(&self) {
        if self.owns_system_shortcuts && !self.takeover_blocked {
            system_shortcuts::apply(&conflicts(&self.bindings.hotkeys));
        } else {
            system_shortcuts::restore();
        }
    }
}
