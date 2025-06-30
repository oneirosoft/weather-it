use iana_time_zone::get_timezone;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, Default)]
pub struct OpenMeteoResponse {
    pub hourly: OpenMeteoHourly,
    pub daily: OpenMeteoDaily,
    // pub current: OpenMeteoCurrent,
    // pub daily_units: DailyUnits,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct OpenMeteoHourly {
    #[serde(rename = "time")]
    pub date_time: Vec<String>,
    pub temperature_2m: Vec<f32>,
    // pub apparent_temperature: Vec<f32>,
    pub precipitation_probability: Vec<u32>,
    // pub relative_humidity_2m: Vec<u32>,
    #[serde(rename = "weathercode")]
    pub weather_code: Vec<u16>,
    // pub windspeed_10m: Vec<f32>,
    // pub winddirection_10m: Vec<f32>,
}

// #[derive(Debug, Deserialize, Default)]
// struct HourlyUnits {
//     temperature_2m: String,
//     precipitation_probability: String,
//     apparent_temperature: String,
// }

#[derive(Debug, Deserialize, Default, Clone)]
pub struct OpenMeteoDaily {
    #[serde(rename = "time")]
    pub date: Vec<String>,
    pub weather_code: Vec<u16>,
    // pub temperature_2m_min: Vec<f32>,
    pub temperature_2m_max: Vec<f32>,
    // pub apparent_temperature_min: Vec<f32>,
    pub apparent_temperature_max: Vec<f32>,
    pub precipitation_probability_max: Vec<f32>,
}

// #[derive(Debug, Deserialize, Default)]
// struct DailyUnits {
//     temperature_2m_max: String,
//     temperature_2m_min: String,
//     apparent_temperature_max: String,
//     apparent_temperature_min: String,
//     precipitation_probability_max: String,
// }

// #[derive(Debug, Deserialize, Default)]
// pub struct OpenMeteoCurrent {
//     #[serde(rename = "time")]
//     pub date_time: String,
//     pub temperature_2m: f32,
//     pub weather_code: u16,
//     pub apparent_temperature: f32,
//     pub precipitation: f32,
//     pub relative_humidity_2m: u32,
// }

// #[derive(Debug, Deserialize, Default)]
// struct CurrentUnits {
//     pub temperature_2m: String,
//     pub apparent_temperature: String,
//     pub relative_humidity_2m: String,
// }

pub async fn fetch_weather(
    latitude: f32,
    longitude: f32,
) -> Result<OpenMeteoResponse, Box<dyn Error + Send + Sync>> {
    let time_zone = get_timezone()?;

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?\
        latitude={}&\
        longitude={}&\
        hourly=temperature_2m,apparent_temperature,precipitation_probability,relative_humidity_2m,weathercode,windspeed_10m,winddirection_10m&\
        daily=weather_code,temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min,precipitation_probability_max&\
        current=temperature_2m,relative_humidity_2m,apparent_temperature,weather_code,precipitation&\
        temperature_unit=fahrenheit&\
        windspeed_unit=mph&\
        timezone={}&\
        forecast_days=7&",
        latitude, longitude, time_zone
    );
    let request = reqwest::get(&url);
    let response = request.await?;
    let result = response.json::<OpenMeteoResponse>().await?;

    Ok(result)
}

pub fn get_weather_description(code: u16) -> (&'static str, &'static str) {
    match code {
        0 => ("Clear sky", "☀️"),
        1 => ("Mainly clear", "🌤️"),
        2 => ("Partly cloudy", "⛅"),
        3 => ("Overcast", "☁️"),
        45 => ("Fog", "🌫️"),
        48 => ("Depositing rime fog", "🌫️❄️"),
        51 => ("Light drizzle", "🌦️"),
        53 => ("Moderate drizzle", "🌧️"),
        55 => ("Dense drizzle", "🌧️"),
        56 => ("Light freezing drizzle", "🌧️❄️"),
        57 => ("Dense freezing drizzle", "🌧️❄️"),
        61 => ("Slight rain", "🌦️"),
        63 => ("Moderate rain", "🌧️"),
        65 => ("Heavy rain", "🌧️🌧️"),
        66 => ("Light freezing rain", "🌧️❄️"),
        67 => ("Heavy freezing rain", "🌧️❄️❄️"),
        71 => ("Slight snow fall", "🌨️"),
        73 => ("Moderate snow fall", "🌨️❄️"),
        75 => ("Heavy snow fall", "❄️❄️"),
        77 => ("Snow grains", "🌨️🧂"),
        80 => ("Slight rain showers", "🌦️"),
        81 => ("Moderate rain showers", "🌧️"),
        82 => ("Violent rain showers", "⛈️"),
        85 => ("Slight snow showers", "🌨️"),
        86 => ("Heavy snow showers", "❄️❄️"),
        95 => ("Thunderstorm", "🌩️"),
        96 => ("Thunderstorm w/ hail", "⛈️🧊"),
        99 => ("Heavy TS w/ hail", "⛈️🧊🧊"),
        _ => ("Unknown", "❓"),
    }
}

pub fn get_cardinal_direction(degrees: f32) -> &'static str {
    let directions = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
        "NW", "NNW",
    ];
    let normalized = (degrees % 360.0 + 360.0) % 360.0;
    let index = (normalized / 22.5).round() as usize % 16;
    directions[index]
}
