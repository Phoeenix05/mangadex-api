//! This module provides types for every manga endpoint available on MangaDex API.

use std::collections::HashMap;

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest_middleware::ClientBuilder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;
use crate::util::client::construct_url;
use crate::{unwrap_api_results, uuid_or_err};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manga {
    pub result: String,
    pub response: String,
    pub data: Data<MangaAttributes>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaAttributes {
    pub title: serde_json::Value,
    pub alt_titles: Vec<serde_json::Value>,
    pub description: serde_json::Value,
    pub is_locked: bool,
    pub links: serde_json::Value,
    pub original_language: String,
    pub last_volume: Option<String>,
    pub last_chapter: Option<String>,
    pub publication_demographic: Option<String>,
    pub status: String,
    pub year: Option<u64>,
    pub content_rating: String,
    pub chapter_numbers_reset_on_new_volume: bool,
    pub available_translated_languages: Vec<serde_json::Value>,
    pub latest_uploaded_chapter: uuid::Uuid,
    pub tags: Vec<serde_json::Value>,
    pub state: String,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaFeed {
    pub result: String,
    pub response: String,
    pub data: Vec<Data<MangaFeedAttributes>>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaFeedAttributes {
    pub title: Option<String>,
    pub volume: Option<String>,
    pub chapter: String,
    pub pages: u64,
    pub translated_language: String,
    pub uploader: Option<uuid::Uuid>,
    pub external_url: Option<String>,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
    pub publish_at: String,
    pub readable_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaList {
    pub result: String,
    pub response: String,
    pub data: Vec<Data<MangaAttributes>>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

/// This is used fetch a list of volumes and chapters of a manga.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaAggregate {
    pub result: String,
    pub volumes: HashMap<String, Volume>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Volume {
    pub volume: String,
    pub count: u64,
    pub chapters: HashMap<String, Chapter>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chapter {
    pub chapter: String,
    pub id: Uuid,
    pub others: Vec<Uuid>,
    pub count: u64,
}

impl Client<Manga> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<Manga, ClientError> {
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager {
                    path: if let Some(mut path) = dirs::cache_dir() {
                        path.push("mangadex_api-cacache");
                        path
                    } else {
                        std::path::PathBuf::from("./mangadex_api-cacache")
                    },
                },
                options: None,
            }))
            .build();
        let uuid = uuid_or_err!(self.get_uuid()).unwrap();
        let res = client
            .get(construct_url(format!("/manga/{uuid}"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl Client<MangaFeed> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<MangaFeed, ClientError> {
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Reload,
                manager: CACacheManager {
                    path: if let Some(mut path) = dirs::cache_dir() {
                        path.push("mangadex_api-cacache");
                        path
                    } else {
                        std::path::PathBuf::from("./mangadex_api-cacache")
                    },
                },
                options: None,
            }))
            .build();
        let uuid = uuid_or_err!(self.get_uuid()).unwrap();
        let res = client
            .get(construct_url(format!("/manga/{uuid}/feed"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl Client<MangaList> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<MangaList, ClientError> {
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager {
                    path: if let Some(mut path) = dirs::cache_dir() {
                        path.push("mangadex_api-cacache");
                        path
                    } else {
                        std::path::PathBuf::from("./mangadex_api-cacache")
                    },
                },
                options: None,
            }))
            .build();
        let res = client
            .get(construct_url(format!("/manga"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl Client<MangaAggregate> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<MangaAggregate, ClientError> {
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager {
                    path: if let Some(mut path) = dirs::cache_dir() {
                        path.push("mangadex_api-cacache");
                        path
                    } else {
                        std::path::PathBuf::from("./mangadex_api-cacache")
                    },
                },
                options: None,
            }))
            .build();
        let res = client
            .get(construct_url(
                format!("/manga/{}/aggregate", self.uuid.unwrap()),
                None,
            ))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}
