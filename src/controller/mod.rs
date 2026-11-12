// SPDX-License-Identifier: GPL-3.0-or-later

mod actions;
mod app_controller;
mod bindings;
mod context;
mod health;
mod keys;
mod lifecycle;
mod mouse;
mod session;
mod thumbnails;

pub use app_controller::{Services, install, previews_enabled};
pub use health::start_health_checks;
pub use keys::on_key;
pub use lifecycle::{
    current_config, on_snapshot_published, reload_config, report_self_window, take_over_system_shortcuts, update_config,
};
pub use mouse::{on_mouse_click, on_mouse_move};
pub use thumbnails::{on_thumbnail, start_periodic_refresh};
