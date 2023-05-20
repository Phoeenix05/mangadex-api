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
pub struct Feed {
    pub result: String,
    pub response: String,
    pub data: Vec<Manga<FeedDataAttr>>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
}

#[cfg(test)]
mod tests {
    use reqwest::{Response, Result};

    use super::*;

    async fn fetch(url: &str) -> Result<Response> {
        let client: reqwest::Client = reqwest::Client::new();
        let res = client.get(url).send().await;
        res
    }

    #[tokio::test]
    async fn manga_list_200() {
        let res = fetch("https://api.mangadex.org/manga")
            .await
            .unwrap()
            .json::<List>()
            .await;

        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn manga_feed_200() {
        let res = fetch("https://api.mangadex.org/manga/e6eb6bd0-0285-4fac-a6da-9bc4234ac1bb/feed")
            .await
            .unwrap()
            .json::<Feed>()
            .await;
        dbg!(&res);
        assert!(res.is_ok())
    }
}
