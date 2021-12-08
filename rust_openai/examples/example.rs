use dotenv::dotenv;
use rust_openai::openai;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load api key from .env into std::env
    dotenv().ok();
    let api_key: String = env::var("API_KEY")?;

    let client = openai::Client::new(api_key);

    // lists all of the available models
    client.get_engines().await?;

    // shows details for a single model
    client.engine("davinci").await?;

    // client.finetune("[training file]", "curie").await?;

    client.completion(
        "hello, ", 
        4, 
        "davinci"
    ).await?;

    Ok(())
}
