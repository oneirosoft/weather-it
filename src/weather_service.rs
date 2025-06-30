use std::error::Error;

use crate::data::location::geocode;
use crate::data::weather::OpenMeteoResponse;
use crate::data::weather::fetch_weather;

pub struct WeatherData {
    pub weather: OpenMeteoResponse,
    pub location_name: String,
}

pub async fn dispatch_weather(query: &str) -> Result<WeatherData, Box<dyn Error + Send + Sync>> {
    let (name, geocode) = geocode(query).await?;

    let (lat, lon) = geocode.first().cloned().unwrap();

    let weather = fetch_weather(lat, lon).await?;

    Ok(WeatherData {
        weather,
        location_name: name,
    })
}
