use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;

/// A single geocoding result from OSM
#[derive(Debug, Deserialize, Clone)]
pub struct OSMResponse {
    #[serde(rename = "lat")]
    latitude: String,
    #[serde(rename = "lon")]
    longitude: String,
    #[serde(rename = "display_name")]
    pub name: String,
}

/// Sanitize the search input to make it URL-safe and compatible with OSM
fn sanitize_input(input: &str) -> String {
    // Remove special characters (keep alphanumeric, spaces, commas)
    let re = Regex::new(r"[^a-zA-Z0-9 ,]").unwrap();
    let cleaned = re.replace_all(input, "");

    // Replace spaces with '+' for the URL
    cleaned.trim().replace(' ', "+")
}

/// Search OpenStreetMap Nominatim for a given location string
pub async fn geocode(
    search: &str,
) -> Result<(String, Vec<(f32, f32)>), Box<dyn Error + Send + Sync>> {
    let sanitized = sanitize_input(search);

    let url = format!(
        "https://nominatim.openstreetmap.org/search?q={}&format=json&limit=1",
        sanitized
    );
    let client = Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", "rust-geocoder")
        .send()
        .await?;
    let data = resp.json::<Vec<OSMResponse>>().await?;

    fn geoloaction(location: OSMResponse) -> (f32, f32) {
        let lat: f32 = location.latitude.parse().unwrap();
        let lon: f32 = location.longitude.parse().unwrap();
        (lat, lon)
    }

    let result = data.iter().map(|i| geoloaction(i.clone())).collect();

    Ok((data.first().unwrap().name.clone(), result))
}
