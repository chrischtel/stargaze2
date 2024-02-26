#[allow(unused)]

use crate::prelude::*;

// Error handling
mod error;
mod prelude;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    Ok(())
}