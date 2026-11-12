// SPDX-License-Identifier: GPL-3.0-or-later

mod binding;
pub mod own_window;
pub mod recorder;
mod settings_window;
pub mod snapshot;
mod tabs;
mod widgets;
mod window;

pub use settings_window::{open, rebuild};
