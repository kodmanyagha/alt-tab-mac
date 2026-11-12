// SPDX-License-Identifier: GPL-3.0-or-later

use std::fs;
use std::path::PathBuf;

use crate::macos::bundle;
use std::io::ErrorKind;

fn plist_path() -> PathBuf {
    let home = std::env::var_os("HOME").map(PathBuf::from).unwrap_or_default();
    home.join("Library/LaunchAgents").join(format!("{}.plist", bundle::identifier()))
}

pub fn is_enabled() -> bool {
    plist_path().exists()
}

pub fn set_enabled(enabled: bool) -> Result<(), String> {
    let path = plist_path();
    if !enabled {
        return match fs::remove_file(&path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e.to_string()),
        };
    }
    let executable = bundle::executable_path().ok_or("executable path not found")?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    fs::write(&path, plist(&executable.to_string_lossy())).map_err(|e| e.to_string())
}

fn plist(executable: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{label}</string>
    <key>ProgramArguments</key>
    <array><string>{executable}</string></array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>
"#,
        label = bundle::identifier(),
        executable = xml_escape(executable),
    )
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}
