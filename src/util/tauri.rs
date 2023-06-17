//! Using reqwest in a Tauri application can be a pain
//! especially when using Leptos or Yew. Tauri doesn't have that
//! great of a support for WASM applications written in Rust or
//! any other language. Trying to use the Tauri's Rust API in
//! my Leptos application with Tauri just gave me compile
//! errors.
//!
//! This module is supposed to make it easier to call the API
//! wrapper from the frontend (Leptos, Yew, etc...).

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum APIRoutes {
    AtHomeServer,
    Chapter,
    ChapterList,
    Cover,
    CoverArtList,
    Manga,
    MangaFeed,
    MangaList,
    Statistics(Statistics),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Statistics {
    Manga,
    Chapter,
}
