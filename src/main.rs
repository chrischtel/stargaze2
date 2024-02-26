#[allow(unused)]

use crate::prelude::*;
use crate::utils::env;
// Error handling
mod error;
mod prelude;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    env::load_env();
    Ok(())
}