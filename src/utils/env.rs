use std::env;

pub fn load_env() {
    const MY_COMPILED_VAR: &str = env!("API_KEY_EX");
    println!("MY_COMPILED_VAR: {}", MY_COMPILED_VAR);
}