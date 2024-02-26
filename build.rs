fn main() {
    if let Ok(value) = std::env::var("API_KEY") {
        println!("cargo:rustc-env=API_KEY_EX={}", value);
    }
}