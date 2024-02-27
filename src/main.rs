


use crate::prelude::*;
use crate::utils::cli::get_user_input;
use crate::utils::user_prefs::load_city;
use crate::utils::user_prefs::save_city;
use crate::utils::weather_types::*;


// Error handling
mod error;
mod prelude;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    //let city = cli::get_user_input("Enter a city: ")?;
    let default_city = load_city()?;
    let prompt = match &default_city {
        Some(city) => format!("Enter a city (recent is {}): ", city),
        None => String::from("Enter a city: "),
    };

    let mut city = get_user_input(prompt.as_str())?;
    if city.is_empty() {
        city = default_city.unwrap_or_else(|| String::from("Unknown City"));
    }
    
    save_city(&city)?;
    let weather = CurrentWeather::fetch_weather(&city).await?;
    println!("The temperature in {} is {}°C", city, weather.get_temp());
    println!("The weather is: {}", weather.get_weather_descriptions()[0]);
    println!("The cloud cover is: {}", weather.get_cloudcover());
    Ok(())
}