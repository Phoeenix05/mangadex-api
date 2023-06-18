// #![cfg(feature = "wrapper")]

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

#[macro_export]
macro_rules! uuid_or_err {
    ($opt:expr) => {
        if let Some(uuid) = $opt {
            uuid
        } else {
            return Err(ApiError::default());
        }
    };
}

lazy_static::lazy_static! {
    pub static ref CLIENT: ClientWithMiddleware = {
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::default(),
                options: None,
            }))
            .build();
        client
    };
}
