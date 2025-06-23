mod weather;

use dotenv::dotenv;
use std::env;
use weather::WeatherClient;

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
