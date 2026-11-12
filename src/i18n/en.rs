// SPDX-License-Identifier: GPL-3.0-or-later

use super::key::Key::{self, *};
use super::template::Template;

pub fn text(key: Key) -> &'static str {
    match key {
        MenuSettings => "Settings…",
        MenuOpenConfig => "Open Config File",
        MenuReloadConfig => "Reload Config File",
        MenuQuit => "Quit",

        SettingsTitle => "alt-tab-mac Settings",
        TabGeneral => "General",
        TabShortcuts => "Shortcuts",
        TabAppearance => "Appearance",
        TabWindows => "Windows",
        TabAbout => "About",

        Startup => "Startup",
        LaunchAtLogin => "Launch at login",
        ShowStatusItem => "Show icon in menu bar",
        StatusItemNote => "When the icon is hidden, open the app again (Finder or Spotlight) to show settings.",
        Mouse => "Mouse",
        MouseSelection => "Mouse selection in panel",
        MouseSelectionNote => "When on, hovering selects an item and clicking switches to that window.",
        Panel => "Panel",
        ShowDelay => "Panel delay",
        Immediately => "Immediately",
        ShowDelayNote => "If the shortcut is released before the delay, it switches without showing the panel.",
        PanelScreen => "Show panel on",
        ScreenFilter => "\"Current screen\" filter",
        ActiveWindowScreen => "Screen of active window",
        CursorScreen => "Screen with mouse pointer",
        PrimaryScreen => "Main screen",
        AllScreens => "All screens",
        Language => "Language",
        LanguageSystem => "System language",
        Logs => "Logs",
        LogLevel => "Log level",
        LogLevelError => "Errors only",
        LogLevelWarn => "Warnings and errors",
        LogLevelInfo => "Info",
        LogLevelDebug => "Detailed (debugging)",
        LogLevelTrace => "Very detailed",
        LogsNote => {
            "When reporting a problem, set the level to \"Detailed\", reproduce the problem and send the log file. The file keeps the newest 2000 lines."
        }
        ShowLogFile => "Show log file in Finder",

        Theme => "Theme",
        ThemeSystem => "System",
        ThemeLight => "Light",
        ThemeDark => "Dark",
        Orientation => "Orientation",
        OrientationHorizontal => "Horizontal (side by side, title below)",
        OrientationVertical => "Vertical (stacked, title on the right)",
        Sort => "Sort order",
        SortRecent => "Recently used",
        SortAppName => "App name",
        SortGrouped => "Grouped by app",
        Items => "Items",
        ItemSize => "Item size",
        IconSize => "Icon size",
        SizeSmall => "Small",
        SizeMedium => "Medium",
        SizeLarge => "Large",
        SizeExtraLarge => "Extra large",
        ItemPadding => "Item padding",
        ItemSpacing => "Spacing between items",
        StyleNote => "The style (icons / previews / list) is chosen per shortcut in the Shortcuts tab.",
        Text => "Text",
        FontFamily => "Font",
        SystemFont => "System font",
        FontSize => "Font size",
        TitleLines => "Title lines",
        OneLine => "1 line",
        TwoLines => "2 lines",
        ThreeLines => "3 lines",

        WindowKinds => "Window types to show",
        Dialogs => "Dialogs",
        DialogsNote => "Small standalone windows such as \"Get Info\".",
        ModalDialogs => "Dialogs that block their main window",
        ModalDialogsNote => {
            "E.g. PhpStorm Settings. When off, switching to the main window also brings the dialog forward."
        }
        FloatingPanels => "Floating panels",
        FloatingPanelsNote => "Fonts, Colors, tool palettes.",
        SheetsNote => "Sheets attached to a window (Save, Print) are never shown as separate items.",
        Blacklist => "Blacklist",
        BlacklistNote => "Windows of these apps are never shown.",
        Remove => "Remove",
        AddApp => "Add app",
        ChooseApp => "Choose an app…",

        ShortcutsIntro => "To change keys, click the \"Keys\" button and press the new combination.",
        Enabled => "Enabled",
        Delete => "Delete",
        Keys => "Keys",
        Style => "Style",
        StyleIcons => "Icons",
        StylePreviews => "Previews",
        StylePreviewsWithIcon => "Previews + icon in corner",
        StyleList => "List",
        Apps => "Apps",
        AllApps => "All apps",
        CurrentApp => "Current app",
        Spaces => "Desktops",
        AllSpaces => "All desktops",
        CurrentSpace => "Current desktop",
        Screens => "Screens",
        CurrentScreen => "Current screen",
        MinimizedWindows => "Minimized windows",
        HiddenApps => "Hidden apps",
        Show => "Show",
        ShowAtEnd => "Show at end",
        Hide => "Hide",
        AddShortcut => "+ Add shortcut",
        PanelKeys => "Keys while the panel is open",
        CloseWindow => "Close window",
        Minimize => "Minimize",
        QuitApp => "Quit app",
        HideApp => "Hide app",
        PressKeys => "Press keys… (Esc: cancel)",

        Donate => "Donate",
        DonateNote => {
            "Alt-Tab Mac is free and will always stay free. As a software developer who is deeply bothered by \
             projects that start as open source and later turn paid, and by charging money for simple things, \
             I built this project for all of you. That said, if you would like to donate and support me, you can \
             send any amount you like to my crypto addresses below. It would make me very happy. In the future, \
             I will list the people who donated here. Thank you in advance!"
        }
        Copy => "Copy",
        Copied => "Copied ✓",
        BinanceUserId => "Binance User ID (UID)",
        BinanceUsername => "Binance username",

        AppDescription => "A fast, window-based switcher for macOS.",
        License => "License",
        LicenseText => {
            "Free software distributed under the GNU General Public License 3.0 (or later). \
                        Closed-source use requires a separate commercial license."
        }
    }
}

pub fn template(template: Template, value: &str) -> String {
    match template {
        Template::ScreenName => format!("Screen: {value}"),
        Template::ShortcutTitle => format!("Shortcut {value}"),
        Template::Version => format!("Version {value}"),
        Template::ConfigFile => format!("Config file: {value}"),
    }
}
