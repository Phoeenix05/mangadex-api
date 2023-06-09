use serde::{Deserialize, Serialize};

use crate::json::{Data, Response};

pub type MangaFeed = Response<Vec<Data<Attributes>>>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
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
    async fn test() {
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
