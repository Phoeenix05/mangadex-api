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

pub use manga::List;

use serde::{Deserialize, Serialize};

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

    #[allow(non_snake_case)]
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

        assert!(res.is_ok())
    }
}
