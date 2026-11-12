# alt-tab-mac

**English** | [Türkçe](README_tr.md)

A fast, **window-based** switcher for macOS, written from scratch in Rust. Free and
open source, now and forever.

- Switches between **windows**, not apps, like Alt-Tab on Windows. Three windows of the
  same app are three items.
- **Zero latency:** the window list is kept up to date in the background. Pressing the
  shortcut never queries the system; the panel is drawn from the cached list, so even a
  50 ms tap works and slow apps (PhpStorm, Electron…) never delay it.
- **Light:** ~0% CPU when idle, ~15 MB memory.
- Intel and Apple Silicon in a single app, macOS 11 Big Sur and later.
- Turkish and English interface.

## Features

- Several shortcuts, each with its own scope and style. Defaults:

  | Shortcut | Windows shown |
  |----------|---------------|
  | `⌘ Tab` | All apps, all desktops, all screens |
  | `` ⌘ ` `` | Current app, all desktops, all screens |
  | `⌥ Tab` | All apps, current desktop, current screen |
  | `` ⌥ ` `` | Current app, current desktop, current screen |

  `` ` `` is the key above Tab (`"` on a Turkish Q keyboard). Keys are matched by
  physical position, so they work with any keyboard layout. `⌘ Tab` and `` ⌘ ` `` take
  over the system shortcuts while the app runs and give them back when it quits.
- **Styles:** icons, previews, previews with the app icon in the corner, list.
- **Scope per shortcut:** all apps or the current app, all or the current desktop,
  all or the current screen; minimized windows and hidden apps can be shown, hidden or
  placed at the end.
- **Panel:** horizontal or vertical layout, theme (system / light / dark), item and icon
  size, padding and spacing, font and font size, 1–3 title lines, sort order (recently
  used, app name, grouped by app), shown on the active window's screen, the mouse's
  screen, the main screen, a specific screen or all screens.
- **Window types:** dialogs, dialogs that block their main window and floating panels
  (Fonts, Colors…) can each be shown or hidden. Sheets (Save, Print) never appear as
  separate items.
- **Blacklist** for apps that should never appear.
- While the panel is open: `Tab` / `⇧Tab` / arrow keys to move, release the modifier or
  press `Enter` to switch, `Esc` to cancel, `W` close window, `M` minimize, `Q` quit app,
  `H` hide app (all keys configurable). Optional mouse selection.
- Launch at login, optional menu bar icon. When the icon is hidden, open the app again
  (Finder or Spotlight) to show the settings.
- If the shortcuts cannot work (Accessibility permission revoked, a password field is
  focused), the system `⌘ Tab` is given back temporarily.

## Installation

**Homebrew** (from your tap):
```sh
brew install --cask <user>/tap/alt-tab-mac
```

**Manually:** download the `.dmg` or `.zip` from [Releases](../../releases) and move the
app to the Applications folder.

The app is not notarized by Apple. If macOS blocks it on first launch, right-click it in
Finder and choose **Open**, or run:
```sh
xattr -dr com.apple.quarantine /Applications/alt-tab-mac.app
```

### Permissions

- **Accessibility** (required): System Settings → Privacy & Security → Accessibility.
  The app starts working as soon as the permission is granted.
- **Screen Recording** (only for preview styles): System Settings → Privacy & Security →
  Screen & System Audio Recording. Restart the app after granting it. Without it, the app
  icon is shown in place of the preview.

## Settings

Menu bar icon → **Settings…** (`⌘,`). Every change is applied immediately and saved to
`~/Library/Application Support/alt-tab-mac/config.toml`, which can also be edited by hand
and reloaded from the menu.

## Logs

The log is written to `~/Library/Logs/alt-tab-mac.log`, the folder macOS uses for app
logs; Console.app lists it as well.

- By default only **errors** are logged. The level can be changed at any time under
  **Settings → General → Logs** (errors only, warnings, info, detailed, very detailed);
  the change takes effect immediately, without a restart.
- Only the newest **2000 lines** are kept; older lines are deleted automatically.
- Lines are written by a background thread, so logging never slows down the switcher.
- **Reporting a problem:** set the level to *Detailed*, reproduce the problem, then click
  **Show log file in Finder** and attach the file to your report.

## Building

```sh
scripts/create-dev-cert.sh    # once: local signing certificate, so permissions survive rebuilds
scripts/bundle.sh             # universal .app → target/alt-tab-mac.app
scripts/bundle.sh --native    # this machine's architecture only (faster)
cargo test                    # unit tests
```

Debugging:
- Log: `~/Library/Logs/alt-tab-mac.log`; the `ALT_TAB_LOG=error|warn|info|debug|trace`
  environment variable overrides the level chosen in the settings
- `alt-tab-mac --diagnose`: permissions, desktops, window types, preview capture test
- `alt-tab-mac --render-settings <dir>` / `--render-panel <dir>`: render the settings tabs
  or the panel to PNG files without showing them (`ALT_TAB_LANG=tr|en` picks the language)

## Releasing

1. Bump `version` in `Cargo.toml` (e.g. `0.2.0`) and commit.
2. `git tag v0.2.0 && git push origin v0.2.0`
3. GitHub Actions (`.github/workflows/release.yml`) runs the tests and attaches the
   universal zip, the dmg, `SHA256SUMS` and the filled-in Homebrew cask (`alt-tab-mac.rb`)
   to a new GitHub Release.
4. Copy `alt-tab-mac.rb` to the `Casks/` folder of your tap repository.

Locally: `scripts/release.sh` and `scripts/update-cask.sh <user/repo>` → `dist/`.
On every push, `.github/workflows/ci.yml` checks formatting, clippy, tests and the
universal build.

## License

[GPL-3.0-or-later](LICENSE). Closed-source use requires a separate commercial license;
contact the author.

## Donations

Alt-Tab Mac is free and will always stay free. As a software developer who is deeply
bothered by projects that start as open source and later turn paid, and by charging money
for simple things, I built this project for all of you. That said, if you would like to
donate and support me, you can send any amount you like to my crypto addresses below. It
would make me very happy. In the future, I will list the people who donated here. Thank
you in advance!

| Network | Address |
|---------|---------|
| Bitcoin (BTC) | `bc1qnac4z5363m5ke937gqlkn69jfhcqqz9uv85drv` |
| Ethereum (ETH, USDT...) | `0xa3Ecccc8A958309519659f0d317A0B5eEEd43a26` |
| Solana (SOL) | `A6Q6aYPihLpvZhBvgXbFhxeXVTVb9T5t5jnciJEJ5RXB` |
| TRON (TRX, USDT...) | `TXt2esaLDc7rebiPCC2SUPaqrbao1NnGab` |
| Binance User ID (UID) | `22770019` |
| Binance username | `rustdev` |

The same addresses, with copy buttons, are in the app under **Settings → About**.

---

<p align="center">Made with ❤️ and 🦀 — <b>I love Rust!</b></p>
