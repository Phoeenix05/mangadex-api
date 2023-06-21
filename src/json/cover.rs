use serde::{Deserialize, Serialize};

use crate::{prelude::*, unwrap_api_results, util::CLIENT, uuid_or_err};

////////////////////////////////////////////////////////////////
/// Cover
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cover;

impl Client<Cover> {
    pub fn new() -> Self {
        todo!()
    }

    pub async fn get(self) -> Result<Cover, ClientError> {
        todo!()
    }
}

////////////////////////////////////////////////////////////////
/// Cover list
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoverList;

impl Client<CoverList> {
    pub fn new() -> Self {
        todo!()
    }

    pub async fn get(self) -> Result<CoverList, ClientError> {
        todo!()
    }
}
