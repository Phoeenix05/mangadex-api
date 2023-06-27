//! This module provides types for athome server endpoint.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;
use crate::util::client::construct_url;
use crate::{client, unwrap_api_results, uuid_or_err};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtHomeServer {
    pub result: String,
    pub base_url: String,
    #[serde(rename = "chapter")]
    pub data: Data,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub hash: String,
    pub data: Vec<String>,
    pub data_saver: Vec<String>,
}

impl Client<AtHomeServer> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<AtHomeServer, ClientError> {
        let client = client!(CacheMode::Default);

        let uuid = uuid_or_err!(self.get_uuid()).unwrap();
        let res = client
            .get(construct_url(format!("/at-home/server/{uuid}"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}
