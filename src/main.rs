


use std::env;

use crate::prelude::*;
use crate::utils::cli::get_user_input;
use crate::utils::user_prefs::delete_prefs;
use crate::utils::user_prefs::load_city;
use crate::utils::user_prefs::save_city;
use crate::utils::weather_types::*;


// Error handling
mod error;
mod prelude;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "clean" {
        delete_prefs()?;
        println!("Successfully cleaned user preferences.");
        return Ok(());
    }

    let default_city = load_city()?;
    let prompt = match &default_city {
        Some(city) => format!("Enter a city (default is {}): ", city),
        None => String::from("Enter a city: "),
    };

    let mut city = get_user_input(&prompt)?;
    if city.is_empty() {
        match default_city {
            Some(default_city) => city = default_city,
            None => {
                println!("Please enter a city.");
                return Ok(());
            }
        }
    }

    // Always save the city entered by the user
    save_city(&city)?;


    let weather = CurrentWeather::fetch_weather(&city).await?;
    println!("The temperature in {} is {}°C", city, weather.get_temp());
    println!("The weather is: {}", weather.get_weather_descriptions()[0]);
    println!("The cloud cover is: {}", weather.get_cloudcover());
    Ok(())
}