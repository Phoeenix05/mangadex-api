use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub result: String,
    pub statistics: serde_json::Value,
}
