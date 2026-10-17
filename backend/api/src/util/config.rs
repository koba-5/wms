pub(crate) fn from_env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("Environment variable not found(key={key})"))
}
