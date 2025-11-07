use log::{self, LevelFilter};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::sync::Mutex;
use std::time::SystemTime;

pub struct Config {
    pub level_filter: LevelFilter,
}

pub struct CustomLogger {
    config: Config,
    log_file: Mutex<File>,
}

impl CustomLogger {
    pub fn new(config: Config, log_file_path: &str) -> Result<Self, std::io::Error> {
        let log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file_path)?;
        Ok(CustomLogger {
            config,
            log_file: Mutex::new(log_file),
        })
    }
}

impl log::Log for CustomLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.config.level_filter
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let time = format!("{:?}", SystemTime::now());

        let log_entry = format!("{} [{}] {}\n", time, record.level(), record.args());

        let mut file = self.log_file.lock().unwrap();
        if let Err(e) = write!(file, "{}", log_entry) {
            eprintln!("Failed to write to log file: {}", e);
        }
    }

    fn flush(&self) {
        let mut file = self.log_file.lock().unwrap();
        let _ = file.flush();
    }
}
