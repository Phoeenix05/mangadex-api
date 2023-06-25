use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest_middleware::ClientBuilder;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;
use crate::util::client::construct_url;
use crate::{unwrap_api_results, uuid_or_err};

////////////////////////////////////////////////////////////////
/// Structs
////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chapter {
    result: String,
    response: String,
    data: Data<ChapterAttributes>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterAttributes {
    title: Option<String>,
    volume: Option<String>,
    chapter: String,
    pages: u64,
    translated_language: String,
    uploader: Option<Uuid>,
    external_url: Option<String>,
    version: u64,
    created_at: String,
    updated_at: String,
    publish_at: String,
    readable_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChapterList {
    result: String,
    response: String,
    data: Vec<Data<ChapterAttributes>>,
    limit: u64,
    offset: u64,
    total: u64,
}

////////////////////////////////////////////////////////////////
/// Implementations
////////////////////////////////////////////////////////////////

impl Chapter {
    pub fn result(&self) -> &String {
        &self.result
    }

    pub fn response(&self) -> &String {
        &self.response
    }

    pub fn data(&self) -> &Data<ChapterAttributes> {
        &self.data
    }
}

impl Client<Chapter> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<Chapter, ClientError> {
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
            .get(construct_url(format!("/chapter/{uuid}"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl ChapterList {
    pub fn result(&self) -> &String {
        &self.result
    }

    pub fn response(&self) -> &String {
        &self.response
    }

    pub fn data(&self) -> &Vec<Data<ChapterAttributes>> {
        &self.data
    }

    pub fn limit(&self) -> &u64 {
        &self.limit
    }

    pub fn offset(&self) -> &u64 {
        &self.offset
    }

    pub fn total(&self) -> &u64 {
        &self.total
    }
}

impl Client<ChapterList> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            // url: todo!(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<ChapterList, ClientError> {
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
            .get(construct_url("/chapter".into(), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl ChapterAttributes {
    pub fn title(&self) -> &Option<String> {
        &self.title
    }

    pub fn volume(&self) -> &Option<String> {
        &self.volume
    }

    pub fn chapter(&self) -> &String {
        &self.chapter
    }

    pub fn pages(&self) -> &u64 {
        &self.pages
    }

    pub fn translated_language(&self) -> &String {
        &self.translated_language
    }

    pub fn uploader(&self) -> &Option<Uuid> {
        &self.uploader
    }

    pub fn external_url(&self) -> &Option<String> {
        &self.external_url
    }

    pub fn version(&self) -> &u64 {
        &self.version
    }

    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    pub fn updated_at(&self) -> &String {
        &self.updated_at
    }

    pub fn publish_at(&self) -> &String {
        &self.publish_at
    }

    pub fn readable_at(&self) -> &String {
        &self.readable_at
    }
}
