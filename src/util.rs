#![cfg(any(test, feature = "wrapper"))]

use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache};
use reqwest::{
    header::{HeaderMap, HeaderValue, USER_AGENT},
    Client,
};
use reqwest_middleware::{ClientBuilder, RequestBuilder};

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Phoeenix05/MangaDex-Reader"),
    );
    headers
}

/// `Path` oarameter does not require `/` at the beginning though
/// putting it there doesn't do anyhting as it is removed if it is there.
/// Not `/manga/{id}` but `manga/{id}`
pub fn request(path: &str) -> RequestBuilder {
    let client_builder = ClientBuilder::new(Client::new()).with(Cache(HttpCache {
        mode: CacheMode::Default,
        manager: CACacheManager::default(),
        options: None,
    }));
    let client = client_builder.build();

    let path = if path.starts_with("/") {
        let mut path = path.to_string();
        path.remove(0);
        path
    } else {
        path.to_string()
    };
    let url = format!("https://api.mangadex.org/{path}",);
    let request = client.get(url).headers(headers());

    request
}
