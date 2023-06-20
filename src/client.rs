use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::json::*;

#[derive(Debug, Deserialize)]
pub struct ClientError {
    pub msg: String,
    pub api_msg: Option<ApiError>,
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:#?}")
    }
}

#[async_trait]
pub trait ApiRoute<T>
where
    T: Clone + DeserializeOwned + Serialize,
{
    async fn get(mut self) -> Result<T, ClientError>;
}

#[async_trait]
pub trait Statistics<T>
where
    T: Clone + DeserializeOwned + Serialize,
{
    async fn get_statistics(mut self) -> Result<T, ClientError>;
}

#[derive(Debug)]
pub struct Client<T>
where
    T: Clone + DeserializeOwned + Serialize,
{
    uuid: Option<Uuid>,
    _t: std::marker::PhantomData<T>,
}

impl<T: Clone + DeserializeOwned + Serialize> Client<T> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            _t: std::marker::PhantomData,
        }
    }

    pub fn set_uuid(mut self, uuid: Uuid) -> Self {
        self.uuid = Some(uuid);
        self
    }

    pub fn uuid(&self) -> &Option<Uuid> {
        &self.uuid
    }
}

impl<T: Clone + DeserializeOwned + Serialize> Default for Client<T> {
    fn default() -> Self {
        Self::new()
    }
}
