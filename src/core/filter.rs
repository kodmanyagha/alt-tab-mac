// SPDX-License-Identifier: GPL-3.0-or-later

use super::model::{Pid, Snapshot, WindowInfo};
use super::scope::{AppScope, Scope, ScreenScope, SpaceScope, Visibility};
use super::screens::screen_of_rect;
use super::window_kind::WindowKinds;

#[derive(Clone, Copy, Debug, Default)]
pub struct FilterContext<'a> {
    pub frontmost_pid: Option<Pid>,
    pub current_screen: Option<usize>,
    pub kinds: WindowKinds,
    pub blacklist: &'a [String],
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Placement {
    Normal,
    AtEnd,
}

/// Returns the windows matching the scope: normal windows first, then the "show at end" ones. Each group keeps its MRU
/// order.
pub fn filter_windows<'a>(snapshot: &'a Snapshot, scope: &Scope, ctx: &FilterContext<'_>) -> Vec<&'a WindowInfo> {
    let mut placed: Vec<(Placement, &WindowInfo)> = snapshot
        .windows
        .iter()
        .filter(|w| passes_location(w, scope, ctx, snapshot))
        .filter_map(|w| placement(w, scope).map(|p| (p, w)))
        .collect();
    placed.sort_by_key(|(p, _)| *p);
    placed.into_iter().map(|(_, w)| w).collect()
}

fn passes_location(w: &WindowInfo, scope: &Scope, ctx: &FilterContext<'_>, snap: &Snapshot) -> bool {
    ctx.kinds.allows(w.kind)
        && !ctx.blacklist.contains(&w.bundle_id)
        && passes_app(w, scope.apps, ctx)
        && passes_space(w, scope.spaces)
        && passes_screen(w, scope.screens, ctx, snap)
}

fn passes_app(w: &WindowInfo, apps: AppScope, ctx: &FilterContext<'_>) -> bool {
    match apps {
        AppScope::All => true,
        AppScope::Current => ctx.frontmost_pid == Some(w.pid),
    }
}

fn passes_space(w: &WindowInfo, spaces: SpaceScope) -> bool {
    match spaces {
        SpaceScope::All => true,
        SpaceScope::Current => w.is_on_active_space || w.is_minimized || w.is_app_hidden,
    }
}

fn passes_screen(w: &WindowInfo, screens: ScreenScope, ctx: &FilterContext<'_>, snap: &Snapshot) -> bool {
    match (screens, ctx.current_screen) {
        (ScreenScope::All, _) | (ScreenScope::Current, None) => true,
        (ScreenScope::Current, Some(current)) => screen_of_rect(&w.frame, &snap.screens) == Some(current),
    }
}

/// `None` means the window is not shown.
fn placement(w: &WindowInfo, scope: &Scope) -> Option<Placement> {
    let rules = [(w.is_app_hidden, scope.hidden_apps), (w.is_minimized, scope.minimized)];
    let mut result = Placement::Normal;
    for (applies, visibility) in rules {
        if !applies {
            continue;
        }
        match visibility {
            Visibility::Hide => return None,
            Visibility::ShowAtEnd => result = Placement::AtEnd,
            Visibility::Show => {}
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::geometry::Rect;
    use crate::core::model::WindowId;
    use crate::core::window_kind::{WindowKind, WindowKinds};

    fn window(id: u32, pid: i32, x: f64) -> WindowInfo {
        WindowInfo {
            id: WindowId(id),
            pid: Pid(pid),
            app_name: format!("app{pid}"),
            bundle_id: format!("bundle{pid}"),
            title: format!("win{id}"),
            frame: Rect::new(x, 0.0, 100.0, 100.0),
            is_minimized: false,
            is_app_hidden: false,
            is_on_active_space: true,
            kind: WindowKind::Standard,
        }
    }

    fn snapshot(windows: Vec<WindowInfo>) -> Snapshot {
        Snapshot {
            windows,
            frontmost_pid: Some(Pid(1)),
            screens: vec![Rect::new(0.0, 0.0, 1000.0, 800.0), Rect::new(1000.0, 0.0, 1000.0, 800.0)],
            ..Snapshot::default()
        }
    }

    fn ids(list: &[&WindowInfo]) -> Vec<u32> {
        list.iter().map(|w| w.id.0).collect()
    }

    fn ctx(screen: Option<usize>) -> FilterContext<'static> {
        FilterContext {
            frontmost_pid: Some(Pid(1)),
            current_screen: screen,
            kinds: WindowKinds::default(),
            blacklist: &[],
        }
    }

    #[test]
    fn all_scope_keeps_mru_order() {
        let snap = snapshot(vec![window(1, 1, 0.0), window(2, 2, 0.0), window(3, 1, 0.0)]);
        let result = filter_windows(&snap, &Scope::default(), &ctx(None));
        assert_eq!(ids(&result), vec![1, 2, 3]);
    }

    #[test]
    fn current_app_scope() {
        let snap = snapshot(vec![window(1, 1, 0.0), window(2, 2, 0.0), window(3, 1, 0.0)]);
        let scope = Scope { apps: AppScope::Current, ..Scope::default() };
        assert_eq!(ids(&filter_windows(&snap, &scope, &ctx(None))), vec![1, 3]);
    }

    #[test]
    fn current_screen_scope() {
        let snap = snapshot(vec![window(1, 1, 0.0), window(2, 2, 1200.0)]);
        let scope = Scope { screens: ScreenScope::Current, ..Scope::default() };
        assert_eq!(ids(&filter_windows(&snap, &scope, &ctx(Some(1)))), vec![2]);
    }

    #[test]
    fn current_space_scope_keeps_minimized() {
        let mut other_space = window(2, 1, 0.0);
        other_space.is_on_active_space = false;
        let mut minimized = window(3, 1, 0.0);
        minimized.is_on_active_space = false;
        minimized.is_minimized = true;
        let snap = snapshot(vec![window(1, 1, 0.0), other_space, minimized]);
        let scope = Scope { spaces: SpaceScope::Current, ..Scope::default() };
        assert_eq!(ids(&filter_windows(&snap, &scope, &ctx(None))), vec![1, 3]);
    }

    #[test]
    fn minimized_at_end() {
        let mut minimized = window(1, 1, 0.0);
        minimized.is_minimized = true;
        let snap = snapshot(vec![minimized, window(2, 1, 0.0)]);
        assert_eq!(ids(&filter_windows(&snap, &Scope::default(), &ctx(None))), vec![2, 1]);
    }

    #[test]
    fn floating_panels_hidden_by_default() {
        let mut panel = window(2, 1, 0.0);
        panel.kind = WindowKind::FloatingPanel;
        let snap = snapshot(vec![window(1, 1, 0.0), panel]);
        assert_eq!(ids(&filter_windows(&snap, &Scope::default(), &ctx(None))), vec![1]);
    }

    #[test]
    fn blacklisted_apps_are_excluded() {
        let snap = snapshot(vec![window(1, 1, 0.0), window(2, 2, 0.0)]);
        let blacklist = vec!["bundle2".to_string()];
        let ctx = FilterContext { blacklist: &blacklist, ..ctx(None) };
        assert_eq!(ids(&filter_windows(&snap, &Scope::default(), &ctx)), vec![1]);
    }

    #[test]
    fn hidden_apps_can_be_hidden() {
        let mut hidden = window(1, 1, 0.0);
        hidden.is_app_hidden = true;
        let snap = snapshot(vec![hidden, window(2, 1, 0.0)]);
        let scope = Scope { hidden_apps: Visibility::Hide, ..Scope::default() };
        assert_eq!(ids(&filter_windows(&snap, &scope, &ctx(None))), vec![2]);
    }
}
