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
pub struct Manga<T>
where
    T: Serialize,
{
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: T,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDataAttr {
    pub title: serde_json::Value,
    #[serde(rename = "altTitles")]
    pub alt_titles: Vec<serde_json::Value>,
    pub description: serde_json::Value,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    pub links: serde_json::Value,
    #[serde(rename = "originalLanguage")]
    pub original_language: String,
    #[serde(rename = "lastVolume")]
    pub last_volume: String,
    #[serde(rename = "lastChapter")]
    pub last_chapter: String,
    #[serde(rename = "publicationDemographic")]
    pub publication_demographic: Option<String>,
    pub status: String,
    pub year: Option<u64>,
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
pub struct FeedDataAttr {
    pub title: Option<String>,
    pub volume: Option<String>,
    pub chapter: String,
    pub pages: u64,
    #[serde(rename = "translatedLanguage")]
    pub translated_language: String,
    pub uploader: Option<uuid::Uuid>,
    #[serde(rename = "externalUrl")]
    pub external_url: Option<String>,
    pub version: u64,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    #[serde(rename = "publishAt")]
    pub published_at: String,
    #[serde(rename = "readableAt")]
    pub readable_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub related: Option<String>,
    pub attributes: Option<serde_json::Value>,
}
