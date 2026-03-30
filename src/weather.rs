//! Weather Data Integration Module
//! Fetches real weather data from Open-Meteo API (free, no key required)

use reqwest;
use serde::Deserialize;

#[derive(Debug)]
pub struct WeatherService {
    client: reqwest::Client,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct GeocodeResponse {
    results: Option<Vec<GeocodeResult>>,
}

#[derive(Debug, Deserialize)]
struct GeocodeResult {
    latitude: f64,
    longitude: f64,
    name: String,
    country: String,
}

#[derive(Debug, Deserialize)]
struct WeatherForecastResponse {
    current: CurrentWeather,
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature_2m: f64,
    relative_humidity_2m: f64,
    apparent_temperature: f64,
    precipitation: Option<f64>,
    rain: Option<f64>,
    snowfall: Option<f64>,
    weather_code: i32,
    cloud_cover: f64,
    wind_speed_10m: f64,
    wind_direction_10m: f64,
    wind_gusts_10m: f64,
}

#[derive(Debug, Clone)]
pub struct WeatherData {
    // Basic measurements
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: f64,
    pub precipitation: f64,
    pub rain: f64,
    pub snowfall: f64,

    // Wind data
    pub wind_speed: f64,
    pub wind_direction: f64,
    pub wind_gusts: f64,

    // Sky conditions
    pub cloud_cover: f64,
    pub weather_code: i32,

    // Derived conditions
    pub condition: String,
    pub is_raining: bool,
    pub is_snowing: bool,
    pub wind_category: String,

    // Location context
    pub location: String,
    pub country: String,

    // Coordinates (optional)
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl WeatherService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.open-meteo.com/v1".to_string(),
        }
    }

    /// Get coordinates for a location name using geocoding
    pub async fn get_coordinates(&self, location: &str) -> Result<GeocodeResult, Box<dyn std::error::Error>> {
        let url = format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={}&count=1",
            urlencoding::encode(location)
        );

        let response = self.client.get(&url).send().await?;
        if !response.status().is_success() {
            return Err(format!("Geocoding API error: {}", response.status()).into());
        }

        let data: GeocodeResponse = response.json().await?;
        
        match data.results.and_then(|mut r| r.pop()) {
            Some(result) => Ok(result),
            None => Err(format!("Location not found: {}", location).into()),
        }
    }

    /// Fetch current weather data for coordinates
    pub async fn get_weather(
        &self,
        latitude: f64,
        longitude: f64,
        location_info: Option<(&str, &str)>,
    ) -> Result<WeatherData, Box<dyn std::error::Error>> {
        let params = [
            ("latitude", latitude.to_string()),
            ("longitude", longitude.to_string()),
            (
                "current",
                "temperature_2m,relative_humidity_2m,apparent_temperature,\
                 precipitation,rain,snowfall,weather_code,cloud_cover,\
                 wind_speed_10m,wind_direction_10m,wind_gusts_10m"
                    .to_string(),
            ),
            ("timezone", "auto".to_string()),
        ];

        let url = format!("{}?{}", self.base_url, serde_qs::to_string(&params)?);

        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(format!("Weather API error: {}", response.status()).into());
        }

        let data: WeatherForecastResponse = response.json().await?;
        Ok(self.parse_weather_data(data.current, location_info))
    }

    /// Parse and enrich weather data
    fn parse_weather_data(&self, current: CurrentWeather, location_info: Option<(&str, &str)>) -> WeatherData {
        let code = current.weather_code;

        WeatherData {
            // Basic measurements
            temperature: current.temperature_2m,
            feels_like: current.apparent_temperature,
            humidity: current.relative_humidity_2m,
            precipitation: current.precipitation.unwrap_or(0.0),
            rain: current.rain.unwrap_or(0.0),
            snowfall: current.snowfall.unwrap_or(0.0),

            // Wind data
            wind_speed: current.wind_speed_10m,
            wind_direction: current.wind_direction_10m,
            wind_gusts: current.wind_gusts_10m,

            // Sky conditions
            cloud_cover: current.cloud_cover,
            weather_code: code,

            // Derived conditions
            condition: self.get_weather_condition(code),
            is_raining: current.rain.unwrap_or(0.0) > 0.0 || current.precipitation.unwrap_or(0.0) > 0.0,
            is_snowing: current.snowfall.unwrap_or(0.0) > 0.0,
            wind_category: self.categorize_wind(current.wind_speed_10m),

            // Location context
            location: location_info.map(|(name, _)| name.to_string()).unwrap_or_else(|| "Unknown".to_string()),
            country: location_info.map(|(_, country)| country.to_string()).unwrap_or_default(),

            // Coordinates
            latitude: None,
            longitude: None,
        }
    }

    /// Convert WMO weather code to human-readable condition
    fn get_weather_condition(&self, code: i32) -> String {
        match code {
            0 => "Clear Sky".to_string(),
            1 => "Mainly Clear".to_string(),
            2 => "Partly Cloudy".to_string(),
            3 => "Overcast".to_string(),
            45 | 48 => "Foggy".to_string(),
            51 | 53 | 55 => "Drizzle".to_string(),
            56 | 57 => "Freezing Drizzle".to_string(),
            61 | 63 | 65 => "Rain".to_string(),
            66 | 67 => "Freezing Rain".to_string(),
            71 | 73 | 75 => "Snow".to_string(),
            77 => "Snow Grains".to_string(),
            80 | 81 | 82 => "Rain Showers".to_string(),
            85 | 86 => "Snow Showers".to_string(),
            95..=99 => "Thunderstorm".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Categorize wind speed for visualization intensity
    fn categorize_wind(&self, speed: f64) -> String {
        if speed < 2.0 {
            "calm".to_string()
        } else if speed < 6.0 {
            "light".to_string()
        } else if speed < 12.0 {
            "moderate".to_string()
        } else if speed < 20.0 {
            "fresh".to_string()
        } else if speed < 30.0 {
            "strong".to_string()
        } else {
            "gale".to_string()
        }
    }

    /// Get complete weather for a location string
    pub async fn get_weather_for_location(&self, location: &str) -> Result<WeatherData, Box<dyn std::error::Error>> {
        let coords = self.get_coordinates(location).await?;
        let mut weather = self.get_weather(coords.latitude, coords.longitude, Some((&coords.name, &coords.country))).await?;
        
        weather.latitude = Some(coords.latitude);
        weather.longitude = Some(coords.longitude);
        
        Ok(weather)
    }
}

impl Default for WeatherService {
    fn default() -> Self {
        Self::new()
    }
}
