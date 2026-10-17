use crate::util::logger::level::Level;
use crate::util::logger::log::Log;

mod level;
mod log;

pub struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn info(message: &str) {
        let line = Self::convert_to_line(Level::Info, message);
        println!("\x1b[38;2;160;160;160m{}\x1b[0m", line);
    }

    pub fn warn(message: &str) {
        let line = Self::convert_to_line(Level::Warn, message);
        println!("\x1b[33m{}\x1b[0m", line);
    }

    pub fn error(message: &str) {
        let line = Self::convert_to_line(Level::Error, message);
        eprintln!("\x1b[31m{}\x1b[0m", line);
    }

    fn convert_to_line(level: Level, message: &str) -> String {
        let log = Log::new(level, message);
        serde_json::to_string(&log).expect("Failed to serialize log")
    }
}
