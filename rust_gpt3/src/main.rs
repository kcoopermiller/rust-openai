use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
struct Engines {
    data: Vec<Engine>,
}

#[derive(Debug, Deserialize)]
struct Engine {
    id: String,
    object: String,
    owner: String,
    ready: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("API_KEY")?;

    let mut headers = HeaderMap::new();

    let engine_id = "davinci";

    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", api_key)
            .parse::<HeaderValue>()
            .unwrap(),
    );
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://api.openai.com/v1/engines/{}", engine_id))
        .headers(headers)
        .send()
        .await?;

    let json = resp.json::<Engine>().await?;
    println!("{:#?}", json);
    Ok(())
}
