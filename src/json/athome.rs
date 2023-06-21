use serde::{Deserialize, Serialize};

use crate::{prelude::*, unwrap_api_results, util::CLIENT, uuid_or_err};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AtHomeServer;

impl Client<AtHomeServer> {
    pub fn new() -> Self {
        todo!()
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
