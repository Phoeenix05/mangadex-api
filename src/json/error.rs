use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub result: String,
    pub errors: Vec<Error>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Error {
    pub id: Uuid,
    pub status: u64,
    pub title: String,
    pub detail: String,
}
