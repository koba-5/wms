use axum::http::StatusCode;

pub struct Error {
    cause: StatusCode,
    message: String,
}

impl Error {
    pub fn new(cause: StatusCode, message: String) -> Self {
        Self { cause, message }
    }
}
