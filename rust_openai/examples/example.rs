use dotenv::dotenv;
use rust_openai::openai;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load api key from .env into std::env
    dotenv().ok();
    let api_key: String = env::var("API_KEY")?;

    let client = openai::Client::new(api_key);

    // client.get_engines().await?;
    // client.engine("davinci").await?;

    // client.finetune("light_novels.jsonl", "curie").await?;

    // client.completion(
    //     "", 
    //     42, 
    //     "curie:ft-user-wlep6pfel6wm7prcxzlzijao-2021-11-26-03-20-31"
    // ).await?;

    client.completion(
        "hello, ", 
        64, 
        "davinci"
    ).await?;

    Ok(())
}
