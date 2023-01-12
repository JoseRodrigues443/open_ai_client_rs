use std::error::Error;

use async_trait::async_trait;
use mockall::automock;
use reqwest::{Client, StatusCode};

use crate::{
    consts::{AUTHORIZATION_KEY, BASE_URL, BEARER, MODELS_ENDPOINT, ORG_KEY},
    data::ModelResponse,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

trait RequestBuilderUtils {
    fn add_org(self, org: String) -> Self;
}

#[async_trait]
#[automock]
pub trait ClientBase {
    fn new(token: String, org: String) -> Self;
    async fn models(&self) -> Result<ModelResponse>;
}
pub struct OpenAI {
    http_client: Client,
    token: String,
    pub org: String,
}

#[async_trait]
impl ClientBase for OpenAI {
    fn new(token: String, org: String) -> OpenAI {
        OpenAI {
            http_client: Client::new(),
            token,
            org,
        }
    }

    async fn models(&self) -> Result<ModelResponse> {
        let url = format!("{}/{}", BASE_URL, MODELS_ENDPOINT);
        let bearer = format!("{}/{}", BEARER, self.token);

        let response = self
            .http_client
            .get(url)
            .header(AUTHORIZATION_KEY, bearer)
            .header(ORG_KEY, &self.org)
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                match response.json::<ModelResponse>().await {
                    Ok(parsed) => return Ok(parsed),
                    Err(_) => {
                        return Err("Hm, the response didn't match the shape we expected.".into())
                    }
                };
            }

            reqwest::StatusCode::UNAUTHORIZED => {
                return Err("Need to grab a new token".into());
            }
            _ => {
                return Err("Uh oh! Something unexpected happened".into());
            }
        };
    }
}
