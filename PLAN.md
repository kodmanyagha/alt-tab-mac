# alt-tab-mac — Plan

A Windows-style **window** switcher for macOS, written from scratch in Rust.
Main goals: open instantly the moment the shortcut is pressed, use few resources, and
run on Intel and Apple Silicon, on **macOS 11 Big Sur and later**.

---

## 1. Goals

| # | Goal | Criterion |
|---|------|-----------|
| 1 | **Zero latency** | Shortcut → panel on screen: **< 16 ms** (1 frame). The OS is not queried at that moment; everything comes from the cache. |
| 2 | **Never miss a quick press** | Even if the shortcut is pressed and released within 50 ms, it switches to the previous window, even if the panel is never drawn. |
| 3 | **Low resource use** | Idle CPU ≈ 0% (no polling, event driven). RAM < 60 MB (previews included). |
| 4 | **Universal binary** | `x86_64` + `arm64` in a single `.app` (lipo). |
| 5 | **Wide compatibility** | macOS 11 Big Sur → current release. Newer APIs are picked at runtime. |
| 6 | **Free software** | GPL-3.0, every feature available, no paid restrictions. |

Out of scope (for now): window tiling, replacing Mission Control.

---

## 2. License

- **GPL-3.0-or-later.** Anyone who takes the project must also release the source code of
  the derivatives they distribute under the GPL. Nobody can ship a closed-source paid version.
- **Dual licensing:** Someone who wants to use the code in closed source can buy a separate
  commercial license from the project owner. This is only possible if the project owner holds
  the copyright to all of the code. External contributions therefore require a **CLA**
  (Contributor License Agreement): the contributor grants the project owner the right to also
  distribute the contribution under a commercial license. This is automated with a GitHub bot
  such as CLA Assistant (Phase 4).
- AltTab is GPL-3.0 too. Even so, **we do not copy its code.** Copying it would make dual
  licensing impossible, because we do not hold the copyright to that code. Everything is
  written from scratch, starting from the macOS API documentation.
- File headers get an SPDX line: `// SPDX-License-Identifier: GPL-3.0-or-later`

---

## 3. Features

### 3.1 Shortcuts (each configured separately)

Each shortcut consists of a **trigger** (modifier + key) and its own **scope settings**.
Shortcuts can be added and removed in the settings, and each one can be turned on or off
individually.

| Shortcut | Default scope (suggested) |
|----------|---------------------------|
| `⌘ Tab` | All apps · All desktops · All screens |
| `⌘ "` | Current app · All desktops · All screens |
| `⌥ Tab` | All apps · Current desktop · Current screen |
| `⌥ "` | Current app · Current desktop · Current screen |

- **Modifier options:**
  - Main combinations: `⌘`, `⌥`, `⌃`, `⌘⌥`, etc.
  - `⇧` always means "go back".
- **Key options:** `Tab`, the key above Tab (`"` / `` ` `` / `§`), or any key pressed in the recorder.
- **Physical key codes are used:** works regardless of the keyboard layout (Turkish Q/F, US, German…).
  - The `"` key on Turkish Q and the `` ` `` key on US are the same physical key.
  - This key has a different code on ISO and ANSI keyboards. The app detects the keyboard type and picks the right code.
- **`⌘ Tab` and `⌘ "` are system shortcuts.** When enabled they replace the system behavior
  (swallowed by the event tap). When disabled, the system behavior comes back.

### 3.2 Scope settings (separate for each shortcut)

| Setting | Options |
|---------|---------|
| **App** | All apps / Only the current (frontmost) app |
| **Desktop (Space)** | All desktops / Only the current desktop |
| **Screen** | All screens / Only the current screen (the one with the cursor or the active window, selectable) |
| **Minimized windows** | Show / Hide / Put at the end |
| **Hidden (⌘H) apps** | Show / Hide / Put at the end |
| **Full-screen windows** | Show / Hide |
| **Running apps without windows** | Show / Hide |
| **Sort order** | Recently used (MRU) / By app name / Grouped by app |
| **Style** | Preview / Icon only / List (icon + title) |

There is also a global **blacklist**: certain apps are never shown, or only their
windowless state is hidden.

### 3.3 Switching behavior
- Lists **windows**, not apps (3 windows of the same app = 3 items).
- Default order is **MRU**: the most recently used window comes first, the first Tab selects the previous one (as on Windows).
- Quick press: pressing the shortcut and releasing it right away switches to the previous window without drawing the panel.
- Optional show delay (default 0 ms).
- While the panel is open:
  - `Tab` / `⇧Tab` / arrow keys: navigate
  - Release the modifier or `Enter`: select
  - `Esc`: cancel
  - `W`: close window
  - `M`: minimize
  - `Q`: quit app
  - `H`: hide
  - Mouse click and hover

### 3.4 Appearance
- **Style:** Preview / Icon only / List
- **Theme:** System / Light / Dark, blurred background (NSVisualEffectView)
- Item size, items per row, which screen the panel opens on
- App icon badge, minimized/hidden window indicator

### 3.5 Other
- Menu bar icon: quick settings, Settings window, Quit
- Launch at login
- Settings are stored in `~/Library/Application Support/alt-tab-mac/config.toml` (can also be edited by hand)

---

## 4. macOS version compatibility (11 Big Sur+)

A single binary runs on every version. Newer frameworks are **weak linked** and picked at
runtime with `respondsToSelector` / version checks / `dlsym`:

| Feature | macOS 11–13 | macOS 14+ |
|---------|-------------|-----------|
| Window preview | `CGWindowListCreateImage` (via dlsym, since it was removed from the newer SDK) | `ScreenCaptureKit` · `SCScreenshotManager` |
| Launch at login | `~/Library/LaunchAgents` plist | `SMAppService` (13+) |
| Window list, AX, event tap | Same APIs | Same APIs |

- Rust targets: `aarch64-apple-darwin` and `x86_64-apple-darwin`, `MACOSX_DEPLOYMENT_TARGET=11.0`
- **Testing constraint:** The development machine runs macOS 27. Support for 11–13 is provided
  at the code level but needs testing on real hardware or a VM (Phase 4: macOS VM with UTM, or
  user testing).

---

## 5. Architecture

```
┌───────────────────────── Main thread (AppKit run loop) ──────────────────────────┐
│                                                                                  │
│  CGEventTap ──► HotkeyEngine ──► Switcher (state machine) ──► Panel (NSPanel)    │
│  (keyboard)      N shortcuts       Idle/Armed/Shown           view pool, layer   │
│                                          │  ▲                                    │
│                                   reads  ▼  │ snapshot (Arc, no copy)            │
│                                    ┌──────────────┐                              │
│                                    │ WindowCache  │◄── AXObserver events         │
│                                    │ (MRU, meta)  │◄── NSWorkspace notifications │
│                                    └──────────────┘                              │
└──────────────────────────────────────────▲───────────────────────────────────────┘
                                           │ updates (channel)
┌─────────────── Background ───────────────┴───────────────────────────────────────┐
│  Reconciler:  full scan on events + rarely on a timer (e.g. every 3 s)           │
│               (CGWindowListCopyWindowInfo + AX), timeouts for slow apps          │
│  Thumbnailer: previews (SCK or CGWindowListCreateImage), ≤ 400 px CGImage        │
│  IconCache:   app icons are rendered once and kept                               │
└──────────────────────────────────────────────────────────────────────────────────┘
```

### 5.1 Why will it be fast?
1. **No OS calls on the hot path.** When the shortcut arrives, the cache's current snapshot
   (`Arc<Vec<WindowInfo>>`) is taken, the shortcut's scope filter is applied (a few µs) and
   the panel is drawn. Slow apps such as PhpStorm only slow down the background thread.
2. **The panel is created in advance.** The NSPanel and item views are created once and kept
   hidden. On open, only the content is updated and `orderFront` is called.
3. **Previews are ready-made CGImages.** They are assigned to a layer's `contents`; drawing
   happens on the GPU.
4. **The CGEventTap callback only runs the state machine** (on the order of µs). If the tap
   times out (`kCGEventTapDisabledByTimeout`), it is re-enabled immediately.
5. **"Current screen / desktop / app" is cached too.** The active Space, the frontmost app
   and the screens are kept up to date through events. Nothing is queried when the shortcut
   is pressed.

### 5.2 How does the window cache stay up to date?
- **NSWorkspace:** app launched / terminated / activated / hidden, Space changed
- **AXObserver (per app):**
  - `kAXWindowCreated`, `kAXUIElementDestroyed`
  - `kAXFocusedWindowChanged`: these determine the MRU order
  - `kAXTitleChanged`, `kAXWindowMiniaturized` / `Deminiaturized`, `kAXWindowMoved` / `Resized`
- **Screen changes:** `NSApplicationDidChangeScreenParametersNotification`
- **Periodic reconciliation:** every ~3 s, missed events are corrected with
  `CGWindowListCopyWindowInfo` (<1 ms). AX calls have a per-app timeout
  (`AXUIElementSetMessagingTimeout`), so a frozen app does not hold up the others.
- **Window id:** the `CGWindowID` is read from the AX element with `_AXUIElementGetWindow`
  (private, but stable for years).

### 5.3 Preview strategy
- When focus leaves a window, a preview of that window is captured.
- Visible windows on the current Space are refreshed at a low rate (e.g. every 10 s), only
  while the panel is closed.
- When the panel opens, the cached image is shown first, then refreshed in the background and
  updated in place.
- Minimized windows and windows on other Spaces cannot be captured; the last known image is
  used. If there is none, the icon is shown.
- Images are scaled to ≤ 400 px wide. 50 windows ≈ 15 MB.
- If no shortcut uses a preview style, the preview system is turned off completely and the
  Screen Recording permission is not requested.

### 5.4 Desktop (Space) and screen support
- SkyLight private APIs are used for the active Space and each window's Space
  (`CGSGetActiveSpace`, `CGSCopySpacesForWindows`, `CGSCopyManagedDisplaySpaces`).
  They are loaded with `dlsym`. If they are missing, the app falls back to "all desktops"
  mode instead of crashing.
- With multiple screens, each screen has its own active Space. The "current desktop" filter
  works accordingly (both states of the "Displays have separate Spaces" setting are supported).
- A window's screen is the screen its frame overlaps the most.
- Switching to a window on another Space activates that window; macOS changes the Space itself.
- **Known difficulty:** some apps do not report their windows on other Spaces through AX.
  These are detected with `CGWindowList`. If needed, another method is investigated in Phase 4.

### 5.5 Switching to a window
1. If minimized, first `kAXMinimizedAttribute = false`
2. `AXUIElementPerformAction(window, kAXRaiseAction)`
3. `NSRunningApplication.activate`

This order makes sure the *right* window of the app comes to the front.

### 5.6 Permissions
- **Accessibility:** required (event tap + AX). A guide screen is shown on first launch.
- **Screen Recording:** only for the preview style. Without it, the icon style is used.

---

## 6. Technology

- **Rust (stable)** + the `objc2` ecosystem:
  - `objc2`, `objc2-foundation`, `objc2-app-kit`, `objc2-quartz-core`
  - `objc2-core-graphics`, `objc2-core-foundation`, `objc2-application-services` (AX)
  - `objc2-screen-capture-kit` (weak link), `block2`, `dispatch2`
- C APIs such as the event tap, AX and CGS through hand-written `extern "C"` FFI where needed
- `serde` + `toml` (settings), `log` + `oslog` (logs visible in Console.app)
- A single Rust binary; the `.app` bundle is created by a script. Not shown in the Dock (`LSUIElement`).

### Module layout
```
src/
  main.rs              app startup, NSApplication
  config.rs            settings (TOML), defaults, validation, shortcut definitions
  core/                ── platform-independent, unit-testable pure logic ──
    model.rs           WindowInfo, AppInfo, WindowId, SpaceId, ScreenId
    mru.rs             MRU ordering
    filter.rs          scope filters (app/desktop/screen, minimized…)
    switcher.rs        state machine (Idle → Armed → Shown → Commit/Cancel)
    hotkey.rs          shortcut matching (modifier + physical key code)
  macos/
    event_tap.rs       CGEventTap, swallowing keys, taking over ⌘Tab
    keyboard.rs        keyboard type (ANSI/ISO/JIS), key names
    ax.rs              AXUIElement wrappers, AXObserver, timeouts
    workspace.rs       NSWorkspace notifications
    window_list.rs     CGWindowListCopyWindowInfo
    spaces.rs          SkyLight private API (dlsym, optional)
    screens.rs         screens, screen-to-window matching
    capture/           previews: sck.rs (14+), legacy.rs (11–13)
    activate.rs        switching to a window
    login_item.rs      launch at login (SMAppService / LaunchAgent)
    permissions.rs     permission checks and guidance
    os_version.rs      runtime version checks
  cache.rs             WindowCache + background reconciler
  ui/
    panel.rs           NSPanel, layout (grid), view pool
    item_view.rs       item (preview/icon/title, selection highlight)
    theme.rs           colors, materials
    status_item.rs     menu bar
    settings/          settings window, shortcut recorder
scripts/
  bundle.sh            universal build + .app + signing
```

---

## 7. Phases

### Phase 1: Core and the fastest path (MVP)
- [x] Project skeleton, GPL license file, `.app` bundling, universal build (deployment target 11.0)
- [x] Permission check (Accessibility)
- [x] CGEventTap: 4 default shortcuts (`⌘Tab`, `⌘"`, `⌥Tab`, `⌥"`), physical key codes, back with `⇧`, `Esc`, swallowing keys while the panel is open
- [x] WindowCache: initial scan with CGWindowList + AX, NSWorkspace and AXObserver events, MRU
- [x] Scope filters: app (all/current) and screen (all/current)
- [x] Panel: icon + title style, selection, switch on modifier release
- [x] Switch without drawing the panel on a quick press
- [x] Switching to a window (raise + activate, minimized included)
- [x] Shortcut and scope settings via `config.toml` (no UI, from the file)
- [x] Latency measurement: shortcut → panel visible time is logged

> Note: the "current desktop" filter works with a temporary method for now (is the window
> on screen right now?). Accurate desktop information through SkyLight comes in Phase 2.

**Done when:** it can replace `⌘Tab` in daily use.

### Phase 2: Previews and desktops
- [x] Thumbnailer: SCK (14+) and CGWindowListCreateImage (11–13), cache, permission flow
- [x] Preview style, live refresh while the panel is open
- [x] Desktop (Space) support, "current desktop" scope, a separate Space per screen
- [x] In-panel actions (W/M/Q/H), mouse support

### Phase 3: Settings and appearance
- [x] Settings window (AppKit, 5 tabs: General, Shortcuts, Appearance, Windows, About)
  - shortcut list (add/remove/enable/disable), shortcut recorder
  - scope and style per shortcut
  - panel action keys (W/M/Q/H) can be changed
  - changes are applied immediately and written to `config.toml`
- [x] Theme (system/light/dark), item size (small/medium/large), list style, sort order
      (recently used / app name / grouped by app)
- [x] **Screen the panel appears on:** the active window's screen, the mouse's screen, the
      main screen, all screens (synchronized panels), a screen chosen by name
- [x] **Window kinds** (3.A): dialogs / blocking dialogs / floating panels are configurable
- [x] Blacklist (picked from running apps, stored by bundle id)
- [x] Show/hide the menu bar icon; when hidden, opening the app again opens the settings
- [x] Launch at login (13+ `SMAppService`, 11–12 LaunchAgent; the state is read from macOS)
- [x] `--render-settings <dir>`: developer tool that saves the settings tabs as PNG
- [x] `--render-panel <dir>`: developer tool that saves the panel in every style × orientation as PNG
- [x] Styles: Icon (default) / Preview / Preview + icon in the corner / List
- [x] Turn mouse selection in the panel on/off
- [x] Localization: Turkish (listed first) and English; defaults to the system language (`src/i18n/`, a missing translation is a compile error)
- [x] Settings window fieldset layout: labels on top, full-width controls, two columns
- [x] Title lines (1–3, the last line is truncated with "…")
- [x] Vertical orientation: items stacked, image at half size, title on the right
- [x] Orientation (horizontal / vertical), icon size, item padding, item spacing, font family and size
- [ ] Badge on the main window while its blocking dialog is hidden (3.A Step 4): postponed
- [ ] Quick settings in the menu bar menu (only "Settings…" for now)
- [ ] Panel not fitting on screen with many windows (scrolling / shrinking): Phase 4

#### 3.A Window kinds

**Step 1: Measure.** `--diagnose` also prints the role / subrole / modal / parent of every AX
window. A table is built from real apps (Finder "Get Info", TextEdit Fonts, PhpStorm Settings,
Safari Web Inspector, Xcode, Electron apps). The classification is based on this data, not on
guesses.

**Step 2: Classify (`core/window_kind.rs`, pure, testable).**

| Kind | Detection (AX) | Example | Default |
|------|----------------|---------|---------|
| Standard window | `AXStandardWindow` | documents, Finder "Get Info" | show |
| Non-blocking dialog | `AXDialog`, `AXModal = false` | "About", some alerts | show |
| Blocking dialog | `AXDialog` / standard, `AXModal = true` | PhpStorm Settings | hide (badge on the main window) |
| Floating panel | `AXFloatingWindow`, `AXSystemFloatingWindow` | Fonts, Colors, palettes | hide |
| Sheet | `AXSheet` (child of the window) | Save / Print | never a separate item |
| Other / unknown | other subroles | tooltips, pop-up menus | hide |

**Step 3: Settings.** A global `[window_kinds]` section (not per shortcut; maybe later if needed):
```toml
[window_kinds]
dialogs = true            # non-blocking dialogs
modal_dialogs = false     # dialogs that block their main window
floating_panels = false   # Fonts, Colors, etc.
```

**Step 4: Badge.** A hidden blocking dialog is indicated by a small mark on its main window's
item; selecting that item brings the dialog to the front.

#### 3.B Settings window technology

**AppKit (objc2)**, not egui:
- winit/egui want their own event loop, which conflicts with our NSApplication loop.
- No GPU drawing or extra dependencies needed; the binary and memory stay small.
- The real macOS look, dark mode and accessibility come for free.
- The cost: more code. This is offset by small, reusable form pieces
  (`ui/settings/widgets/`: checkbox row, pop-up row, shortcut recorder, section header).
- Changes are applied immediately and written to `config.toml` (no "reload" needed).

### Phase 4: Hardening and distribution
- [x] Edge cases:
  - [x] sleep/wake: full reconciliation on wake; the health check re-enables the event tap if it is disabled
  - [x] revoked permission and Secure Input (password field): the system ⌘Tab is given back
        temporarily and taken over again once things recover (once-a-second health check,
        `controller/health.rs`)
  - [x] many windows: if the panel does not fit on screen, the items shrink step by step (`ui/fit.rs`)
  - [x] arrow keys move by row/column in the grid (`core/grid_nav.rs`)
  - [ ] full-screen apps, Electron/Java (PhpStorm): to be observed in real use
- [ ] Another method for missing windows on other Spaces
- [ ] macOS 11–13 tests (VM)
- [x] Resource use: AppKit/CG/QuartzCore bindings with only the features in use;
      CloudKit, CoreData, CoreText, CoreImage, CoreVideo and Metal are no longer linked.
      Measured: idle CPU 0.0%, memory ~15 MB.
- [x] Bundle id: `emirbugra.alttabmac.app` (single source: `packaging/Info.plist`)
- [x] Distribution (without a Developer ID, ad-hoc signed):
  - `scripts/release.sh`: universal zip + dmg + SHA256SUMS
  - Homebrew cask template + `scripts/update-cask.sh`
  - GitHub Actions: `ci.yml` (fmt, clippy -D warnings, test, universal .app), `release.yml` (v* tag → GitHub Release)
- [ ] Later: Developer ID signing + notarization, automatic updates
- [ ] Project files:
  - README, screenshots
  - contribution guide, CLA bot

---

## 8. Testing strategy
- The `core/` module is pure Rust: unit tests for MRU, filters, shortcut matching and the
  state machine (quick press, back with Shift, cancel, switching between shortcuts…).
- macOS side: a manual test list (multiple screens, multiple desktops, minimized, hidden,
  full screen, simulated frozen app, different keyboard layouts).
- Performance: the `shortcut → first frame` time is logged on every open, target < 16 ms.
  A stress test with 100+ windows is also done.

---

## 9. Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Private APIs (`_AXUIElementGetWindow`, SkyLight) may change in a new macOS | The desktop filter breaks | Optional loading with `dlsym`, graceful fallback if missing |
| macOS 11–13 cannot be tested on this machine | Bugs may appear on older versions | Weak linking + runtime checks, VM testing in Phase 4 |
| An ad-hoc signed build resets permissions on every build (during development) | Permission must be granted after each build | Sign with a fixed self-signed certificate |
| The Screen Recording permission may show periodic reminders (macOS 15+) | Annoys the user | The icon style works without the permission; documented |
| The event tap cannot receive keys while Secure Input (password fields) is on | The shortcut does not work at that moment | macOS limitation; documented |
| Some apps answer AX slowly or incorrectly | The list may be incomplete or stale | Timeouts, cross-checking with CGWindowList, periodic reconcile |
| `⌘Tab` takeover: if the event tap is disabled, the system switcher comes back | Temporary inconsistency | Tap health is monitored, automatic re-enabling |

---

## 10. Open questions
1. **App name:** `alt-tab-mac` is a working name. Before publishing, a name that cannot be confused with AltTab would be better.
2. **Default scopes:** is the mapping in the table in 3.1 right?
