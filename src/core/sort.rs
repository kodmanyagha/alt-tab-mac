// SPDX-License-Identifier: GPL-3.0-or-later

use super::appearance::SortOrder;
use super::model::{Pid, WindowInfo};

pub fn sort_windows(windows: &mut [&WindowInfo], order: SortOrder) {
    match order {
        SortOrder::RecentlyUsed => {}
        SortOrder::AppName => windows.sort_by_cached_key(|w| w.app_name.to_lowercase()),
        SortOrder::GroupedByApp => group_by_app(windows),
    }
}

/// The position of an app's first (most recently used) window decides the group order. The stable sort keeps the MRU
/// order inside each group.
fn group_by_app(windows: &mut [&WindowInfo]) {
    let mut first_seen: Vec<Pid> = Vec::new();
    for w in windows.iter() {
        if !first_seen.contains(&w.pid) {
            first_seen.push(w.pid);
        }
    }
    windows.sort_by_key(|w| first_seen.iter().position(|&p| p == w.pid));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::geometry::Rect;
    use crate::core::model::WindowId;
    use crate::core::window_kind::WindowKind;

    fn window(id: u32, pid: i32, app: &str) -> WindowInfo {
        WindowInfo {
            id: WindowId(id),
            pid: Pid(pid),
            app_name: app.into(),
            bundle_id: String::new(),
            title: String::new(),
            frame: Rect::default(),
            is_minimized: false,
            is_app_hidden: false,
            is_on_active_space: true,
            kind: WindowKind::Standard,
        }
    }

    fn ids(list: &[&WindowInfo]) -> Vec<u32> {
        list.iter().map(|w| w.id.0).collect()
    }

    #[test]
    fn grouped_keeps_app_order_by_recency() {
        let all = [window(1, 1, "B"), window(2, 2, "A"), window(3, 1, "B")];
        let mut list: Vec<&WindowInfo> = all.iter().collect();
        sort_windows(&mut list, SortOrder::GroupedByApp);
        assert_eq!(ids(&list), vec![1, 3, 2]);
    }

    #[test]
    fn app_name_is_alphabetical_and_stable() {
        let all = [window(1, 1, "b"), window(2, 2, "A"), window(3, 1, "b")];
        let mut list: Vec<&WindowInfo> = all.iter().collect();
        sort_windows(&mut list, SortOrder::AppName);
        assert_eq!(ids(&list), vec![2, 1, 3]);
    }
}
