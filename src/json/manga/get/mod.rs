mod feed;
mod list;

pub use feed::MangaFeed;
pub use list::MangaList;

use serde::Deserialize;

use crate::json::{Data, Response};

pub type Manga = Response<Data<Attributes>>;

#[derive(Debug, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fetch;

    #[tokio::test]
    async fn test() {
        let test_cases = vec![
            "e6eb6bd0-0285-4fac-a6da-9bc4234ac1bb",
            "b35163ef-efbf-4cb9-bf97-0acc89200455",
            "bd6d0982-0091-4945-ad70-c028ed3c0917",
            "c7421641-dc50-4a3c-80a5-5cdcb2cae890",
            "1e3feba8-0c4d-4465-b3a3-e9a2a4451bd1",
        ];

        for case in test_cases.into_iter() {
            let url = format!("https://api.mangadex.org/manga/{case}");
            let res = fetch(url.as_str()).await.unwrap().json::<Manga>().await;
            assert!(res.is_ok(), "{res:#?}")
        }
    }
}
