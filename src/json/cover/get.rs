use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub volume: Option<String>,
    pub file_name: String,
    pub description: String,
    pub locale: String,
    pub version: u64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
