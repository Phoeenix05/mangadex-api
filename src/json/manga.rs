use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::client::util::construct_url;
use crate::prelude::*;
use crate::util::*;
use crate::{unwrap_api_results, uuid_or_err};

////////////////////////////////////////////////////////////////
/// Structs
////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manga {
    result: String,
    response: String,
    data: Data<MangaAttributes>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MangaAttributes {
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
pub struct MangaFeedAttributes {
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaList {
    result: String,
    response: String,
    data: Vec<Data<MangaAttributes>>,
    limit: u64,
    offset: u64,
    total: u64,
}

////////////////////////////////////////////////////////////////
/// Implementations
////////////////////////////////////////////////////////////////

impl Manga {
    pub fn result(&self) -> &String {
        &self.result
    }

    pub fn response(&self) -> &String {
        &self.response
    }

    pub fn data(&self) -> &Data<MangaAttributes> {
        &self.data
    }
}

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
            .get(construct_url(format!("/manga/{uuid}"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl MangaFeed {
    pub fn result(&self) -> &String {
        &self.result
    }

    pub fn response(&self) -> &String {
        &self.response
    }

    pub fn data(&self) -> &Vec<Data<MangaFeedAttributes>> {
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
            .get(construct_url(format!("/manga/{uuid}/feed"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl MangaList {
    pub fn result(&self) -> &String {
        &self.result
    }

    pub fn response(&self) -> &String {
        &self.response
    }

    pub fn data(&self) -> &Vec<Data<MangaAttributes>> {
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

impl Client<MangaList> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<MangaList, ClientError> {
        let res = CLIENT
            .get(construct_url(format!("/manga/"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl MangaAttributes {
    pub fn title(&self) -> &serde_json::Value {
        &self.title
    }

    pub fn alt_titles(&self) -> &Vec<serde_json::Value> {
        &self.alt_titles
    }

    pub fn description(&self) -> &serde_json::Value {
        &self.description
    }

    pub fn is_locked(&self) -> &bool {
        &self.is_locked
    }

    pub fn links(&self) -> &serde_json::Value {
        &self.links
    }

    pub fn original_language(&self) -> &String {
        &self.original_language
    }

    pub fn last_volume(&self) -> &Option<String> {
        &self.last_volume
    }

    pub fn last_chapter(&self) -> &Option<String> {
        &self.last_chapter
    }

    pub fn publication_demographic(&self) -> &Option<String> {
        &self.publication_demographic
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn year(&self) -> &Option<u64> {
        &self.year
    }

    pub fn content_rating(&self) -> &String {
        &self.content_rating
    }

    pub fn chapter_numbers_reset_on_new_volume(&self) -> &bool {
        &self.chapter_numbers_reset_on_new_volume
    }

    pub fn available_translated_languages(&self) -> &Vec<serde_json::Value> {
        &self.available_translated_languages
    }

    pub fn latest_uploaded_chapter(&self) -> &Uuid {
        &self.latest_uploaded_chapter
    }

    pub fn tags(&self) -> &Vec<serde_json::Value> {
        &self.tags
    }

    pub fn state(&self) -> &String {
        &self.state
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
}

impl MangaFeedAttributes {
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
