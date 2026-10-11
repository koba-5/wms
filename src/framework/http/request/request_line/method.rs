use crate::framework::http::request::request_line::method::Method::{
    Get, Post,
};
use serde::Serialize;
use std::str::FromStr;

#[derive(Serialize)]
pub(crate) enum Method {
    Get,
    Post,
}

impl FromStr for Method {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Get),
            "POST" => Ok(Post),
            _ => Err("The Method is invalid."),
        }
    }
}
