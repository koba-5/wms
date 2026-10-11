use crate::framework::http::request::request_line::protocol::Protocol::Http11;
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize)]
pub(crate) enum Protocol {
    Http11,
}

impl FromStr for Protocol {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTTP/1.1" => Ok(Http11),
            _ => Err("The protocol is invalid."),
        }
    }
}
