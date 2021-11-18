use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Deserialize)]
struct PostResponse {
    id: String,
    object: String,
    created: u32,
    model: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    text: String,
    index: u32,
    logprobs: Option<u32>,
    finish_reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Body {
    prompt: String,
    max_tokens: u32,
}
// TODO: create lib.rs, create client struct, study unfamiliar terminology

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load api key from .env into std::env
    dotenv().ok();
    let api_key = env::var("API_KEY")?;

    // map of http request headers
    let mut headers = HeaderMap::new();

    let engine_id = "davinci";

    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", api_key)
            .parse::<HeaderValue>()
            .unwrap(),
    );

    // get request
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://api.openai.com/v1/engines/{}", &engine_id))
        .headers(headers)
        .send()
        .await?;

    // deserializing into Engine
    let json = resp.json::<Engine>().await?;
    println!("GET RESPONSE\n");
    println!("{:#?}", json);

    // post request
    let mut post_headers = HeaderMap::new();

    post_headers.insert(
        CONTENT_TYPE,
        "application/json".parse::<HeaderValue>().unwrap(),
    );

    post_headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", api_key)
            .parse::<HeaderValue>()
            .unwrap(),
    );

    let params = Body {
        prompt: "once upon a time".to_string(),
        max_tokens: 5,
    };
    let post_client = reqwest::Client::new();
    let post_resp = post_client
        .post(format!(
            "https://api.openai.com/v1/engines/{}/completions",
            &engine_id
        ))
        .headers(post_headers)
        .json(&params)
        .send()
        .await?;

    println!("POST RESPONSE\n");
    println!("Received response status: {:?}", post_resp.status());
    let post_json = post_resp.json::<PostResponse>().await?;

    println!("{:#?}", post_json);

    Ok(())
}
