// SPDX-License-Identifier: GPL-3.0-or-later

use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{self, ErrorKind, Write};
use std::path::{Path, PathBuf};

/// Lines kept when the file is trimmed.
const MAX_LINES: usize = 2000;
/// Extra lines allowed before trimming again, so the file is not rewritten on every line.
const TRIM_SLACK: usize = 500;

/// `~/Library/Logs` is where macOS expects app logs; Console.app lists files there.
pub fn log_path() -> PathBuf {
    let home = env::var_os("HOME").map(PathBuf::from).unwrap_or_default();
    home.join("Library/Logs/alt-tab-mac.log")
}

/// Append-only log file that keeps only the newest `MAX_LINES` lines. Owned by the background log thread, so it needs
/// no lock.
pub struct LogFile {
    path: PathBuf,
    file: File,
    line_count: usize,
}

impl LogFile {
    /// Trims what earlier runs left behind and opens the file for appending.
    pub fn open(path: PathBuf) -> io::Result<Self> {
        let line_count = trim_file(&path)?;
        let file = OpenOptions::new().create(true).append(true).open(&path)?;
        Ok(Self { path, file, line_count })
    }

    pub fn write_all(&mut self, buffer: &[u8]) -> io::Result<()> {
        self.file.write_all(buffer)?;
        self.line_count += count_lines(buffer);
        if self.line_count >= MAX_LINES + TRIM_SLACK {
            self.trim()?;
        }
        Ok(())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }

    /// The handle is in append mode, so it keeps writing at the new end after the file is rewritten.
    fn trim(&mut self) -> io::Result<()> {
        self.file.flush()?;
        self.line_count = trim_file(&self.path)?;
        Ok(())
    }
}

/// Rewrites the file with only its newest `MAX_LINES` lines and returns how many lines it now has.
fn trim_file(path: &Path) -> io::Result<usize> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(0),
        Err(error) => return Err(error),
    };
    let kept = newest_lines(&bytes, MAX_LINES);
    if kept.len() < bytes.len() {
        fs::write(path, kept)?;
    }
    Ok(count_lines(kept))
}

/// The tail of `bytes` holding at most `max_lines` newline-terminated lines.
fn newest_lines(bytes: &[u8], max_lines: usize) -> &[u8] {
    let mut newlines_seen = 0;
    for (index, &byte) in bytes.iter().enumerate().rev() {
        if byte == b'\n' {
            newlines_seen += 1;
            if newlines_seen > max_lines {
                return &bytes[index + 1..];
            }
        }
    }
    bytes
}

fn count_lines(bytes: &[u8]) -> usize {
    bytes.iter().filter(|&&byte| byte == b'\n').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_log_keeps_only_the_newest_lines() {
        assert_eq!(newest_lines(b"1\n2\n3\n4\n", 2), b"3\n4\n");
    }

    #[test]
    fn short_log_is_left_untouched() {
        assert_eq!(newest_lines(b"1\n2\n", 5), b"1\n2\n");
    }

    #[test]
    fn empty_log_stays_empty() {
        assert_eq!(newest_lines(b"", 5), b"");
    }

    #[test]
    fn trimming_a_file_rewrites_it_with_the_newest_lines() {
        let path = env::temp_dir().join(format!("alt-tab-mac-trim-test-{}.log", std::process::id()));
        let lines: String = (0..MAX_LINES + 10).map(|n| format!("{n}\n")).collect();
        fs::write(&path, lines).expect("temp file is writable");
        let kept = trim_file(&path).expect("temp file is readable");
        let content = fs::read_to_string(&path).expect("temp file is readable");
        fs::remove_file(&path).expect("temp file is removable");
        assert_eq!(kept, MAX_LINES);
        assert!(content.starts_with("10\n"), "the 10 oldest lines must be gone, file starts with {:?}", &content[..8]);
    }

    #[test]
    fn missing_file_counts_as_empty() {
        let path = env::temp_dir().join("alt-tab-mac-trim-test-missing.log");
        assert_eq!(trim_file(&path).expect("missing file is not an error"), 0);
    }
}
