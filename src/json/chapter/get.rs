use super::*;

/// Provides types for API endpoint `/chapter`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChapterList {
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
#[serde(rename_all = "camelCase")]
pub struct DataAttr {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fetch;

    #[tokio::test]
    async fn chapter_list() {
        let res = fetch("https://api.mangadex.org/chapter")
            .await
            .unwrap()
            .json::<ChapterList>()
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
