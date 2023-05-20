mod get;
pub use get::*;

use serde::{Deserialize, Serialize};

use crate::Relationship;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manga<T> {
    pub id: uuid::Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: T,
    pub relationships: Vec<Relationship>,
}
