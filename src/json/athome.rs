use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{prelude::*, unwrap_api_results, util::CLIENT, uuid_or_err};

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
        let uuid = uuid_or_err!(self.get_uuid()).unwrap();
        let res = CLIENT
            .get(ClientUtil::construct_url(
                format!("/at-home/server/{uuid}"),
                None,
            ))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}
