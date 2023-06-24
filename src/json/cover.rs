use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::prelude::*;

use super::Data;

////////////////////////////////////////////////////////////////
/// Cover
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cover {
    result: String,
    response: String,
    data: Data<CoverAttributes>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct CoverAttributes {
    volume: Option<String>,
    file_name: String,
    description: String,
    locale: String,
    version: u64,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl Client<Cover> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<Cover, ClientError> {
        todo!()
    }
}

////////////////////////////////////////////////////////////////
/// Cover list
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoverList {
    result: String,
    response: String,
    data: Vec<Data<CoverAttributes>>,
    limit: u64,
    offset: u64,
    total: u64,
}

impl Client<CoverList> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<CoverList, ClientError> {
        todo!()
    }
}
