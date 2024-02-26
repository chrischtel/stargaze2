
use crate::prelude::*;
use crate::utils::weather_types::*;


// Error handling
mod error;
mod prelude;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let city = "New York";
    let weather = CurrentWeather::fetch_weather(city).await?;
    println!("The temperature in {} is {}°C", city, weather.get_temp());
    Ok(())
}