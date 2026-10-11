use crate::framework::config::from_env;

pub(crate) struct Config {
    pub ip: String,
    pub port: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            ip: from_env("SERVER_IP"),
            port: from_env("SERVER_PORT"),
        }
    }
}
