mod at_home;
// mod auth;
mod author;
// mod captcha;
mod chapter;
mod cover;
// mod customlist;
// mod feed;
// mod follows;
// mod forums;
// mod infrastructure;
// mod legacy;
mod manga;
// mod rating;
// mod readmarker;
// mod report;
// mod scanlationgroup;
// mod settings;
// mod upload;
// mod user;

pub mod types;

use serde::{Deserialize, Serialize};

pub trait APIResponse {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: serde::de::DeserializeOwned + serde::ser::Serialize")]
pub struct Response<T> {
    pub result: String,
    pub response: String,
    pub data: T,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
}

impl<T> APIResponse for Response<T> {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: serde::de::DeserializeOwned + serde::ser::Serialize")]
pub struct Data<T> {
    #[serde(rename = "id")]
    pub uuid: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: T,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    #[serde(rename = "id")]
    pub uuid: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub related: Option<String>,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Err40X {
    pub result: String,
    pub errors: Vec<Error>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub id: String,
    pub status: u64,
    pub title: String,
    pub detail: String,
}
