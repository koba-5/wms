pub(crate) fn from_env(key: &str) -> String {
    std::env::var(key)
        .expect(&format!("Environment variable not found(key={})", key))
}
