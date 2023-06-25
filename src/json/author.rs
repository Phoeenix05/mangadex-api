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
pub struct Author {
    result: String,
    response: String,
    data: Data<AuthorAttributes>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthorList {
    result: String,
    response: String,
    data: Vec<Data<AuthorAttributes>>,
    limit: u64,
    offset: u64,
    total: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorAttributes {
    name: String,
    image_url: Option<String>,
    biography: serde_json::Value,
    twitter: Option<String>,
    pixiv: Option<String>,
    melon_book: Option<String>,
    fan_box: Option<String>,
    booth: Option<String>,
    nico_video: Option<String>,
    skeb: Option<String>,
    fantia: Option<String>,
    tumblr: Option<String>,
    youtube: Option<String>,
    weibo: Option<String>,
    naver: Option<String>,
    website: Option<String>,
    version: u64,
    created_at: String,
    updated_at: String,
}

////////////////////////////////////////////////////////////////
/// Implementations
////////////////////////////////////////////////////////////////

impl Author {
    pub fn result(&self) -> &String {
        &self.result
    }

    pub fn response(&self) -> &String {
        &self.response
    }

    pub fn data(&self) -> &Data<AuthorAttributes> {
        &self.data
    }
}

impl Client<Author> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<Author, ClientError> {
        let uuid = uuid_or_err!(self.get_uuid()).unwrap();
        let res = CLIENT
            .get(construct_url(format!("/author/{uuid}"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl AuthorList {
    pub fn result(&self) -> &String {
        &self.result
    }

    pub fn response(&self) -> &String {
        &self.response
    }

    pub fn data(&self) -> &Vec<Data<AuthorAttributes>> {
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

impl Client<AuthorList> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<AuthorList, ClientError> {
        let res = CLIENT
            .get(construct_url(format!("/author"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl AuthorAttributes {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn image_url(&self) -> &Option<String> {
        &self.image_url
    }

    pub fn biography(&self) -> &serde_json::Value {
        &self.biography
    }

    pub fn twitter(&self) -> &Option<String> {
        &self.twitter
    }

    pub fn pixiv(&self) -> &Option<String> {
        &self.pixiv
    }

    pub fn melon_book(&self) -> &Option<String> {
        &self.melon_book
    }

    pub fn fan_box(&self) -> &Option<String> {
        &self.fan_box
    }

    pub fn booth(&self) -> &Option<String> {
        &self.booth
    }

    pub fn nico_video(&self) -> &Option<String> {
        &self.nico_video
    }

    pub fn skeb(&self) -> &Option<String> {
        &self.skeb
    }

    pub fn fantia(&self) -> &Option<String> {
        &self.fantia
    }

    pub fn tumblr(&self) -> &Option<String> {
        &self.tumblr
    }

    pub fn youtube(&self) -> &Option<String> {
        &self.youtube
    }

    pub fn weibo(&self) -> &Option<String> {
        &self.website
    }

    pub fn naver(&self) -> &Option<String> {
        &self.naver
    }

    pub fn website(&self) -> &Option<String> {
        &self.website
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
