// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::NSView;

use crate::config::Config;
use crate::controller::update_config;
use crate::core::window_kind::WindowKinds;
use crate::i18n::{Key, t};
use crate::macos::workspace::{self, AppDesc};
use crate::ui::settings::widgets::{button, checkbox, fieldset, hstack, label, note, popup_field, two_columns};

pub fn build(mtm: MainThreadMarker, config: &Config) -> Retained<NSView> {
    let kinds = kinds_fieldset(mtm, config.window_kinds);
    let blacklist = blacklist_fieldset(mtm, &config.blacklist);
    Retained::into_super(two_columns(mtm, &[&kinds], &[&blacklist]))
}

fn kinds_fieldset(mtm: MainThreadMarker, kinds: WindowKinds) -> Retained<NSView> {
    let dialogs = checkbox(mtm, t(Key::Dialogs), kinds.dialogs, |v| update_config(|c| c.window_kinds.dialogs = v));
    let dialogs_note = note(mtm, t(Key::DialogsNote));
    let modal = checkbox(mtm, t(Key::ModalDialogs), kinds.modal_dialogs, |v| {
        update_config(|c| c.window_kinds.modal_dialogs = v)
    });
    let modal_note = note(mtm, t(Key::ModalDialogsNote));
    let panels = checkbox(mtm, t(Key::FloatingPanels), kinds.floating_panels, |v| {
        update_config(|c| c.window_kinds.floating_panels = v)
    });
    let panels_note = note(mtm, t(Key::FloatingPanelsNote));
    let sheet_note = note(mtm, t(Key::SheetsNote));
    let items: [&NSView; 7] = [&dialogs, &dialogs_note, &modal, &modal_note, &panels, &panels_note, &sheet_note];
    fieldset(mtm, t(Key::WindowKinds), &items)
}

fn blacklist_fieldset(mtm: MainThreadMarker, blacklist: &[String]) -> Retained<NSView> {
    let running = workspace::running_apps();
    let mut views: Vec<Retained<NSView>> =
        vec![Retained::into_super(Retained::into_super(note(mtm, t(Key::BlacklistNote))))];
    for bundle_id in blacklist {
        views.push(blacklist_row(mtm, bundle_id, &running));
    }
    views.push(add_field(mtm, blacklist, running));
    let refs: Vec<&NSView> = views.iter().map(|v| &**v).collect();
    fieldset(mtm, t(Key::Blacklist), &refs)
}

fn blacklist_row(mtm: MainThreadMarker, bundle_id: &str, running: &[AppDesc]) -> Retained<NSView> {
    let id = bundle_id.to_string();
    let remove = button(mtm, t(Key::Remove), move || {
        update_config(|c| c.blacklist.retain(|b| b != &id));
        crate::ui::settings::rebuild();
    });
    let name = display_name(bundle_id, running);
    Retained::into_super(hstack(mtm, &[&remove, &label(mtm, &name)]))
}

/// Adds an app picked from the running apps that are not in the list yet.
fn add_field(mtm: MainThreadMarker, blacklist: &[String], running: Vec<AppDesc>) -> Retained<NSView> {
    let candidates: Vec<AppDesc> =
        running.into_iter().filter(|a| !a.bundle_id.is_empty() && !blacklist.contains(&a.bundle_id)).collect();
    let mut options = vec![t(Key::ChooseApp).to_string()];
    options.extend(candidates.iter().map(|a| a.name.clone()));
    popup_field(mtm, t(Key::AddApp), &options, 0, move |i| {
        if let Some(app) = i.checked_sub(1).and_then(|i| candidates.get(i)) {
            let id = app.bundle_id.clone();
            update_config(|c| c.blacklist.push(id));
            crate::ui::settings::rebuild();
        }
    })
}

fn display_name(bundle_id: &str, running: &[AppDesc]) -> String {
    match running.iter().find(|a| a.bundle_id == bundle_id) {
        Some(app) => format!("{} ({bundle_id})", app.name),
        None => bundle_id.to_string(),
    }
}
