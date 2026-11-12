// SPDX-License-Identifier: GPL-3.0-or-later

use super::model::WindowId;

#[derive(Debug)]
struct Session {
    hotkey: usize,
    items: Vec<WindowId>,
    selected: usize,
}

#[derive(Debug, Default)]
pub struct Switcher {
    session: Option<Session>,
}

impl Switcher {
    /// Starts a new session and returns the index of the initially selected item: the one after the focused window
    /// (Windows behavior).
    pub fn start(&mut self, hotkey: usize, items: Vec<WindowId>, focused: Option<WindowId>, reverse: bool) -> usize {
        let selected = initial_selection(&items, focused, reverse);
        self.session = Some(Session { hotkey, items, selected });
        selected
    }

    pub fn active_hotkey(&self) -> Option<usize> {
        self.session.as_ref().map(|s| s.hotkey)
    }

    pub fn selected(&self) -> Option<usize> {
        self.session.as_ref().map(|s| s.selected)
    }

    /// Moves the selection by `delta` (wrapping around) and returns the new index.
    pub fn step(&mut self, delta: isize) -> Option<usize> {
        let session = self.session.as_mut()?;
        let len = session.items.len();
        if len == 0 {
            return None;
        }
        session.selected = wrap(session.selected as isize + delta, len);
        Some(session.selected)
    }

    pub fn selected_id(&self) -> Option<WindowId> {
        let session = self.session.as_ref()?;
        session.items.get(session.selected).copied()
    }

    /// Sets the selection directly (mouse).
    pub fn select(&mut self, index: usize) -> Option<usize> {
        let session = self.session.as_mut()?;
        (index < session.items.len()).then(|| {
            session.selected = index;
            index
        })
    }

    /// Removes windows from the list (closed / app quit). The selection keeps its position, moving back if the list got
    /// shorter.
    pub fn remove(&mut self, ids: &[WindowId]) -> Option<usize> {
        let session = self.session.as_mut()?;
        session.items.retain(|id| !ids.contains(id));
        session.selected = session.selected.min(session.items.len().saturating_sub(1));
        Some(session.selected)
    }

    /// Ends the session and returns the selected window.
    pub fn commit(&mut self) -> Option<WindowId> {
        let session = self.session.take()?;
        session.items.get(session.selected).copied()
    }

    pub fn cancel(&mut self) {
        self.session = None;
    }
}

fn initial_selection(items: &[WindowId], focused: Option<WindowId>, reverse: bool) -> usize {
    if items.is_empty() {
        return 0;
    }
    match (items.iter().position(|&id| Some(id) == focused), reverse) {
        (Some(i), false) => wrap(i as isize + 1, items.len()),
        (Some(i), true) => wrap(i as isize - 1, items.len()),
        (None, false) => 0,
        (None, true) => items.len() - 1,
    }
}

fn wrap(index: isize, len: usize) -> usize {
    index.rem_euclid(len as isize) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(n: u32) -> Vec<WindowId> {
        (1..=n).map(WindowId).collect()
    }

    #[test]
    fn starts_on_second_item_when_first_is_focused() {
        let mut s = Switcher::default();
        assert_eq!(s.start(0, ids(3), Some(WindowId(1)), false), 1);
    }

    #[test]
    fn starts_on_first_item_when_focused_window_is_filtered_out() {
        let mut s = Switcher::default();
        assert_eq!(s.start(0, ids(3), Some(WindowId(9)), false), 0);
    }

    #[test]
    fn reverse_starts_on_last_item() {
        let mut s = Switcher::default();
        assert_eq!(s.start(0, ids(3), Some(WindowId(1)), true), 2);
    }

    #[test]
    fn single_window_selects_itself() {
        let mut s = Switcher::default();
        assert_eq!(s.start(0, ids(1), Some(WindowId(1)), false), 0);
    }

    #[test]
    fn starts_after_focused_window_in_the_middle() {
        let mut s = Switcher::default();
        assert_eq!(s.start(0, ids(3), Some(WindowId(2)), false), 2);
    }

    #[test]
    fn step_wraps_around() {
        let mut s = Switcher::default();
        s.start(0, ids(3), Some(WindowId(1)), false);
        assert_eq!(s.step(1), Some(2));
        assert_eq!(s.step(1), Some(0));
        assert_eq!(s.step(-1), Some(2));
    }

    #[test]
    fn quick_tap_commits_previous_window() {
        let mut s = Switcher::default();
        s.start(0, ids(3), Some(WindowId(1)), false);
        assert_eq!(s.commit(), Some(WindowId(2)));
        assert_eq!(s.active_hotkey(), None);
    }

    #[test]
    fn commit_on_empty_list_returns_none() {
        let mut s = Switcher::default();
        s.start(0, vec![], None, false);
        assert_eq!(s.step(1), None);
        assert_eq!(s.commit(), None);
    }

    #[test]
    fn removing_selected_last_item_moves_selection_back() {
        let mut s = Switcher::default();
        s.start(0, ids(3), None, true);
        assert_eq!(s.remove(&[WindowId(3)]), Some(1));
        assert_eq!(s.selected_id(), Some(WindowId(2)));
    }

    #[test]
    fn select_rejects_out_of_range() {
        let mut s = Switcher::default();
        s.start(0, ids(2), None, false);
        assert_eq!(s.select(5), None);
        assert_eq!(s.select(1), Some(1));
    }

    #[test]
    fn cancel_ends_session() {
        let mut s = Switcher::default();
        s.start(0, ids(2), None, false);
        s.cancel();
        assert_eq!(s.commit(), None);
    }
}
