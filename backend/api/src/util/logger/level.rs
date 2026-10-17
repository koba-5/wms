use serde::Serialize;

#[derive(Serialize)]
pub enum Level {
    Info,
    Warn,
    Error,
}
