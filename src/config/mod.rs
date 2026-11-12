// SPDX-License-Identifier: GPL-3.0-or-later

mod app_config;
mod defaults;
mod file;
pub mod general;
pub mod shortcut;

pub use app_config::Config;
pub use file::{config_path, load_or_create, save};
