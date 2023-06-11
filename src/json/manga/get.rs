use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub title: serde_json::Value,
    pub alt_titles: Vec<serde_json::Value>,
    pub description: serde_json::Value,
    pub is_locked: bool,
    pub links: serde_json::Value,
    pub original_language: String,
    pub last_volume: Option<String>,
    pub last_chapter: Option<String>,
    pub publication_demographic: Option<String>,
    pub status: String,
    pub year: Option<u64>,
    pub content_rating: String,
    pub chapter_numbers_reset_on_new_volume: bool,
    pub available_translated_languages: Vec<serde_json::Value>, // Vec<String>
    pub latest_uploaded_chapter: uuid::Uuid,
    pub tags: Vec<serde_json::Value>,
    pub state: String,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedAttributes {
    pub title: Option<String>,
    pub volume: Option<String>,
    pub chapter: String,
    pub pages: u64,
    pub translated_language: String,
    pub uploader: Option<uuid::Uuid>,
    pub external_url: Option<String>,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
    pub publish_at: String,
    pub readable_at: String,
}
