use std::io::{self, Write};

use crate::prelude::*;

pub fn get_user_input(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    match io::stdout().flush() {
        Ok(_) => {},
        Err(e) => return Err(Error::Io(e)),
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {},
        Err(e) => return Err(Error::Io(e)),
    }

    Ok(input.trim().to_string())
}