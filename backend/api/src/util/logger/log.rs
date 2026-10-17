use crate::util::logger::level::Level;
use crate::util::time::now;
use serde::Serialize;

#[derive(Serialize)]
pub(super) struct Log<'a> {
    level: Level,
    timestamp: String,
    message: &'a str,
}

impl<'a> Log<'a> {
    pub(super) fn new(level: Level, message: &'a str) -> Self {
        Self {
            level,
            timestamp: now().to_string(),
            message,
        }
    }
}
