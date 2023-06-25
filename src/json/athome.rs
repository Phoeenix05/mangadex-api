use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::client::util::construct_url;
use crate::prelude::*;
use crate::util::*;
use crate::{unwrap_api_results, uuid_or_err};

////////////////////////////////////////////////////////////////
/// Structs
////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////
/// Implementations
////////////////////////////////////////////////////////////////

impl AtHomeServer {
    pub fn result(&self) -> &String {
        &self.result
    }

    pub fn base_url(&self) -> &String {
        &self.base_url
    }

    pub fn data(&self) -> &Data {
        &self.data
    }
}

impl Client<AtHomeServer> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<AtHomeServer, ClientError> {
        let uuid = uuid_or_err!(self.get_uuid()).unwrap();
        let res = CLIENT
            .get(construct_url(format!("/at-home/server/{uuid}"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

impl Data {
    pub fn hash(&self) -> &String {
        &self.hash
    }

    pub fn data(&self) -> &Vec<String> {
        &self.data
    }

    pub fn data_saver(&self) -> &Vec<String> {
        &self.data_saver
    }
}
