// SPDX-License-Identifier: GPL-3.0-or-later

use std::env;
use std::path::PathBuf;

use crate::logging::LogTarget;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    App,
    Diagnose,
    RenderSettings(PathBuf),
    RenderPanel(PathBuf),
}

impl Mode {
    /// Developer tools must not truncate the running app's log file.
    pub fn log_target(&self) -> LogTarget {
        match self {
            Mode::App => LogTarget::StderrAndFile,
            Mode::Diagnose | Mode::RenderSettings(_) | Mode::RenderPanel(_) => LogTarget::StderrOnly,
        }
    }
}

pub fn parse() -> Mode {
    let args: Vec<String> = env::args().collect();
    parse_args(&args)
}

fn parse_args(args: &[String]) -> Mode {
    if args.iter().any(|arg| arg == "--diagnose") {
        return Mode::Diagnose;
    }
    if let Some(dir) = value_after(args, "--render-settings") {
        return Mode::RenderSettings(dir);
    }
    if let Some(dir) = value_after(args, "--render-panel") {
        return Mode::RenderPanel(dir);
    }
    Mode::App
}

/// Value of an argument in `--flag value` form.
fn value_after(args: &[String], flag: &str) -> Option<PathBuf> {
    let index = args.iter().position(|arg| arg == flag)?;
    args.get(index + 1).map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter().map(|arg| arg.to_string()).collect()
    }

    #[test]
    fn no_flags_runs_the_app() {
        assert_eq!(parse_args(&args(&["alt-tab-mac"])), Mode::App);
    }

    #[test]
    fn render_flag_takes_the_following_directory() {
        let mode = parse_args(&args(&["alt-tab-mac", "--render-panel", "/tmp/out"]));
        assert_eq!(mode, Mode::RenderPanel(PathBuf::from("/tmp/out")));
    }

    #[test]
    fn render_flag_without_directory_runs_the_app() {
        assert_eq!(parse_args(&args(&["alt-tab-mac", "--render-settings"])), Mode::App);
    }
}
