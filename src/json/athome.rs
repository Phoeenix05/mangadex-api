use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtHomeServer {
    pub result: String,
    pub base_url: Url,
    pub chapter: Chapter,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    pub hash: String,
    pub data: Vec<String>,
    pub data_saver: Vec<String>,
}
