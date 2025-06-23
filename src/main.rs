use dotenv::dotenv;
use serde::Deserialize;
use std::{collections::HashMap, env};

#[derive(Debug, Deserialize)]
struct Coordinates {
    lat: f32,
    lon: f32,
}
#[derive(Debug, Deserialize)]
struct WeatherMain {
    temp: f32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: WeatherMain,
}

struct WeatherClient {
    api_key: String,
    client: reqwest::Client,
}

impl WeatherClient {
    fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    async fn get_location_by_city(
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

        println!("Request URL: {}", request.url());
        let resp = self.client.execute(request).await?;

        // check if the request was successful
        println!("Response status: {}", resp.status());

        let resp_json = resp.json::<Vec<Coordinates>>().await?;
        Ok(resp_json)
    }

    async fn get_location_by_zip(
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

        println!("Request URL: {}", request.url());
        let resp = self.client.execute(request).await?;
        println!("Response status: {}", resp.status());

        let location = resp.json::<Coordinates>().await?;
        Ok(location)
    }

    async fn get_current_weather(
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

        println!("Request URL: {}", request.url());
        let resp = self.client.execute(request).await?;
        println!("Response status: {}", resp.status());

        let weather_response = resp.json::<WeatherResponse>().await?;
        Ok(weather_response.main)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load env variables
    dotenv().ok();
    let api_key = env::var("API_KEY")?;

    let weather = WeatherClient::new(api_key);

    let city_locations = weather.get_location_by_city("Framingham").await?;
    println!(
        "lat: {}, lon: {}",
        city_locations[0].lat, city_locations[0].lon
    );

    // zip lookup
    let zip_location = weather.get_location_by_zip("01701").await?;
    println!("Zip search result: {:#?}", zip_location);
    println!(
        "Zip coordinates: {}, {}",
        zip_location.lat, zip_location.lon
    );

    let current_weather = weather
        .get_current_weather(String::from("imperial"), zip_location)
        .await?;

    println!("Current Weather: {:#?}", current_weather);

    Ok(())
}
