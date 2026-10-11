use crate::framework::logger::level::Level;
use crate::framework::logger::log::Log;

mod level;
mod log;

pub(crate) struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn info(&self, message: &str) {
        self.output(Level::Info, message)
    }

    pub fn warn(&self, message: &str) {
        self.output(Level::Warn, message)
    }

    pub fn error(&self, message: &str) {
        self.output(Level::Error, message)
    }

    fn output(&self, level: Level, message: &str) {
        let log = Log::new(level, message);
        let line =
            serde_json::to_string(&log).expect("failed to serialize log");

        match level {
            Level::Info => println!("\x1b[32m{}\x1b[0m", line),
            Level::Warn => println!("\x1b[33m{}\x1b[0m", line),
            Level::Error => eprintln!("\x1b[31m{}\x1b[0m", line),
        }
    }
}
