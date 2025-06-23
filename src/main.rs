use dotenv::dotenv;
use serde::Deserialize;
use std::{collections::HashMap, env};

#[derive(Debug, Deserialize)]
struct Location {
    lat: f32,
    lon: f32,
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
    ) -> Result<Vec<Location>, Box<dyn std::error::Error>> {
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

        let resp_json = resp.json::<Vec<Location>>().await?;
        Ok(resp_json)
    }

    async fn get_location_by_zip(
        &self,
        zipcode: &str,
    ) -> Result<Location, Box<dyn std::error::Error>> {
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

        let location = resp.json::<Location>().await?;
        Ok(location)
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

    Ok(())
}
