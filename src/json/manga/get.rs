use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga<T> {
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: T,
    pub relationships: Vec<Relationship>,
}

/// Provides types for API endpoint `/manga`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaList {
    pub result: String,
    pub response: String,
    pub data: Vec<Manga<ListDataAttr>>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListDataAttr {
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
    pub tags: Vec<serde_json::Value>,
    pub state: String,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
}

/// Provides types for API endpoint `/manga/{chapterId}/feed`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MangaFeed {
    pub result: String,
    pub response: String,
    pub data: Vec<Manga<FeedDataAttr>>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedDataAttr {
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

    #[ignore = "wrapper::get::tests::manga_list does basically the same thing"]
    #[tokio::test]
    async fn manga_list() {
        let res = fetch("https://api.mangadex.org/manga")
            .await
            .unwrap()
            .json::<MangaList>()
            .await;

        assert!(res.is_ok(), "{res:#?}")
    }

    #[ignore = "wrapper::get::tests::manga_feed does basically the same thing"]
    #[tokio::test]
    async fn manga_feed() {
        let test_cases = vec![
            "e6eb6bd0-0285-4fac-a6da-9bc4234ac1bb",
            "b35163ef-efbf-4cb9-bf97-0acc89200455",
            "bd6d0982-0091-4945-ad70-c028ed3c0917",
            "c7421641-dc50-4a3c-80a5-5cdcb2cae890",
            "1e3feba8-0c4d-4465-b3a3-e9a2a4451bd1",
        ];

        for case in test_cases.into_iter() {
            let url = format!("https://api.mangadex.org/manga/{case}/feed");
            let res = fetch(url.as_str()).await.unwrap().json::<MangaFeed>().await;
            assert!(res.is_ok(), "{res:#?}")
        }
    }
}
