//! Using reqwest in a Tauri application can be a pain
//! especially when using Leptos or Yew. Tauri doesn't have that
//! great of a support for WASM applications written in Rust or
//! any other language. Trying to use the Tauri's Rust API in
//! my Leptos application with Tauri just gave me compile
//! errors.
//!
//! This module is supposed to make it easier to call the API
//! wrapper from the frontend (Leptos, Yew, etc...).

// use serde::{de::DeserializeOwned, Deserialize, Serialize};
// use uuid::Uuid;

// use crate::{json::error::ErrorResponse, prelude::*};

// pub async fn fetch_endpoint() -> Result<String, ErrorResponse> {
//     Ok(serde_json::to_string(&res).unwrap())
// }
