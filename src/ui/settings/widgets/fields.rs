// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::NSView;

use super::controls::{label, popup};
use super::layout::vstack;
use crate::i18n::{Key, t};

/// "Label" on top, `control` below at full width.
pub fn field(mtm: MainThreadMarker, title: &str, control: &NSView) -> Retained<NSView> {
    let caption = label(mtm, title);
    Retained::into_super(vstack(mtm, &[&caption, control], 4.0))
}

/// Popup field with a free-form option list.
pub fn popup_field(
    mtm: MainThreadMarker,
    title: &str,
    options: &[String],
    selected: usize,
    on_change: impl Fn(usize) + 'static,
) -> Retained<NSView> {
    let control = popup(mtm, options, selected, on_change);
    field(mtm, title, &control)
}

/// Popup that picks a value; option names are translated.
pub fn choice_field<T: Copy + PartialEq + 'static>(
    mtm: MainThreadMarker,
    title: Key,
    choices: &[(T, Key)],
    current: T,
    on_change: impl Fn(T) + 'static,
) -> Retained<NSView> {
    let named: Vec<(T, String)> = choices.iter().map(|&(v, k)| (v, t(k).to_string())).collect();
    choice_field_raw(mtm, t(title), &named, current, on_change)
}

/// Popup whose option names are not translated (e.g. "12 pt").
pub fn choice_field_raw<T: Copy + PartialEq + 'static>(
    mtm: MainThreadMarker,
    title: &str,
    choices: &[(T, String)],
    current: T,
    on_change: impl Fn(T) + 'static,
) -> Retained<NSView> {
    let names: Vec<String> = choices.iter().map(|(_, n)| n.clone()).collect();
    let values: Vec<T> = choices.iter().map(|(v, _)| *v).collect();
    let selected = values.iter().position(|v| *v == current).unwrap_or(0);
    popup_field(mtm, title, &names, selected, move |i| on_change(values[i]))
}
