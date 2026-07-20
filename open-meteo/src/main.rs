use reqwest;
use tokio::task::JoinHandle;
use std::time::Instant;
use serde::{Deserialize, Serialize};

/*
  {
   "args": {},
   "headers": { ... },
   "origin": "...",
   "url": "..."
 }
 */

#[derive(Debug, Serialize, Deserialize)]
struct HttpBin {  // this struct is not working, for display purpose only
    origin: String,
    url: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let url = "https://httpbin.org/status/404";
    let url = "https://httpbin.org/get";
    let start = Instant::now();
    
    // Create tasks for each URL
    // let tasks: Vec<_> = urls
    //     .into_iter()
    //     .map(|url| tokio::spawn(fetch_repo_info(url.to_string())))
    //     .collect();

    let tasks: Vec<JoinHandle<Result<String, Box<dyn std::error::Error + Send + Sync>>>> = vec![tokio::spawn(fetch_http(url.to_string()))];
    
    // Wait for all tasks to complete
    for task in tasks {
        match task.await? {
            Ok(info) => println!("Fetched: {}", info),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    println!("Total time: {:?}", start.elapsed());
    Ok(())
}

async fn fetch_http(url: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let response = reqwest::get(&url).await?;
    if response.status().is_success() {
        let x: String = response.text().await?;
        let parse: HttpBin = serde_json::from_str(&x)?;
        println!("{:?}", parse);
        Ok("successful!".to_string())
    } else {
        let code = response.status().as_u16();
        Err(format!("not successful {code}").into())
    }
}
