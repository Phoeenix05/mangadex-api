use serde::{Deserialize, Serialize};

use crate::json::{Data, Response};

pub type Chapter = Response<Data<Attributes>>;
pub type ChapterList = Response<Vec<Data<Attributes>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let res = fetch("https://api.mangadex.org/chapter/029b7226-5673-41d2-9ae6-09793f200bd9")
            .await
            .unwrap()
            .json::<Chapter>()
            .await;

        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn test_list() {
        let res = fetch("https://api.mangadex.org/chapter")
            .await
            .unwrap()
            .json::<ChapterList>()
            .await;

        assert!(res.is_ok(), "{res:#?}")
    }
}
