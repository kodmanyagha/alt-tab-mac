// SPDX-License-Identifier: GPL-3.0-or-later

use std::env;
use std::sync::{Once, OnceLock};

use parking_lot::Mutex;
use tracing::level_filters::LevelFilter;
use tracing_appender::non_blocking::{NonBlocking, NonBlockingBuilder, WorkerGuard};
use tracing_subscriber::Registry;
use tracing_subscriber::reload::{Handle, Layer};

use super::log_file::{LogFile, log_path};
use super::log_level::LogLevel;
use super::log_target::LogTarget;
use super::tee_writer::TeeWriter;

/// When set (`error`, `warn`, `info`, `debug`, `trace`), overrides the level chosen in the settings.
const LEVEL_VARIABLE: &str = "ALT_TAB_LOG";
const WRITER_THREAD_NAME: &str = "log-writer";

static INIT: Once = Once::new();
static WRITER_GUARD: Mutex<Option<WorkerGuard>> = Mutex::new(None);
static LEVEL_HANDLE: OnceLock<Handle<LevelFilter, Registry>> = OnceLock::new();

/// Flushes the background log writer when dropped.
pub struct LogGuard;

impl Drop for LogGuard {
    fn drop(&mut self) {
        flush();
    }
}

/// Log lines are written by a background thread (`tracing_appender::non_blocking`), so a slow terminal or disk never
/// stalls the thread that logs. No line is dropped: only when the buffer (128k lines) is full does the logging thread
/// wait. The app starts at `LogLevel::default()` until `set_level` applies the settings; developer tools use
/// `LogTarget::StderrOnly`, log at info level and leave the app's log file alone.
#[must_use = "dropping the guard flushes and stops the log writer"]
pub fn init(target: LogTarget) -> LogGuard {
    INIT.call_once(|| {
        let writer = start_writer(target);
        install_subscriber(writer, initial_level(target));
    });
    LogGuard
}

/// Writes out the buffered log lines and stops the writer; later lines are not written.
pub fn flush() {
    let guard = WRITER_GUARD.lock().take();
    drop(guard);
}

/// Applies the level chosen in the settings while the app runs. `ALT_TAB_LOG` wins when it is set.
pub fn set_level(level: LogLevel) {
    if level_from_env().is_some() {
        return;
    }
    let Some(handle) = LEVEL_HANDLE.get() else {
        return;
    };
    if let Err(error) = handle.modify(|filter| *filter = LevelFilter::from(level)) {
        tracing::error!(%error, "could not change the log level");
    }
}

fn initial_level(target: LogTarget) -> LevelFilter {
    let default = match target {
        LogTarget::StderrAndFile => LevelFilter::from(LogLevel::default()),
        LogTarget::StderrOnly => LevelFilter::INFO,
    };
    level_from_env().unwrap_or(default)
}

fn level_from_env() -> Option<LevelFilter> {
    env::var(LEVEL_VARIABLE).ok().and_then(|value| value.parse().ok())
}

fn start_writer(target: LogTarget) -> NonBlocking {
    let file = match target {
        LogTarget::StderrAndFile => open_log_file(),
        LogTarget::StderrOnly => None,
    };
    let (writer, guard) =
        NonBlockingBuilder::default().lossy(false).thread_name(WRITER_THREAD_NAME).finish(TeeWriter::new(file));
    *WRITER_GUARD.lock() = Some(guard);
    writer
}

/// Logging is not up yet, so a failure can only be reported on stderr.
fn open_log_file() -> Option<LogFile> {
    let path = log_path();
    match LogFile::open(path.clone()) {
        Ok(file) => Some(file),
        Err(error) => {
            eprintln!("could not open the log file {}: {error}", path.display());
            None
        }
    }
}

fn install_subscriber(writer: NonBlocking, level: LevelFilter) {
    use tracing_subscriber::layer::SubscriberExt;

    let (filter, handle) = Layer::new(level);
    let output = tracing_subscriber::fmt::layer()
        .with_writer(writer)
        .with_ansi(false)
        .with_target(false)
        .with_file(true)
        .with_line_number(true);
    let subscriber = tracing_subscriber::registry().with(filter).with(output);
    if tracing::subscriber::set_global_default(subscriber).is_ok() {
        let _first_init_only = LEVEL_HANDLE.set(handle);
    }
}
