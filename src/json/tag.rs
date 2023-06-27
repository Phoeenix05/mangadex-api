use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    client,
    prelude::{Client, ClientError},
    unwrap_api_results,
    util::client::construct_url,
};

use super::Relationship;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagList {
    pub result: String,
    pub response: String,
    pub data: Vec<Tag>,
    pub limit: u64,
    pub offset: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub id: Uuid,
    #[serde(rename = "type")]
    pub data_type: String,
    pub attributes: TagAttributes,
    pub relationships: Vec<Relationship>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TagAttributes {
    pub name: HashMap<String, String>,
    pub description: HashMap<String, String>,
    pub group: String,
    pub version: u64,
}

impl Client<TagList> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<TagList, ClientError> {
        let client = client!(CacheMode::Default);
        let res = client
            .get(construct_url("/manga/tag".into(), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}
