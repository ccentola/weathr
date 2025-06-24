use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Coordinates {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Debug, Deserialize)]
pub struct WeatherMain {
    pub temp: f32,
    pub temp_min: f32,
    pub temp_max: f32,
}

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub main: WeatherMain,
}
