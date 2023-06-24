pub mod athome;
pub use athome::*;

pub mod author;
pub use author::*;

pub mod chapter;
pub use chapter::*;

pub mod cover;
pub use cover::*;

pub mod manga;
pub use manga::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    result: String,
    errors: Vec<serde_json::Value>,
}

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
