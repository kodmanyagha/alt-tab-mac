// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::{self, Write};

use super::log_file::LogFile;

/// Writes every line to stderr and, when available, to the log file. Owned by the background log thread only.
pub struct TeeWriter {
    file: Option<LogFile>,
}

impl TeeWriter {
    pub fn new(file: Option<LogFile>) -> Self {
        Self { file }
    }
}

impl Write for TeeWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        io::stderr().write_all(buffer)?;
        if let Some(file) = &mut self.file {
            file.write_all(buffer)?;
        }
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stderr().flush()?;
        match &mut self.file {
            Some(file) => file.flush(),
            None => Ok(()),
        }
    }
}
