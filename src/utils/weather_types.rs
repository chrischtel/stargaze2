use serde::Deserialize;

use crate::prelude::*;

use super::env;

#[derive(Deserialize)]
pub struct Weather {
    pub current: CurrentWeather,
}

#[derive(Deserialize)]
pub struct CurrentWeather {
    pub temperature: f64,
    pub weather_descriptions: Vec<String>,
    pub cloudcover: i64,
}

impl CurrentWeather {
     pub async fn fetch_weather(city: &str) -> Result<CurrentWeather> {
        let api_key = env::load_env();
        let url = format!("http://api.weatherstack.com/current?access_key={}&query={}", api_key, &city);
        let response = reqwest::get(url).await.unwrap().json::<Weather>().await;

        match response {
            Err(e) => return Err(Error::Reqwest(e)),
            Ok(weather) => Ok(weather.current),
            
        }     
    }
}

//TODO: Implement Forecast struct


// traits
pub trait WeatherData {
    fn get_temp(&self) -> f64;
    fn get_weather_descriptions(&self) -> &Vec<String>;
    fn get_cloudcover(&self) -> i64;
}

impl WeatherData for CurrentWeather {
    fn get_temp(&self) -> f64 {
        self.temperature
    }

    fn get_weather_descriptions(&self) -> &Vec<String> {
        &self.weather_descriptions
    }

    fn get_cloudcover(&self) -> i64 {
        self.cloudcover
    }
}

