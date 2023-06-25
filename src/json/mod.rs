pub mod athome;
pub use athome::AtHomeServer;

pub mod author;
pub use author::{Author, AuthorList};

pub mod chapter;
pub use chapter::{Chapter, ChapterList};

pub mod cover;
pub use cover::{Cover, CoverList};

pub mod manga;
pub use manga::{Manga, MangaFeed, MangaList};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

////////////////////////////////////////////////////////////////
/// Structs
////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub result: String,
    pub errors: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: serde::de::DeserializeOwned + serde::ser::Serialize")]
pub struct Data<T> {
    #[serde(rename = "id")]
    pub uuid: Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: T,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    #[serde(rename = "id")]
    pub uuid: Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub related: Option<String>,
    pub attributes: Option<serde_json::Value>,
}
