// SPDX-License-Identifier: GPL-3.0-or-later

mod en;
mod key;
mod language;
mod template;
mod tr;
mod translate;

pub use key::Key;
pub use language::{LanguageSetting, apply, apply_for_tools};
pub use template::Template;
pub use translate::{t, tf};
