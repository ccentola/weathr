mod weather;

use dotenv::dotenv;
use std::env;
use std::io;
use weather::WeatherClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load env variables
    dotenv().ok();
    let api_key = env::var("API_KEY")?;
    let weather = WeatherClient::new(api_key);

    println!("=== * WEATHR * ===");
    println!("Please enter a U.S. zip code: ");

    let mut zipcode = String::new();
    io::stdin()
        .read_line(&mut zipcode)
        .expect("Please enter a valid U.S. zip code");

    let zipcode = zipcode.trim();
    println!("Checking weather for {}", zipcode);

    // zip lookup
    let zip_location = weather.get_location_by_zip(&zipcode).await?;

    let current_weather = weather
        .get_current_weather(String::from("imperial"), zip_location)
        .await?;
    println!("Current Weather: {:#?}", current_weather);

    Ok(())
}
