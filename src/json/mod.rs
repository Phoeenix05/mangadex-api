use serde::Deserialize;
use uuid::Uuid;

pub mod error;

pub mod chapter;
pub mod cover;
pub mod manga;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", bound = "T: serde::de::DeserializeOwned")]
pub struct MangaDexAPIResponse<T> {
    pub result: String,
    pub data: T,
    pub limit: Option<u64>,
    pub offsset: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", bound = "T: serde::de::DeserializeOwned")]
pub struct Data<T> {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: T,
    pub relationships: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Relationship {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub related: String,
    pub attributes: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: serde_json::Value,
    pub description: serde_json::Value,
    pub group: String,
    pub version: u64,
}
