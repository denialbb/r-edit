use std::fs::{File, OpenOptions};
use std::io::Write;
use log::{self, LevelFilter};
use std::sync::Mutex;
use chrono::Local;

pub struct Config {
    pub level_filter: LevelFilter,
    pub truncate: bool,
}

pub struct CustomLogger {
    config: Config,
    log_file: Mutex<File>,
}

impl CustomLogger {
    pub fn new(config: Config, log_file_path: &str) -> Result<Self, std::io::Error> {
        let mut open_options = OpenOptions::new();
        if config.truncate {
            open_options.create(true).write(true).truncate(true);
        } else {
            open_options.create(true).append(true);
        }
        let log_file = open_options.open(log_file_path)?;
        Ok(CustomLogger { config, log_file: Mutex::new(log_file) })
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

        let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let log_entry = format!(
            "{} [{}] {}\n",
            time,
            record.level(),
            record.args()
        );

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
