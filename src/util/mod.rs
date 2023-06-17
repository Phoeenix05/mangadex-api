#[cfg(feature = "tauri")]
pub mod tauri;

use std::{path::PathBuf, sync::Arc};

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};

pub const BASE_URL: &str = "https://api.mangadex.org";

lazy_static::lazy_static! {
    pub static ref CLIENT: Arc<ClientWithMiddleware> = {
        let manager = CACacheManager {
            path: if let Some(mut path) = dirs::cache_dir() {
               path.push("mangadex_api-cacache");
               path
            } else {
                PathBuf::from("./mangadex_api-cacache")
            },
        };
        let http_cache = Cache(HttpCache {
            mode: CacheMode::Default,
            manager,
            options: None
        });

        let client = ClientBuilder::new(Client::new()).with(http_cache).build();
        Arc::new(client)
    };
}
