// #![cfg(feature = "wrapper")]

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

#[macro_export]
macro_rules! uuid_or_err {
    ($opt:expr) => {{
        let uuid: &Option<uuid::Uuid> = $opt;
        if let Some(uuid) = uuid {
            uuid
        } else {
            let err = $crate::client::ClientError {
                msg: format!("This method requires a uuid"),
                api_msg: None,
            };
            return Err(err);
        }
    }};
}

#[macro_export]
macro_rules! unwrap_api_results {
    ($res:expr) => {{
        let res: reqwest::Response = $res;
        if res.status().is_success() {
            let json = res.json().await.unwrap();
            Some(json)
        } else {
            let json: $crate::json::ApiError = res.json().await.unwrap();
            let err = $crate::client::ClientError {
                msg: "Failed to fetch data from API".into(),
                api_msg: Some(json),
            };
            return Err(err);
        }
    }};
}

lazy_static::lazy_static! {
    pub static ref CLIENT: ClientWithMiddleware = ClientBuilder::new(reqwest::Client::new())
        .with(Cache(HttpCache {
            mode: CacheMode::Default,
            manager: CACacheManager::default(),
            options: None,
        }))
        .build();
}
