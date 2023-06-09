use serde::{Deserialize, Serialize};

use crate::json::{Data, Response};

pub type Cover = Response<Data<Attributes>>;
pub type CoverArtList = Response<Vec<Data<Attributes>>>;

#[derive(Debug, Copy, Clone, Serialize, DeserializeDebug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub volume: String,
    pub file_name: String,
    pub description: String,
    pub locale: String,
    pub version: u64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json::Manga;
    use crate::tests::fetch;

    #[tokio::test]
    async fn test() {
        let manga = fetch("https://api.mangadex.org/manga/77bee52c-d2d6-44ad-a33a-1734c1fe696a")
            .await
            .unwrap()
            .json::<Manga>()
            .await
            .unwrap();

        let cover_uuid = manga
            .data
            .relationships
            .into_iter()
            .find(|rel| rel.data_type == "cover_art")
            .unwrap()
            .id;

        let url = format!("https://api.mangadex.org/cover/{cover_uuid}");
        let cover = fetch(url.as_str()).await.unwrap().json::<Cover>().await;
        assert!(cover.is_ok(), "{cover:#?}")
    }

    #[tokio::test]
    async fn test_list() {
        let cover = fetch("https://api.mangadex.org/cover")
            .await
            .unwrap()
            .json::<CoverArtList>()
            .await;

        assert!(cover.is_ok(), "{cover:#?}")
    }
}
