// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::NSView;

use super::shortcut_card;
use crate::config::Config;
use crate::config::shortcut::{KeySpec, NamedKey, ShortcutConfig};
use crate::controller::update_config;
use crate::core::actions::PanelKeys;
use crate::core::hotkey::Modifier;
use crate::core::scope::Scope;
use crate::core::style::Style;
use crate::i18n::{Key, t};
use crate::macos::key_names::key_name;
use crate::ui::settings::recorder::{self, Target};
use crate::ui::settings::widgets::{button, button_with_sender, field, fieldset, note, scroll, two_columns, vstack};

type PanelKeyField = fn(&mut PanelKeys) -> &mut u16;

const PANEL_KEYS: [(Key, PanelKeyField); 4] = [
    (Key::CloseWindow, |k| &mut k.close),
    (Key::Minimize, |k| &mut k.minimize),
    (Key::QuitApp, |k| &mut k.quit),
    (Key::HideApp, |k| &mut k.hide),
];

pub fn build(mtm: MainThreadMarker, config: &Config) -> Retained<NSView> {
    let intro = note(mtm, t(Key::ShortcutsIntro));
    let cards = shortcut_list(mtm, &config.shortcuts);
    let add = button(mtm, t(Key::AddShortcut), add_shortcut);
    let panel_keys = panel_keys_fieldset(mtm, &config.panel_keys);
    let content = vstack(mtm, &[&intro, &cards, &add, &panel_keys], 16.0);
    Retained::into_super(scroll(mtm, &content))
}

/// Shortcut boxes, full width, one below the other.
fn shortcut_list(mtm: MainThreadMarker, shortcuts: &[ShortcutConfig]) -> Retained<NSView> {
    let cards: Vec<Retained<NSView>> =
        shortcuts.iter().enumerate().map(|(i, s)| shortcut_card::build(mtm, i, s)).collect();
    let refs: Vec<&NSView> = cards.iter().map(|v| &**v).collect();
    Retained::into_super(vstack(mtm, &refs, 16.0))
}

fn add_shortcut() {
    update_config(|c| {
        c.shortcuts.push(ShortcutConfig {
            enabled: true,
            modifiers: vec![Modifier::Ctrl],
            key: KeySpec::Named(NamedKey::Tab),
            style: Style::default(),
            scope: Scope::default(),
        })
    });
    crate::ui::settings::rebuild();
}

fn panel_keys_fieldset(mtm: MainThreadMarker, keys: &PanelKeys) -> Retained<NSView> {
    let fields: Vec<Retained<NSView>> = PANEL_KEYS
        .into_iter()
        .map(|(name, accessor)| {
            let current = *accessor(&mut keys.clone());
            let recorder =
                button_with_sender(mtm, &key_name(current), move |b| recorder::start(Target::PanelKey(accessor), b));
            field(mtm, t(name), &recorder)
        })
        .collect();
    let left: Vec<&NSView> = fields.iter().take(2).map(|v| &**v).collect();
    let right: Vec<&NSView> = fields.iter().skip(2).map(|v| &**v).collect();
    let grid = two_columns(mtm, &left, &right);
    fieldset(mtm, t(Key::PanelKeys), &[&grid])
}
