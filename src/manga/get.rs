use super::*;

/// Provides types for API endpoint `/manga`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub result: String,
    pub response: String,
    pub data: Vec<Manga>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
}

#[cfg(test)]
mod tests {
    use crate::Err400;

    use super::*;

    #[tokio::test]
    async fn manga_list_200() {
        let client = reqwest::Client::new();

        let res = client
            .get("https://api.mangadex.org/manga")
            .send()
            .await
            .unwrap()
            .json::<List>()
            .await;
        
        assert!(res.is_ok())
    }

    #[tokio::test]
    async fn manga_list_400() {
        let client = reqwest::Client::new();

        let res = client
            .get("https://api.mangadex.org/manga?fail")
            .send()
            .await
            .unwrap()
            .json::<Err400>()
            .await;
        
        assert!(res.is_ok())
    }
}
