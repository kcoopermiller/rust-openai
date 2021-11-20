use dotenv::dotenv;
use rust_openai::openai;
use std::env;

// TODO: create lib.rs, create client struct, study unfamiliar terminology

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load api key from .env into std::env
    dotenv().ok();
    let api_key: String = env::var("API_KEY")?;

    let client = openai::Client::new(api_key);

    client.get_engines().await?;

    client.completion("i'm in your", 5, "davinci").await?;

    Ok(())
}
