use uuid::Uuid;

use crate::json::{error::ErrorResponse, statistics::Statistics};

#[cfg(any(feature = "wrapper", feature = "dl"))]
#[async_trait::async_trait]
pub trait Endpoint
where
    Self: Sized,
{
    async fn get() -> Result<Self, ErrorResponse>;

    async fn get_uuid(uuid: Uuid) -> Result<Self, ErrorResponse>;

    /// This method will always return a Success. Atleast based on MangaDex API.
    async fn get_statistics(uuid: Uuid) -> Result<Statistics, ErrorResponse>;

    /// This method will always return a Success. Atleast based on MangaDex API.
    async fn get_statistics_list(uuid: Vec<Uuid>) -> Result<Statistics, ErrorResponse>;
}
