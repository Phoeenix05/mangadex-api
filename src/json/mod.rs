pub mod at_home;
pub mod auth;
pub mod author;
pub mod captcha;
pub mod chapter;
pub mod cover;
pub mod customlist;
pub mod feed;
pub mod follows;
pub mod forums;
pub mod infrastructure;
pub mod legacy;
pub mod manga;
pub mod rating;
pub mod readmarker;
pub mod report;
pub mod scanlationgroup;
pub mod settings;
pub mod upload;
pub mod user;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub result: String,
    pub response: String,
    pub data: Vec<T>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub related: Option<String>,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Err40X {
    pub result: String,
    pub errors: Vec<Error>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub id: String,
    pub status: u64,
    pub title: String,
    pub detail: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use reqwest::{Response, Result};

    pub async fn fetch(url: &str) -> Result<Response> {
        let client: reqwest::Client = reqwest::Client::new();
        let res = client.get(url).send().await;
        res
    }

    #[tokio::test]
    async fn fetch_errors() {
        let client = reqwest::Client::new();
        let res = client
            .get("https://api.mangadex.org/fail")
            .send()
            .await
            .unwrap()
            .json::<Err40X>()
            .await;

        assert!(res.is_ok(), "{res:#?}")
    }
}
