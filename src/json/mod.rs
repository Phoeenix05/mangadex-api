//! This module provides types for deserializing API responses from
//! MangaDex API.
//!
//! ```
//! use mangadex_api::prelude::Manga;
//!
//! #[tokio::main]
//! async fn main() {
//!     let json: Manga = reqwest::Client::new()
//!         .get("https://api.mangadex.org/manga/c288b108-5162-4065-aa3a-5857ec38c8d9")
//!         .send()
//!         .await
//!         .unwrap()
//!         .json()
//!         .await
//!         .unwrap();
//! }
//! ```

pub mod athome;
pub use athome::AtHomeServer;

pub mod author;
pub use author::{Author, AuthorList};

pub mod chapter;
pub use chapter::{Chapter, ChapterList};

pub mod cover;
pub use cover::{Cover, CoverList};

pub mod manga;
pub use manga::{Manga, MangaAggregate, MangaFeed, MangaList};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

/// This struct represents the relationships between mangas, covers, authors,
/// chapters, etc.
///
/// The `attributes` field of the relationship is `serde_json::Value` as the attribute
/// value can be so many things it would be difficult to deserialize it automatically into
/// a struct. The data can still be queried using `serde_json` crate.
///
/// For now please check [MangaDex API docs](https://api.mangadex.org/docs/) for more info.
/// Though I think the attributes field can be deserialized in to `mangadex_api::prelude::Manga`
/// if the type of the relationship is `manga` same for `cover_art` and so on.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    #[serde(rename = "id")]
    pub uuid: Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub related: Option<String>,
    pub attributes: Option<serde_json::Value>,
}
