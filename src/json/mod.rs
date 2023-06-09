mod at_home;
// mod auth;
mod author;
// mod captcha;
mod chapter;
mod cover;
// mod customlist;
// mod feed;
// mod follows;
// mod forums;
// mod infrastructure;
// mod legacy;
mod manga;
// mod rating;
// mod readmarker;
// mod report;
// mod scanlationgroup;
// mod settings;
// mod upload;
// mod user;

pub use at_home::*;
pub use author::*;
pub use chapter::*;
pub use cover::*;
pub use manga::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "T: serde::de::DeserializeOwned + serde::ser::Serialize")]
pub struct Response<T> {
    pub result: String,
    pub response: String,
    pub data: T,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound = "T: serde::de::DeserializeOwned + serde::ser::Serialize")]
pub struct Data<T> {
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: T,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub related: Option<String>,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Err40X {
    pub result: String,
    pub errors: Vec<Error>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub id: String,
    pub status: u64,
    pub title: String,
    pub detail: String,
}

#[cfg(test)]
mod tests {
    use super::*;

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
