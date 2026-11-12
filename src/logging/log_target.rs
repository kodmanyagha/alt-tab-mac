// SPDX-License-Identifier: GPL-3.0-or-later

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LogTarget {
    StderrOnly,
    StderrAndFile,
}
