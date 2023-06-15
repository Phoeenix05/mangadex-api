use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub result: String,
    pub errors: Vec<Error>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Error {
    pub id: Uuid,
    pub status: u64,
    pub title: String,
    pub detail: String,
}
