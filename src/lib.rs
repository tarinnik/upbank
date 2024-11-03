pub mod accounts;
pub mod attachments;
pub mod categories;
pub mod common;
mod currencies;
mod error;
pub mod transactions;

pub use crate::error::Error;

use crate::accounts::AccountList;
use reqwest::{Client, Response, Url};

const USER_AGENT: &str = "up_web";

const BASE_URL: &str = "https://api.up.com.au/api/v1/";
const PING_URL: &str = "util/ping";
const ACCOUNTS_URL: &str = "accounts";

#[derive(Clone)]
pub struct UpApi {
    client: Client,
    token: String,
}

impl UpApi {
    pub fn new(token: String) -> Result<Self, Error> {
        let client = Client::builder().user_agent(USER_AGENT).build()?;
        Ok(Self { client, token })
    }

    pub async fn ping(&self) -> Result<String, Error> {
        let response = self.request(PING_URL).await?;
        println!("Ping status: {}", response.status());

        response.text().await.map_err(|e| -> Error { e.into() })
    }

    pub async fn accounts(&self) -> Result<AccountList, Error> {
        let response = self.request(ACCOUNTS_URL).await?;
        response
            .json::<AccountList>()
            .await
            .map_err(|e| -> Error { e.into() })
    }

    async fn request(&self, path: &str) -> Result<Response, Error> {
        let url = Url::parse(BASE_URL).unwrap().join(path).unwrap();
        self.client
            .get(url)
            .bearer_auth(&self.token)
            .send()
            .await
            .map_err(|e| -> Error { e.into() })
    }
}
