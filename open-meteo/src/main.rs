use reqwest;
use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CurrentUnits {
    time: String,
    interval: String,
    temperature_2m: String,
    wind_speed_10m: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Current {
    time: String,
    interval: u64,
    temperature_2m: f64,
    wind_speed_10m: f64
}

#[derive(Debug, Serialize, Deserialize)]
struct HourlyUnits {
    time: String,
    temperature_2m: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Hourly {
    time: Vec<String>,
    temperature_2m: Vec<f64>
}


#[derive(Debug, Serialize, Deserialize)]
struct OpenMeteo {
    latitude: f64,
    longitude: f64,
    generationtime_ms: f64,
    utc_offset_seconds: u64,
    timezone: String,
    timezone_abbreviation: String,
    elevation: f64,
    current_units: CurrentUnits,
    current: Current,
    hourly_units: HourlyUnits,
    hourly: Hourly
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec!["https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&current=temperature_2m,wind_speed_10m&hourly=temperature_2m"];
    let start = Instant::now();
    
    let tasks: Vec<_> = urls
        .into_iter()
        .map(|url| tokio::spawn(fetch_http(url.to_string())))
        .collect();

    
    for task in tasks {
        match task.await? {
            Ok(info) => println!("Fetched successfully:\n{:#?}", info),
            Err(e) => eprintln!("Error: {}", e),
        }
    }


    
    println!("Total time: {:?}", start.elapsed());
    Ok(())
}

async fn fetch_http(url: String) -> Result<OpenMeteo, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let x: String = response.text().await?;
        Ok(serde_json::from_str(&x)?)
    } else {
        let code = response.status().as_u16();
        Err(format!("not successful {code}").into())
    }
}
