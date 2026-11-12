// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::NSView;

use crate::config::Config;
use crate::controller::update_config;
use crate::core::appearance::{Appearance, IconSize, ItemSize, Orientation, SortOrder, Theme};
use crate::i18n::{Key, t};
use crate::ui::fonts;
use crate::ui::settings::widgets::{choice_field, choice_field_raw, fieldset, note, popup_field, two_columns};

const THEMES: [(Theme, Key); 3] =
    [(Theme::System, Key::ThemeSystem), (Theme::Light, Key::ThemeLight), (Theme::Dark, Key::ThemeDark)];
const ORIENTATIONS: [(Orientation, Key); 2] =
    [(Orientation::Horizontal, Key::OrientationHorizontal), (Orientation::Vertical, Key::OrientationVertical)];
const SORTS: [(SortOrder, Key); 3] = [
    (SortOrder::RecentlyUsed, Key::SortRecent),
    (SortOrder::AppName, Key::SortAppName),
    (SortOrder::GroupedByApp, Key::SortGrouped),
];
const SIZES: [(ItemSize, Key); 3] =
    [(ItemSize::Small, Key::SizeSmall), (ItemSize::Medium, Key::SizeMedium), (ItemSize::Large, Key::SizeLarge)];
const ICON_SIZES: [(IconSize, Key); 4] = [
    (IconSize::Small, Key::SizeSmall),
    (IconSize::Medium, Key::SizeMedium),
    (IconSize::Large, Key::SizeLarge),
    (IconSize::ExtraLarge, Key::SizeExtraLarge),
];
const TITLE_LINES: [(u8, Key); 3] = [(1, Key::OneLine), (2, Key::TwoLines), (3, Key::ThreeLines)];
const PADDINGS: [f64; 6] = [2.0, 4.0, 8.0, 12.0, 16.0, 24.0];
const SPACINGS: [f64; 6] = [0.0, 4.0, 8.0, 12.0, 16.0, 24.0];
const FONT_SIZES: [f64; 8] = [10.0, 11.0, 12.0, 13.0, 14.0, 16.0, 18.0, 20.0];

pub fn build(mtm: MainThreadMarker, config: &Config) -> Retained<NSView> {
    let a = &config.appearance;
    let panel = panel_fieldset(mtm, a);
    let text = text_fieldset(mtm, a);
    let items = items_fieldset(mtm, a);
    Retained::into_super(two_columns(mtm, &[&panel, &text], &[&items]))
}

fn panel_fieldset(mtm: MainThreadMarker, a: &Appearance) -> Retained<NSView> {
    let theme = choice_field(mtm, Key::Theme, &THEMES, a.theme, |v| update_config(|c| c.appearance.theme = v));
    let orientation = choice_field(mtm, Key::Orientation, &ORIENTATIONS, a.orientation, |v| {
        update_config(|c| c.appearance.orientation = v)
    });
    let sort = choice_field(mtm, Key::Sort, &SORTS, a.sort, |v| update_config(|c| c.appearance.sort = v));
    fieldset(mtm, t(Key::Panel), &[&theme, &orientation, &sort])
}

fn items_fieldset(mtm: MainThreadMarker, a: &Appearance) -> Retained<NSView> {
    let size = choice_field(mtm, Key::ItemSize, &SIZES, a.size, |v| update_config(|c| c.appearance.size = v));
    let icon =
        choice_field(mtm, Key::IconSize, &ICON_SIZES, a.icon_size, |v| update_config(|c| c.appearance.icon_size = v));
    let padding = choice_field_raw(mtm, t(Key::ItemPadding), &points(&PADDINGS), a.item_padding, |v| {
        update_config(|c| c.appearance.item_padding = v)
    });
    let spacing = choice_field_raw(mtm, t(Key::ItemSpacing), &points(&SPACINGS), a.item_spacing, |v| {
        update_config(|c| c.appearance.item_spacing = v)
    });
    let style_note = note(mtm, t(Key::StyleNote));
    fieldset(mtm, t(Key::Items), &[&size, &icon, &padding, &spacing, &style_note])
}

fn text_fieldset(mtm: MainThreadMarker, a: &Appearance) -> Retained<NSView> {
    let family = font_family_field(mtm, &a.font_family);
    let sizes: Vec<(f64, String)> = FONT_SIZES.iter().map(|&s| (s, format!("{s}"))).collect();
    let size =
        choice_field_raw(mtm, t(Key::FontSize), &sizes, a.font_size, |v| update_config(|c| c.appearance.font_size = v));
    let lines = choice_field(mtm, Key::TitleLines, &TITLE_LINES, a.title_lines, |v| {
        update_config(|c| c.appearance.title_lines = v)
    });
    fieldset(mtm, t(Key::Text), &[&family, &size, &lines])
}

/// Options formatted as "12 pt".
fn points(values: &[f64]) -> Vec<(f64, String)> {
    values.iter().map(|&v| (v, format!("{v} pt"))).collect()
}

/// The first option is the system font, followed by the installed families.
fn font_family_field(mtm: MainThreadMarker, current: &str) -> Retained<NSView> {
    let families = fonts::families(mtm);
    let mut options = vec![t(Key::SystemFont).to_string()];
    options.extend(families.iter().cloned());
    let selected = families.iter().position(|f| f == current).map_or(0, |i| i + 1);
    popup_field(mtm, t(Key::FontFamily), &options, selected, move |i| {
        let family = i.checked_sub(1).and_then(|i| families.get(i)).cloned().unwrap_or_default();
        update_config(|c| c.appearance.font_family = family);
    })
}
