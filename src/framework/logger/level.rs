use serde::Serialize;

#[derive(Serialize, Copy, Clone)]
pub(crate) enum Level {
    Info,
    Warn,
    Error,
}
