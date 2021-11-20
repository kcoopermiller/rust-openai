// type Result<T> = std::result::Result<T, Error>;

pub mod openai {
    // sending http requests
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

    // (de)serialize json
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize)]
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

    #[derive(Debug, Clone)]
    pub struct Client {
        _api_key: String,
        _base_url: String,
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
        max_tokens: i32,
    }

    impl Client {
        pub fn new(api_key: String) -> Client {
            Client {
                _api_key: api_key,
                _base_url: String::from("https://api.openai.com/v1"),
            }
        }

        pub async fn get_engines(&self) -> Result<(), Box<dyn std::error::Error>> {
            let client = reqwest::Client::new();
            let mut headers = HeaderMap::new();
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", &self._api_key)
                    .parse::<HeaderValue>()
                    .unwrap(),
            );
            let resp = client
                .get(format!("{}/engines", &self._base_url))
                .headers(headers)
                .send()
                .await?;

            // deserializing into Engines
            let json = resp.json::<Engines>().await?;
            println!("{:#?}", json);
            Ok(())
        }

        pub async fn engine_info() {}

        pub async fn completion(
            &self,
            prompt_: &str,
            max_tokens_: i32,
            engine_id: &str,
        ) -> Result<(), Box<dyn std::error::Error>> {
            let mut headers = HeaderMap::new();
            headers.insert(
                CONTENT_TYPE,
                "application/json".parse::<HeaderValue>().unwrap(),
            );
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", &self._api_key)
                    .parse::<HeaderValue>()
                    .unwrap(),
            );

            let params = Body {
                prompt: prompt_.to_string(),
                max_tokens: max_tokens_,
            };

            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}/engines/{}/completions",
                    &self._base_url,
                    engine_id.to_string()
                ))
                .headers(headers)
                .json(&params)
                .send()
                .await?;

            let json = resp.json::<PostResponse>().await?;

            println!("{:#?}", json);
            Ok(())
        }

        pub async fn search() {}

        pub async fn finetune() {}
    }
}
