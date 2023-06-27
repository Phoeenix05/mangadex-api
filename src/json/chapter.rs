//! This module provides types for every chapter endpoint available on MangaDex API.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;
use crate::util::client::construct_url;
use crate::{client, unwrap_api_results, uuid_or_err};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chapter {
    pub result: String,
    pub response: String,
    pub data: Data<ChapterAttributes>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChapterList {
    pub result: String,
    pub response: String,
    pub data: Vec<Data<ChapterAttributes>>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChapterAttributes {
    pub title: Option<String>,
    pub volume: Option<String>,
    pub chapter: String,
    pub pages: u64,
    pub translated_language: String,
    pub uploader: Option<Uuid>,
    pub external_url: Option<String>,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
    pub publish_at: String,
    pub readable_at: String,
}

impl Client<Chapter> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<Chapter, ClientError> {
        let client = client!(CacheMode::Default);

        let uuid = uuid_or_err!(self.get_uuid()).unwrap();
        let res = client
            .get(construct_url(format!("/chapter/{uuid}"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl Client<ChapterList> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<ChapterList, ClientError> {
        let client = client!(CacheMode::NoCache);
        let res = client
            .get(construct_url("/chapter".into(), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}
