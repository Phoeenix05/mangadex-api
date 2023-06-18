use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::Data;

pub type Chapter = Data<Attributes>;
pub type ChapterList = Vec<Data<Attributes>>;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub title: Option<String>,
    pub volume: Option<String>,
    pub chapter: String,
    pub pages: u64,
    pub translated_language: String,
    pub uploader: Option<Uuid>,
    pub external_url: Option<Url>,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
    pub publish_at: String,
    pub readable_at: String,
}
