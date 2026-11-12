// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "on", content = "name")]
pub enum PanelScreen {
    #[default]
    ActiveWindow,
    Cursor,
    Primary,
    All,
    Named(String),
}

pub struct ScreenRefs<'a> {
    pub names: &'a [String],
    pub active_window: Option<usize>,
    pub cursor: Option<usize>,
}

/// Indices of the screens the panel is drawn on (non-empty if there is any screen).
pub fn resolve(choice: &PanelScreen, refs: &ScreenRefs) -> Vec<usize> {
    let count = refs.names.len();
    if count == 0 {
        return Vec::new();
    }
    let single = match choice {
        PanelScreen::All => return (0..count).collect(),
        PanelScreen::ActiveWindow => refs.active_window.or(refs.cursor),
        PanelScreen::Cursor => refs.cursor,
        PanelScreen::Primary => Some(0),
        PanelScreen::Named(name) => refs.names.iter().position(|n| n == name),
    };
    vec![single.filter(|&i| i < count).unwrap_or(0)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn refs(names: &[String]) -> ScreenRefs<'_> {
        ScreenRefs { names, active_window: Some(1), cursor: Some(2) }
    }

    fn names() -> Vec<String> {
        vec!["Built-in".into(), "DELL".into(), "LG".into()]
    }

    #[test]
    fn resolves_each_choice() {
        let n = names();
        assert_eq!(resolve(&PanelScreen::ActiveWindow, &refs(&n)), vec![1]);
        assert_eq!(resolve(&PanelScreen::Cursor, &refs(&n)), vec![2]);
        assert_eq!(resolve(&PanelScreen::Primary, &refs(&n)), vec![0]);
        assert_eq!(resolve(&PanelScreen::All, &refs(&n)), vec![0, 1, 2]);
        assert_eq!(resolve(&PanelScreen::Named("LG".into()), &refs(&n)), vec![2]);
    }

    #[test]
    fn missing_named_screen_falls_back_to_primary() {
        let n = names();
        assert_eq!(resolve(&PanelScreen::Named("Yok".into()), &refs(&n)), vec![0]);
    }

    #[test]
    fn active_window_falls_back_to_cursor() {
        let n = names();
        let r = ScreenRefs { names: &n, active_window: None, cursor: Some(2) };
        assert_eq!(resolve(&PanelScreen::ActiveWindow, &r), vec![2]);
    }
}
