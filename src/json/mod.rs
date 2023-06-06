mod at_home;
mod auth;
mod author;
mod captcha;
mod chapter;
mod cover;
mod customlist;
mod feed;
mod follows;
mod forums;
mod infrastructure;
mod legacy;
mod manga;
mod rating;
mod readmarker;
mod report;
mod scanlationgroup;
mod settings;
mod upload;
mod user;

pub use at_home::AtHomeServer;
pub use chapter::{Chapter, ChapterList};
pub use manga::{MangaFeed, MangaList};

use serde::{Deserialize, Serialize};

pub enum Result<T> {
    Ok(T),
    Err(Error),
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
