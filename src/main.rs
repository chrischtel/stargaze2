
use crate::prelude::*;
use crate::utils::weather_types::*;
use crate::utils::cli;


// Error handling
mod error;
mod prelude;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let city = cli::get_user_input("Enter a city: ")?;
    let weather = CurrentWeather::fetch_weather(&city).await?;
    println!("The temperature in {} is {}°C", city, weather.get_temp());
    Ok(())
}