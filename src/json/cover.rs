use serde::Deserialize;

use super::Data;

pub type Cover = Data<Attributes>;
pub type CoverArtList = Vec<Data<Attributes>>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub volume: String,
    pub file_name: String,
    pub description: String,
    pub locale: String,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
}
