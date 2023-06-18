#[cfg(any(feature = "wrapper", feature = "dl"))]
use std::{path::PathBuf, sync::Arc};

#[cfg(any(feature = "wrapper", feature = "dl"))]
pub const BASE_URL: &str = "https://api.mangadex.org";

#[cfg(any(feature = "wrapper", feature = "dl"))]
lazy_static::lazy_static! {
    pub static ref CLIENT: Arc<reqwest_middleware::ClientWithMiddleware> = {
        use reqwest::Client;
        use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
        use reqwest_middleware::ClientBuilder;

        let client = ClientBuilder::new(Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager {
                    path: if let Some(mut path) = dirs::cache_dir() {
                        path.push("mangadex_api-cacache");
                        path
                    } else {
                        PathBuf::from("./mangadex_api-cacache")
                    },
                },
                options: None
            }))
            .build();
        Arc::new(client)
    };
}
