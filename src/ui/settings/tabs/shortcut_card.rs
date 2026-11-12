// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::NSView;

use crate::config::Config;
use crate::config::shortcut::ShortcutConfig;
use crate::controller::update_config;
use crate::core::hotkey::Modifiers;
use crate::core::scope::{AppScope, ScreenScope, SpaceScope, Visibility};
use crate::core::style::Style;
use crate::i18n::{Key, Template, t, tf};
use crate::macos::key_names::shortcut_label;
use crate::macos::keyboard::resolve_key;
use crate::ui::settings::recorder::{self, Target};
use crate::ui::settings::widgets::{
    button, button_with_sender, checkbox, choice_field, field, fieldset, hstack, two_columns,
};

const STYLES: [(Style, Key); 4] = [
    (Style::Icons, Key::StyleIcons),
    (Style::Previews, Key::StylePreviews),
    (Style::PreviewsWithIcon, Key::StylePreviewsWithIcon),
    (Style::List, Key::StyleList),
];
const APPS: [(AppScope, Key); 2] = [(AppScope::All, Key::AllApps), (AppScope::Current, Key::CurrentApp)];
const SPACES: [(SpaceScope, Key); 2] = [(SpaceScope::All, Key::AllSpaces), (SpaceScope::Current, Key::CurrentSpace)];
const SCREENS: [(ScreenScope, Key); 2] =
    [(ScreenScope::All, Key::AllScreens), (ScreenScope::Current, Key::CurrentScreen)];
const VISIBILITY: [(Visibility, Key); 3] =
    [(Visibility::Show, Key::Show), (Visibility::ShowAtEnd, Key::ShowAtEnd), (Visibility::Hide, Key::Hide)];

pub fn build(mtm: MainThreadMarker, index: usize, s: &ShortcutConfig) -> Retained<NSView> {
    let header = header_row(mtm, index, s);
    let keys = keys_field(mtm, index, s);
    let style = choice_field(mtm, Key::Style, &STYLES, s.style, move |v| update_shortcut(index, |s| s.style = v));
    let [apps, spaces, screens, minimized, hidden] = scope_fields(mtm, index, s);
    let columns = two_columns(mtm, &[&keys, &style, &minimized, &hidden], &[&apps, &spaces, &screens]);
    fieldset(mtm, &tf(Template::ShortcutTitle, index + 1), &[&header, &columns])
}

/// The Enabled checkbox and the "Delete" button.
fn header_row(mtm: MainThreadMarker, i: usize, s: &ShortcutConfig) -> Retained<NSView> {
    let enabled = checkbox(mtm, t(Key::Enabled), s.enabled, move |v| update_shortcut(i, |s| s.enabled = v));
    let delete = button(mtm, t(Key::Delete), move || {
        update_config(|c| {
            c.shortcuts.remove(i);
        });
        crate::ui::settings::rebuild();
    });
    Retained::into_super(hstack(mtm, &[&enabled, &delete]))
}

/// Shortcut recorder: click it, then press the new combination.
fn keys_field(mtm: MainThreadMarker, i: usize, s: &ShortcutConfig) -> Retained<NSView> {
    let recorder = button_with_sender(mtm, &label_of(s), move |b| recorder::start(Target::Shortcut(i), b));
    field(mtm, t(Key::Keys), &recorder)
}

fn scope_fields(mtm: MainThreadMarker, i: usize, s: &ShortcutConfig) -> [Retained<NSView>; 5] {
    let sc = &s.scope;
    [
        choice_field(mtm, Key::Apps, &APPS, sc.apps, move |v| update_shortcut(i, |s| s.scope.apps = v)),
        choice_field(mtm, Key::Spaces, &SPACES, sc.spaces, move |v| update_shortcut(i, |s| s.scope.spaces = v)),
        choice_field(mtm, Key::Screens, &SCREENS, sc.screens, move |v| update_shortcut(i, |s| s.scope.screens = v)),
        choice_field(mtm, Key::MinimizedWindows, &VISIBILITY, sc.minimized, move |v| {
            update_shortcut(i, |s| s.scope.minimized = v)
        }),
        choice_field(mtm, Key::HiddenApps, &VISIBILITY, sc.hidden_apps, move |v| {
            update_shortcut(i, |s| s.scope.hidden_apps = v)
        }),
    ]
}

fn update_shortcut(index: usize, change: impl FnOnce(&mut ShortcutConfig)) {
    update_config(|c: &mut Config| {
        if let Some(s) = c.shortcuts.get_mut(index) {
            change(s);
        }
    });
}

fn label_of(s: &ShortcutConfig) -> String {
    shortcut_label(Modifiers::from_list(&s.modifiers), resolve_key(s.key))
}
