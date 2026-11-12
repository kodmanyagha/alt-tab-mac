// SPDX-License-Identifier: GPL-3.0-or-later

use super::model::WindowId;

#[derive(Debug, Default)]
pub struct MruList {
    order: Vec<WindowId>,
}

impl MruList {
    /// Moves the window to the front (adds it if missing).
    pub fn touch(&mut self, id: WindowId) {
        self.remove(id);
        self.order.insert(0, id);
    }

    /// Appends the window if it has not been seen before.
    pub fn add_if_missing(&mut self, id: WindowId) {
        if !self.order.contains(&id) {
            self.order.push(id);
        }
    }

    pub fn remove(&mut self, id: WindowId) {
        self.order.retain(|&w| w != id);
    }

    /// Position of the window in the order; `usize::MAX` (last) if unknown.
    pub fn rank(&self, id: WindowId) -> usize {
        self.order.iter().position(|&w| w == id).unwrap_or(usize::MAX)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn touch_moves_to_front() {
        let mut mru = MruList::default();
        mru.add_if_missing(WindowId(1));
        mru.add_if_missing(WindowId(2));
        mru.touch(WindowId(2));
        assert_eq!(mru.rank(WindowId(2)), 0);
        assert_eq!(mru.rank(WindowId(1)), 1);
    }

    #[test]
    fn add_if_missing_keeps_existing_rank() {
        let mut mru = MruList::default();
        mru.touch(WindowId(1));
        mru.touch(WindowId(2));
        mru.add_if_missing(WindowId(1));
        assert_eq!(mru.rank(WindowId(1)), 1);
    }

    #[test]
    fn unknown_window_ranks_last() {
        let mru = MruList::default();
        assert_eq!(mru.rank(WindowId(9)), usize::MAX);
    }
}
