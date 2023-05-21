use serde::{Deserialize, Serialize};

use crate::Relationship;

/// Provides types for API endpoint `/chapter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub result: String,
    pub response: String,
    pub data: Vec<ChapterData>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
}

/// Provides types for API endpoint `/chapter/{id}`
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fetch;

    #[tokio::test]
    async fn chapter_list() {
        let res = fetch("https://api.mangadex.org/chapter")
            .await
            .unwrap()
            .json::<List>()
            .await;

        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn chapter() {
        let res = fetch("https://api.mangadex.org/chapter/029b7226-5673-41d2-9ae6-09793f200bd9")
            .await
            .unwrap()
            .json::<Chapter>()
            .await;

        assert!(res.is_ok(), "{res:#?}")
    }
}
