mod get;
pub use get::*;
use serde::{Deserialize, Serialize};

// struct Response<T> {
//     result: String,
//     response: String,
//     data: T,
//     limit: Option<u64>,
//     offset: Option<u64>,
//     total: Option<u64>,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga {
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: Attributes,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attributes {
    pub title: Title,
    #[serde(rename = "altTitles")]
    pub alt_titles: Vec<Title>,
    pub description: Title,             // Used here because the types are the same, idk why
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    pub links: Title,                    // Used here because the types are the same, idk why
    #[serde(rename = "originalLanguage")]
    pub original_language: String,
    #[serde(rename = "lastVolume")]
    pub last_volume: String,
    #[serde(rename = "lastChapter")]
    pub last_chapter: String,
    #[serde(rename = "publicationDemgraphic")]
    pub publication_demographic: String,
    pub status: String,
    pub year: u64,
    #[serde(rename = "contentRating")]
    pub content_rating: String,
    #[serde(rename = "chapterNumbersResetOnNewVolume")]
    pub chapter_numbers_reset_on_new_volume: bool,
    #[serde(rename = "availableTranslatedLanguages")]
    pub available_translated_languages: Vec<serde_json::Value>,
    #[serde(rename = "latestUploadedChapter")]
    pub latest_uploaded_chapter: uuid::Uuid,
    pub tags: Vec<serde_json::Value>,
    pub state: String,
    pub version: u64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub related: String,
    pub attributes: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub property1: String,
    pub property2: String,
}
