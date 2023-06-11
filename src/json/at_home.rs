use serde::{Deserialize, Serialize};

/// Provides types for API endpoint `/at-home/server/{chapterId}`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtHomeServer {
    pub result: String,
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "chapter")]
    pub data: Data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub hash: String,
    pub data: Vec<String>,
    #[serde(rename = "dataSaver")]
    pub data_saver: Vec<String>,
}
