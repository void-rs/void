use std::{env, fs::OpenOptions, io::Write, sync::RwLock};

use log::{self, Level, LevelFilter, Metadata, Record, SetLoggerError};

struct ScreenLogger;

static SCREEN_LOGGER: ScreenLogger = ScreenLogger;

impl log::Log for ScreenLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let line = format!("{} - {}", record.level(), record.args());
            let mut logs = LOGS.write().unwrap();
            logs.insert(0, line);
            logs.truncate(5);
        }
        let line = format!(
            "{} {} {}:{}] {}\n",
            crate::now().as_secs(),
            record.level(),
            record.file().unwrap().split('/').last().unwrap(),
            record.line().unwrap(),
            record.args()
        );

        if let Ok(path) = env::var("LOGFILE") {
            let mut f = OpenOptions::new()
                .append(true)
                .create(true)
                .open(path)
                .unwrap();
            f.write_all(line.as_bytes()).unwrap();
        }
    }

    fn flush(&self) {}
}

pub fn init_screen_log() -> Result<(), SetLoggerError> {
    log::set_max_level(LevelFilter::Debug);
    log::set_logger(&SCREEN_LOGGER)
}

pub fn read_logs() -> Vec<String> {
    LOGS.read().unwrap().clone()
}

lazy_static! {
    static ref LOGS: RwLock<Vec<String>> = RwLock::new(vec![]);
}
