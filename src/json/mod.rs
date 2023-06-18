use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod error;

pub mod author;
pub mod chapter;
pub mod cover;
pub mod manga;
pub mod statistics;
// pub mod athome;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", bound = "T: serde::de::DeserializeOwned")]
pub struct MangaDexAPIResponse<T> {
    pub result: String,
    pub data: T,
    pub limit: Option<u64>,
    pub offsset: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(
    rename_all = "camelCase",
    bound = "T: serde::de::DeserializeOwned + serde::Serialize"
)]
pub struct Data<T> {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: T,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub related: Option<String>,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: serde_json::Value,
    pub description: serde_json::Value,
    pub group: String,
    pub version: u64,
}
