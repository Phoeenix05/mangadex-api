use serde::{Deserialize, Serialize};

/// Provides types for API endpoint `/at-home/server/{chapterId}`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtHomeServer {
    pub result: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub chapter: Chapter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub hash: String,
    pub data: Vec<String>,
    #[serde(rename = "dataSaver")]
    pub data_saver: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn server() {
        let client = reqwest::Client::new();
        let res = client
            .get("https://api.mangadex.org/at-home/server/02fe08b6-eb3b-4875-9c2c-ad5a66520f56")
            .send()
            .await
            .unwrap()
            .json::<AtHomeServer>()
            .await;

        assert!(res.is_ok(), "{res:#?}")
    }
}
