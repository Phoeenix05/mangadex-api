use serde::{Serialize, Deserialize};

use crate::Relationship;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub result: String,
    pub response: String,
    pub data: Vec<ChapterData>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub result: String,
    pub response: String,
    pub data: ChapterData,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterData {
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: DataAttr,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataAttr {
    pub title: String,
    pub volume: String,
    pub chapter: String,
    pub pages: u64,
    #[serde(rename = "translatedLanguage")]
    pub translated_language: String,
    pub uploader: uuid::Uuid,
    #[serde(rename = "externalUrl")]
    pub external_url: String,
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