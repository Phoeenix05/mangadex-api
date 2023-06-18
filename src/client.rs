use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use uuid::Uuid;

use crate::json::*;

#[async_trait]
pub trait ApiRoute
where
    Self: Sized,
{
    async fn get(mut self) -> Result<Self, ApiError>;
}

#[async_trait]
pub trait Statistics
where
    Self: Sized,
{
    async fn get_statistics(mut self) -> Result<Self, ApiError>;
}

#[derive(Debug)]
pub struct Client<T>
where
    T: Clone + DeserializeOwned + Serialize,
{
    uuid: Option<Uuid>,
    pub data: Option<T>,
    // _api_route: std::marker::PhantomData<T>,
}

impl<T: Clone + DeserializeOwned + Serialize> Client<T> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            data: None,
            // _api_route: PhantomData,
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
