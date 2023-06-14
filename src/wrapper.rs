use std::sync::Arc;

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use uuid::Uuid;

use crate::{json::error::ErrorResponse, types::get::*};

lazy_static::lazy_static! {
    static ref CLIENT: Arc<ClientWithMiddleware> = {
        let client = ClientBuilder::new(Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager::default(),
                options: None
            }))
            .build();
        Arc::new(client)
    };
}

#[cfg(any(feature = "wrapper", feature = "dl"))]
#[async_trait::async_trait]
pub trait Endpoint
where
    Self: Sized,
{
    async fn get() -> Result<Self, ErrorResponse> {
        unimplemented!()
    }

    async fn get_uuid(_uuid: Uuid) -> Result<Self, ErrorResponse> {
        unimplemented!()
    }
}

#[cfg(any(feature = "wrapper", feature = "dl"))]
#[async_trait::async_trait]
impl Endpoint for AtHomeServer {
    async fn get() -> Result<Self, ErrorResponse> {
        todo!()
    }
}

#[cfg(any(feature = "wrapper"))]
#[async_trait::async_trait]
impl Endpoint for Manga {
    async fn get_uuid(uuid: Uuid) -> Result<Self, ErrorResponse> {
        let url = format!("https://api.mangadex.org/manga/{}", uuid);
        let res = CLIENT.get(url).send().await.unwrap();

        if res.status().is_success() {
            let json = res.json().await.unwrap();
            Ok(json)
        } else {
            let json = res.json().await.unwrap();
            Err(json)
        }
    }
}

#[cfg(any(feature = "wrapper", feature = "dl"))]
#[async_trait::async_trait]
impl Endpoint for MangaFeed {
    async fn get_uuid(uuid: Uuid) -> Result<Self, ErrorResponse> {
        let url = format!("https://api.mangadex.org/manga/{}/feed", uuid);
        let res = CLIENT.get(url).send().await.unwrap();

        if res.status().is_success() {
            let json = res.json().await.unwrap();
            Ok(json)
        } else {
            let json = res.json().await.unwrap();
            Err(json)
        }
    }
}

#[cfg(any(feature = "wrapper"))]
#[async_trait::async_trait]
impl Endpoint for MangaList {
    async fn get() -> Result<Self, ErrorResponse> {
        let res = CLIENT
            .get("https://api.mangadex.org/manga")
            .send()
            .await
            .unwrap();

        if res.status().is_success() {
            let json = res.json().await.unwrap();
            Ok(json)
        } else {
            let json = res.json().await.unwrap();
            Err(json)
        }
    }
}
