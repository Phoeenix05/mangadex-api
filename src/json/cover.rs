//! This module provides types for every cover endpoint available on MangaDex API.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;
use crate::util::client::construct_url;
use crate::{client, unwrap_api_results, uuid_or_err};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cover {
    pub result: String,
    pub response: String,
    pub data: Data<CoverAttributes>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoverList {
    pub result: String,
    pub response: String,
    pub data: Vec<Data<CoverAttributes>>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CoverAttributes {
    pub volume: Option<String>,
    pub file_name: String,
    pub description: String,
    pub locale: String,
    pub version: u64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Client<Cover> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<Cover, ClientError> {
        let client = client!(CacheMode::Default);
        let uuid = uuid_or_err!(self.get_uuid()).unwrap();
        let res = client
            .get(construct_url(format!("/cover/{uuid}"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
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
        let client = client!(CacheMode::NoCache);
        let res = client
            .get(construct_url(format!("/cover"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}
