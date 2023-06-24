use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Data;
use crate::prelude::{Client, ClientError, ClientUtil};
use crate::{unwrap_api_results, util::CLIENT, uuid_or_err};

////////////////////////////////////////////////////////////////
/// Manga
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manga {
    result: String,
    response: String,
    data: Data<MangaAttributes>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MangaAttributes {
    title: serde_json::Value,
    alt_titles: Vec<serde_json::Value>,
    description: serde_json::Value,
    is_locked: bool,
    links: serde_json::Value,
    original_language: String,
    last_volume: Option<String>,
    last_chapter: Option<String>,
    publication_demographic: Option<String>,
    status: String,
    year: Option<u64>,
    content_rating: String,
    chapter_numbers_reset_on_new_volume: bool,
    available_translated_languages: Vec<serde_json::Value>, // Vec<String>
    latest_uploaded_chapter: uuid::Uuid,
    tags: Vec<serde_json::Value>,
    state: String,
    version: u64,
    created_at: String,
    updated_at: String,
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct MangaStatistics;

impl Client<Manga> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<Manga, ClientError> {
        let uuid = uuid_or_err!(self.get_uuid()).unwrap();
        let res = CLIENT
            .get(ClientUtil::construct_url(format!("/manga/{uuid}"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }

    // pub async fn get_statistics(self) -> Result<Manga, ClientError> {
    //     let err = ClientError {
    //         msg: "Not implemented".to_string(),
    //         api_msg: None,
    //     };
    //     Err(err)
    // }
}

////////////////////////////////////////////////////////////////
/// Manga feed
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaFeed {
    result: String,
    response: String,
    data: Vec<Data<MangaFeedAttributes>>,
    limit: u64,
    offset: u64,
    total: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct MangaFeedAttributes {
    title: Option<String>,
    volume: Option<String>,
    chapter: String,
    pages: u64,
    translated_language: String,
    uploader: Option<uuid::Uuid>,
    external_url: Option<String>,
    version: u64,
    created_at: String,
    updated_at: String,
    publish_at: String,
    readable_at: String,
}

impl Client<MangaFeed> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<MangaFeed, ClientError> {
        let uuid = uuid_or_err!(self.get_uuid()).unwrap();
        let res = CLIENT
            .get(ClientUtil::construct_url(
                format!("/manga/{uuid}/feed"),
                None,
            ))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

////////////////////////////////////////////////////////////////
/// Manga list
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaList {
    result: String,
    response: String,
    data: Vec<Data<MangaAttributes>>,
    limit: u64,
    offset: u64,
    total: u64,
}

impl Client<MangaList> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<MangaList, ClientError> {
        let res = CLIENT
            .get(ClientUtil::construct_url(format!("/manga/"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct MangaListStatistics;

// impl Client<MangaListStatistics> {
//     pub fn new() -> Self {
//         todo!()
//     }

//     pub async fn get_statistics(self) -> Result<MangaListStatistics, ClientError> {
//         todo!()
//     }
// }
