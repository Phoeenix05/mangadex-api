//! This module provides the `Client` to fetch data from MangaDex API.
//!
//! This module might change or disappear completely in the future. Currently
//! the way the client is implemented makes all the methods from all the implementations
//! available for all client implementations. That is why the client has to be
//! initialized with `Client::<Manga>::new()` and why you can't do `let client: Client<Manga> = Client::new()`.
//!
//! ```
//! use mangadex_api::prelude::{Client, Manga};
//!
//! #[tokio::main]
//! async fn main() {
//!     let uuid = uuid::uuid!("c288b108-5162-4065-aa3a-5857ec38c8d9");
//!     let client = Client::<Manga>::new(uuid);
//!     let manga_data = client.get().await.unwrap();
//! }
//! ```

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use crate::json::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct ClientError {
    pub msg: String,
    pub api_msg: Option<ApiError>,
}

/// MangaDex API Client. API client to fetch data from MangaDex API.
///
/// ```
/// use mangadex_api::prelude::{Client, Manga};
///
/// let client = Client::<Manga>::new(uuid::uuid!("c288b108-5162-4065-aa3a-5857ec38c8d9"));
/// ```
#[derive(Debug)]
pub struct Client<T>
where
    T: DeserializeOwned + Serialize,
{
    pub uuid: Option<Uuid>,
    pub _phantom: std::marker::PhantomData<T>,
}

impl<T: DeserializeOwned + Serialize> Client<T> {
    pub fn get_uuid(&self) -> &Option<Uuid> {
        &self.uuid
    }
}
