use super::*;
use serde::{Deserialize, Serialize};

mod get;
pub use get::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga<T> {
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: T,
    pub relationships: Vec<Relationship>,
}
