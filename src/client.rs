use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::json::*;

////////////////////////////////////////////////////////////////
/// Structs
////////////////////////////////////////////////////////////////

#[derive(Debug, Deserialize)]
pub struct ClientError {
    pub msg: String,
    pub api_msg: Option<ApiError>,
}

#[derive(Debug)]
pub struct Client<T>
where
    T: DeserializeOwned + Serialize,
{
    pub uuid: Option<Uuid>,
    // pub url: String,
    pub _phantom: std::marker::PhantomData<T>,
}

pub struct ClientUtil;

////////////////////////////////////////////////////////////////
/// Implementations
////////////////////////////////////////////////////////////////
impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

impl<T: DeserializeOwned + Serialize> Client<T> {
    pub fn get_uuid(&self) -> &Option<Uuid> {
        &self.uuid
    }
}
