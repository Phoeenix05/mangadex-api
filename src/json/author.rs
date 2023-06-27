//! This module provides types for every author endpoint available on MangaDex API.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;
use crate::util::client::construct_url;
use crate::{client, unwrap_api_results, uuid_or_err};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Author {
    pub result: String,
    pub response: String,
    pub data: Data<AuthorAttributes>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthorList {
    pub result: String,
    pub response: String,
    pub data: Vec<Data<AuthorAttributes>>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorAttributes {
    pub name: String,
    pub image_url: Option<String>,
    pub biography: serde_json::Value,
    pub twitter: Option<String>,
    pub pixiv: Option<String>,
    pub melon_book: Option<String>,
    pub fan_box: Option<String>,
    pub booth: Option<String>,
    pub nico_video: Option<String>,
    pub skeb: Option<String>,
    pub fantia: Option<String>,
    pub tumblr: Option<String>,
    pub youtube: Option<String>,
    pub weibo: Option<String>,
    pub naver: Option<String>,
    pub website: Option<String>,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
}

impl Client<Author> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<Author, ClientError> {
        let client = client!(CacheMode::Default);

        let uuid = uuid_or_err!(self.get_uuid()).unwrap();
        let res = client
            .get(construct_url(format!("/author/{uuid}"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl Client<AuthorList> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<AuthorList, ClientError> {
        let client = client!(CacheMode::Default);
        let res = client
            .get(construct_url(format!("/author"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}
