// SPDX-License-Identifier: GPL-3.0-or-later

use objc2::MainThreadMarker;
use objc2::rc::Retained;
use objc2_app_kit::{NSScreen, NSView};

use crate::config::Config;
use crate::controller::update_config;
use crate::core::panel_screen::PanelScreen;
use crate::core::scope::ScreenReference;
use crate::i18n::{Key, LanguageSetting, Template, t, tf};
use crate::logging::{self, LogLevel};
use crate::macos::{finder, login_item};
use crate::ui::settings::widgets::{
    button, checkbox, choice_field, choice_field_raw, fieldset, note, popup, popup_field, two_columns,
};

const LANGUAGES: [(LanguageSetting, &str); 2] =
    [(LanguageSetting::Turkish, "Türkçe"), (LanguageSetting::English, "English")];

const REFERENCES: [(ScreenReference, Key); 2] =
    [(ScreenReference::ActiveWindow, Key::ActiveWindowScreen), (ScreenReference::Cursor, Key::CursorScreen)];

const LOG_LEVELS: [(LogLevel, Key); 5] = [
    (LogLevel::Error, Key::LogLevelError),
    (LogLevel::Warn, Key::LogLevelWarn),
    (LogLevel::Info, Key::LogLevelInfo),
    (LogLevel::Debug, Key::LogLevelDebug),
    (LogLevel::Trace, Key::LogLevelTrace),
];

const FIXED_PANEL_SCREENS: [(PanelScreen, Key); 4] = [
    (PanelScreen::ActiveWindow, Key::ActiveWindowScreen),
    (PanelScreen::Cursor, Key::CursorScreen),
    (PanelScreen::Primary, Key::PrimaryScreen),
    (PanelScreen::All, Key::AllScreens),
];

pub fn build(mtm: MainThreadMarker, config: &Config) -> Retained<NSView> {
    let language = language_fieldset(mtm, config);
    let startup = startup_fieldset(mtm, config);
    let mouse = mouse_fieldset(mtm, config);
    let panel = panel_fieldset(mtm, config);
    let logs = logs_fieldset(mtm, config);
    Retained::into_super(two_columns(mtm, &[&language, &startup, &mouse], &[&panel, &logs]))
}

/// The box title already says "Language", so the popup has no label.
fn language_fieldset(mtm: MainThreadMarker, config: &Config) -> Retained<NSView> {
    let mut choices: Vec<LanguageSetting> = LANGUAGES.iter().map(|&(v, _)| v).collect();
    choices.push(LanguageSetting::System);
    let mut names: Vec<String> = LANGUAGES.iter().map(|&(_, n)| n.to_string()).collect();
    names.push(t(Key::LanguageSystem).to_string());
    let selected = choices.iter().position(|&c| c == config.general.language).unwrap_or(0);
    let control = popup(mtm, &names, selected, move |i| {
        let choice = choices[i];
        update_config(|c| c.general.language = choice);
        crate::ui::settings::rebuild();
    });
    fieldset(mtm, t(Key::Language), &[&control])
}

fn startup_fieldset(mtm: MainThreadMarker, config: &Config) -> Retained<NSView> {
    let login = checkbox(mtm, t(Key::LaunchAtLogin), login_item::is_enabled(), set_login_item);
    let status = checkbox(mtm, t(Key::ShowStatusItem), config.general.show_status_item, |v| {
        update_config(|c| c.general.show_status_item = v)
    });
    let status_note = note(mtm, t(Key::StatusItemNote));
    fieldset(mtm, t(Key::Startup), &[&login, &status, &status_note])
}

fn mouse_fieldset(mtm: MainThreadMarker, config: &Config) -> Retained<NSView> {
    let mouse = checkbox(mtm, t(Key::MouseSelection), config.general.mouse_selection, |v| {
        update_config(|c| c.general.mouse_selection = v)
    });
    let mouse_note = note(mtm, t(Key::MouseSelectionNote));
    fieldset(mtm, t(Key::Mouse), &[&mouse, &mouse_note])
}

fn panel_fieldset(mtm: MainThreadMarker, config: &Config) -> Retained<NSView> {
    let delay = delay_field(mtm, config.general.show_delay_ms);
    let delay_note = note(mtm, t(Key::ShowDelayNote));
    let screen = panel_screen_field(mtm, &config.general.panel_screen);
    let reference = choice_field(mtm, Key::ScreenFilter, &REFERENCES, config.general.screen_reference, |v| {
        update_config(|c| c.general.screen_reference = v)
    });
    fieldset(mtm, t(Key::Panel), &[&delay, &delay_note, &screen, &reference])
}

fn logs_fieldset(mtm: MainThreadMarker, config: &Config) -> Retained<NSView> {
    let level = choice_field(mtm, Key::LogLevel, &LOG_LEVELS, config.general.log_level, |v| {
        update_config(|c| c.general.log_level = v)
    });
    let logs_note = note(mtm, t(Key::LogsNote));
    let show = button(mtm, t(Key::ShowLogFile), || finder::reveal(&logging::log_path()));
    fieldset(mtm, t(Key::Logs), &[&level, &logs_note, &show])
}

fn delay_field(mtm: MainThreadMarker, current: u64) -> Retained<NSView> {
    let mut choices = vec![(0u64, t(Key::Immediately).to_string())];
    choices.extend([50u64, 100, 150, 200, 300].map(|ms| (ms, format!("{ms} ms"))));
    choice_field_raw(mtm, t(Key::ShowDelay), &choices, current, |v| update_config(|c| c.general.show_delay_ms = v))
}

fn set_login_item(enabled: bool) {
    if let Err(err) = login_item::set_enabled(enabled) {
        tracing::error!(error = %err, "could not set launch at login");
    }
}

/// Fixed options plus the names of the connected screens.
fn panel_screen_field(mtm: MainThreadMarker, current: &PanelScreen) -> Retained<NSView> {
    let names: Vec<String> = NSScreen::screens(mtm).iter().map(|s| s.localizedName().to_string()).collect();
    let mut options: Vec<String> = FIXED_PANEL_SCREENS.iter().map(|(_, k)| t(*k).to_string()).collect();
    options.extend(names.iter().map(|n| tf(Template::ScreenName, n)));
    let selected = selected_panel_screen(current, &names);
    popup_field(mtm, t(Key::PanelScreen), &options, selected, move |i| {
        let choice = panel_screen_at(i, &names);
        update_config(|c| c.general.panel_screen = choice);
    })
}

fn selected_panel_screen(current: &PanelScreen, names: &[String]) -> usize {
    if let PanelScreen::Named(name) = current {
        return names.iter().position(|n| n == name).map_or(0, |i| FIXED_PANEL_SCREENS.len() + i);
    }
    FIXED_PANEL_SCREENS.iter().position(|(v, _)| v == current).unwrap_or(0)
}

fn panel_screen_at(index: usize, names: &[String]) -> PanelScreen {
    match FIXED_PANEL_SCREENS.get(index) {
        Some((choice, _)) => choice.clone(),
        None => PanelScreen::Named(names[index - FIXED_PANEL_SCREENS.len()].clone()),
    }
}
