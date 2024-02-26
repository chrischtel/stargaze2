use std::env;

pub fn load_env() -> &'static str {
    const API_KEY: &str = env!("API_KEY_EX");
    API_KEY
}