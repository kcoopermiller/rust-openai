// type Result<T> = std::result::Result<T, Error>;

pub mod openai {
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
    use serde::{Deserialize, Serialize};

    // all structs were created to store deserialized json data from the api
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
    struct Body { // make a default option
        prompt: String,
        max_tokens: i32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct FinetuneBody {
        _training_file: String,
        _base_model: String,
    }

    #[derive(Debug, Clone)]
    pub struct Client {
        _api_key: String,
        _base_url: String,
    }

    impl Client {
        // constructor that takes the user's openai api key
        pub fn new(api_key: String) -> Client {
            Client {
                _api_key: api_key,
                _base_url: String::from("https://api.openai.com/v1"),
            }
        }

        // get request that requests a list of all of the avaiable models
        pub async fn get_engines(&self) -> Result<(), Box<dyn std::error::Error>> {
            let client = reqwest::Client::new();
            let mut headers = HeaderMap::new();
            
            // creates REST api header using the user's api key
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", &self._api_key)
                    .parse::<HeaderValue>()
                    .unwrap(),
            );
            
            // get request
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

        // get request that requests the details of a specific model
        pub async fn engine(&self, engine_id: &str) -> Result<(), Box<dyn std::error::Error>> {
            let client = reqwest::Client::new();
            let mut headers = HeaderMap::new();
            
            // creates REST api header using the user's api key
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", &self._api_key)
                    .parse::<HeaderValue>()
                    .unwrap(),
            );
            
            // get request
            let resp = client
                .get(format!("{}/engines/{}", &self._base_url, &engine_id))
                .headers(headers)
                .send()
                .await?;

            // deserializing into Engine
            let json = resp.json::<Engine>().await?;
            println!("{:#?}", json);
            Ok(())
        }

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
           
            // creates REST api header using the user's api key
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", &self._api_key)
                    .parse::<HeaderValue>()
                    .unwrap(),
            );

            // creates REST api body
            let params = Body {
                prompt: prompt_.to_string(),
                max_tokens: max_tokens_,
            };

            
            // post request
            let client = reqwest::Client::new();
            let resp = client
                .post(format!(
                    "{}/engines/{}/completions",
                    &self._base_url, &engine_id
                ))
                .headers(headers)
                .json(&params)
                .send()
                .await?;

            let json = resp.json::<PostResponse>().await?;

            println!("{:#?}", json);
            Ok(())
        }

        pub async fn finetune(&self, training_file: &str,  base_model: &str) -> Result<(), Box<dyn std::error::Error>> {
            let mut headers = HeaderMap::new();
            headers.insert(
                CONTENT_TYPE,
                "application/json".parse::<HeaderValue>().unwrap(),
            );
            
            // rest api header
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", &self._api_key)
                    .parse::<HeaderValue>()
                    .unwrap(),
            );
            
            // rest api body designed for finetuning
            let params = FinetuneBody {
                _training_file: training_file.to_string(),
                _base_model: base_model.to_string()
            };

            // post request
            let client = reqwest::Client::new();
            let resp = client
                .post(" 
                https://api.openai.com/v1/fine-tunes")
                .headers(headers)
                .json(&params)
                .send()
                .await?;

            let json = resp.json::<PostResponse>().await?;

            println!("{:#?}", json);
            Ok(())
        }
    }
}
