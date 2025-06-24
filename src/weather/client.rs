use super::types::{Coordinates, WeatherMain, WeatherResponse};
use std::collections::HashMap;

pub struct WeatherClient {
    api_key: String,
    client: reqwest::Client,
}

impl WeatherClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_location_by_city(
        &self,
        city: &str,
    ) -> Result<Vec<Coordinates>, Box<dyn std::error::Error>> {
        let mut params = HashMap::new();
        params.insert("q", city);
        params.insert("limit", "1");
        params.insert("appid", &self.api_key);

        let request = self
            .client
            .get("http://api.openweathermap.org/geo/1.0/direct")
            .query(&params)
            .build()?;

        let resp = self.client.execute(request).await?;

        let resp_json = resp.json::<Vec<Coordinates>>().await?;
        Ok(resp_json)
    }

    pub async fn get_location_by_zip(
        &self,
        zipcode: &str,
    ) -> Result<Coordinates, Box<dyn std::error::Error>> {
        let mut params = HashMap::new();
        params.insert("zip", zipcode);
        params.insert("appid", &self.api_key);

        let request = self
            .client
            .get("http://api.openweathermap.org/geo/1.0/zip")
            .query(&params)
            .build()?;

        let resp = self.client.execute(request).await?;

        let location = resp.json::<Coordinates>().await?;
        Ok(location)
    }

    pub async fn get_current_weather(
        &self,
        units: String,
        coords: Coordinates,
    ) -> Result<WeatherMain, Box<dyn std::error::Error>> {
        let mut params = HashMap::new();
        let lat_str = coords.lat.to_string();
        let lon_str = coords.lon.to_string();
        params.insert("lat", &lat_str);
        params.insert("lon", &lon_str);
        params.insert("appid", &self.api_key);
        params.insert("units", &units);

        let request = self
            .client
            .get("https://api.openweathermap.org/data/2.5/weather")
            .query(&params)
            .build()?;

        let resp = self.client.execute(request).await?;

        let weather_response = resp.json::<WeatherResponse>().await?;
        Ok(weather_response.main)
    }
}
