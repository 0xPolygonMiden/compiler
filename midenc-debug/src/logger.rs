use std::{
    borrow::Cow,
    collections::VecDeque,
    sync::{Arc, LazyLock, Mutex},
};

use log::{Level, Log};

static LOGGER: LazyLock<DebugLogger> = LazyLock::new(DebugLogger::default);

#[derive(Default)]
struct DebugLoggerImpl {
    inner: Option<Box<dyn Log>>,
    captured: VecDeque<LogEntry>,
}

pub struct LogEntry {
    pub level: Level,
    pub file: Option<Cow<'static, str>>,
    pub line: Option<u32>,
    pub message: String,
}

#[derive(Default, Clone)]
pub struct DebugLogger(Arc<Mutex<DebugLoggerImpl>>);
impl Log for DebugLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let file = record
            .file_static()
            .map(Cow::Borrowed)
            .or_else(|| record.file().map(|f| f.to_string()).map(Cow::Owned));
        let entry = LogEntry {
            level: record.level(),
            file,
            line: record.line(),
            message: format!("{}", record.args()),
        };
        let mut guard = self.0.lock().unwrap();
        guard.captured.push_back(entry);
        if guard.captured.len() > 100 {
            guard.captured.pop_front();
        }
        if let Some(inner) = guard.inner.as_ref() {
            if inner.enabled(record.metadata()) {
                inner.log(record);
            }
        }
    }

    fn flush(&self) {}
}
impl DebugLogger {
    pub fn install(inner: Box<dyn Log>) {
        let logger = &*LOGGER;
        logger.set_inner(inner);
        log::set_logger(logger).unwrap_or_else(|err| panic!("failed to install logger: {err}"));
        log::set_max_level(log::LevelFilter::Trace);
    }

    pub fn get() -> &'static Self {
        &LOGGER
    }

    pub fn take_captured(&self) -> VecDeque<LogEntry> {
        let mut guard = self.0.lock().unwrap();
        core::mem::take(&mut guard.captured)
    }

    fn set_inner(&self, logger: Box<dyn Log>) {
        drop(self.0.lock().unwrap().inner.replace(logger));
    }
}
