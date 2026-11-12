// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::NSView;

use super::donations;
use crate::i18n::{Key, Template, t, tf};
use crate::ui::settings::widgets::{fieldset, label, note, two_columns, vstack};

pub fn build(mtm: MainThreadMarker) -> Retained<NSView> {
    let app = app_fieldset(mtm);
    let license = license_fieldset(mtm);
    let donations = donations::donations_fieldset(mtm);
    let top = two_columns(mtm, &[&app], &[&license]);
    Retained::into_super(vstack(mtm, &[&top, &donations], 16.0))
}

fn app_fieldset(mtm: MainThreadMarker) -> Retained<NSView> {
    let version = label(mtm, &tf(Template::Version, env!("CARGO_PKG_VERSION")));
    let description = note(mtm, t(Key::AppDescription));
    let config = note(mtm, &tf(Template::ConfigFile, crate::config::config_path().display()));
    fieldset(mtm, "alt-tab-mac", &[&version, &description, &config])
}

fn license_fieldset(mtm: MainThreadMarker) -> Retained<NSView> {
    let text = note(mtm, t(Key::LicenseText));
    fieldset(mtm, t(Key::License), &[&text])
}
