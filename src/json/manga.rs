use serde::Deserialize;

use super::{Data, Tag};

pub type Manga = Data<Attributes>;
pub type MangaList = Vec<Data<Attributes>>;

#[derive(Debug, Clone, Deserialize)]
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
    pub available_translated_languages: Vec<serde_json::Value>,
    pub latest_uploaded_chapter: uuid::Uuid,
    pub tags: Vec<Data<Tag>>,
    pub state: String,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
}
