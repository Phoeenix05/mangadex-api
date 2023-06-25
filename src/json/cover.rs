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
#[serde(rename_all = "camelCase")]
pub struct CoverAttributes {
    volume: Option<String>,
    file_name: String,
    description: String,
    locale: String,
    version: u64,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cover {
    result: String,
    response: String,
    data: Data<CoverAttributes>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoverList {
    result: String,
    response: String,
    data: Vec<Data<CoverAttributes>>,
    limit: u64,
    offset: u64,
    total: u64,
}

////////////////////////////////////////////////////////////////
/// Implementations
////////////////////////////////////////////////////////////////

impl Cover {
    pub fn result(&self) -> &String {
        &self.result
    }

    pub fn response(&self) -> &String {
        &self.response
    }

    pub fn data(&self) -> &Data<CoverAttributes> {
        &self.data
    }
}

impl Client<Cover> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<Cover, ClientError> {
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
            .get(construct_url(format!("/cover/{uuid}"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl CoverList {
    pub fn result(&self) -> &String {
        &self.result
    }

    pub fn response(&self) -> &String {
        &self.response
    }

    pub fn data(&self) -> &Vec<Data<CoverAttributes>> {
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

impl Client<CoverList> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<CoverList, ClientError> {
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
            .get(construct_url(format!("/cover"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl CoverAttributes {
    pub fn volume(&self) -> &Option<String> {
        &self.volume
    }

    pub fn file_name(&self) -> &String {
        &self.file_name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn locale(&self) -> &String {
        &self.locale
    }

    pub fn version(&self) -> &u64 {
        &self.version
    }

    pub fn created_at(&self) -> &Option<String> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &Option<String> {
        &self.updated_at
    }
}
