use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

use crate::json::*;

#[derive(Debug)]
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
pub trait ApiRoute
where
    Self: Sized,
{
    async fn get(mut self) -> Result<Self, ClientError>;
}

#[async_trait]
pub trait Statistics
where
    Self: Sized,
{
    async fn get_statistics(mut self) -> Result<Self, ClientError>;
}

#[derive(Debug)]
pub struct Client<T>
where
    T: Clone + DeserializeOwned + Serialize,
{
    uuid: Option<Uuid>,
    pub data: Option<T>,
}

impl<T: Clone + DeserializeOwned + Serialize> Client<T> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            data: None,
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
