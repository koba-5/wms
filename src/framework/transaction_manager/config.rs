use crate::framework::config::from_env;

struct Config {
    username: String,
    password: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            username: from_env("DB_USERNAME"),
            password: from_env("DB_PASSWORD"),
        }
    }
}
