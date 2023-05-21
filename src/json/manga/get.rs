use super::*;

/// Provides types for API endpoint `/manga`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub result: String,
    pub response: String,
    pub data: Vec<Manga<ListDataAttr>>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
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

/// Provides types for API endpoint `/manga/{chapterId}/feed`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    pub result: String,
    pub response: String,
    pub data: Vec<Manga<FeedDataAttr>>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::tests::fetch;

    #[tokio::test]
    async fn manga_list() {
        let res = fetch("https://api.mangadex.org/manga")
            .await
            .unwrap()
            .json::<List>()
            .await;

        assert!(res.is_ok(), "{res:#?}")
    }

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
            let res = fetch(url.as_str()).await.unwrap().json::<Feed>().await;
            assert!(res.is_ok(), "{res:#?}")
        }
    }
}
