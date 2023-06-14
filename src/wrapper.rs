use uuid::Uuid;

use crate::util::CLIENT;
use crate::{json::error::ErrorResponse, types::get::*};

///// Macros
macro_rules! get_api_unwrap {
    ($url:expr) => {{
        let res = CLIENT.get($url).send().await.unwrap();
        if res.status().is_success() {
            let json = res.json().await.unwrap();
            Ok(json)
        } else {
            let json = res.json().await.unwrap();
            Err(json)
        }
    }};
}

#[allow(unused_macros)]
macro_rules! post_put_api_unwrap {
    ($url:expr, $body:expr) => {{
        let res = CLIENT.post($url).body($body).send().await.unwrap();
        if res.status().is_success() {
            let json = res.json().await.unwrap();
            Ok(json)
        } else {
            let json = res.json().await.unwrap();
            Err(json)
        }
    }};
}

// #[allow(unused_macros)]
// macro_rules! put_api_unwrap {
//     ($url:expr, $body:expr) => {{
//         let res = CLIENT.put($url).body($body).send().await.unwrap();
//         if res.status().is_success() {
//             let json = res.json().await.unwrap();
//             Ok(json)
//         } else {
//             let json = res.json().await.unwrap();
//             Err(json)
//         }
//     }};
// }

#[allow(unused_macros)]
macro_rules! del_api_unwrap {
    ($url:expr) => {{
        let res = CLIENT.delete($url).send().await.unwrap();
        if res.status().is_success() {
            let json = res.json().await.unwrap();
            Ok(json)
        } else {
            let json = res.json().await.unwrap();
            Err(json)
        }
    }};
}

#[cfg(any(feature = "wrapper", feature = "dl"))]
#[async_trait::async_trait]
pub trait Endpoint
where
    Self: Sized,
{
    async fn get() -> Result<Self, ErrorResponse> {
        unimplemented!()
    }

    async fn get_uuid(_uuid: Uuid) -> Result<Self, ErrorResponse> {
        unimplemented!()
    }
}

#[cfg(any(feature = "wrapper", feature = "dl"))]
#[async_trait::async_trait]
impl Endpoint for AtHomeServer {
    async fn get_uuid(chapter_uuid: Uuid) -> Result<Self, ErrorResponse> {
        let url = format!("https://api.mangadex.org/at-home/server/{}", chapter_uuid);
        get_api_unwrap!(url)
    }
}

#[cfg(any(feature = "wrapper"))]
#[async_trait::async_trait]
impl Endpoint for Manga {
    async fn get_uuid(manga_uuid: Uuid) -> Result<Self, ErrorResponse> {
        let url = format!("https://api.mangadex.org/manga/{}", manga_uuid);
        get_api_unwrap!(url)
    }
}

#[cfg(any(feature = "wrapper", feature = "dl"))]
#[async_trait::async_trait]
impl Endpoint for MangaFeed {
    async fn get_uuid(manga_uuid: Uuid) -> Result<Self, ErrorResponse> {
        let url = format!("https://api.mangadex.org/manga/{}/feed", manga_uuid);
        get_api_unwrap!(url)
    }
}

#[cfg(any(feature = "wrapper"))]
#[async_trait::async_trait]
impl Endpoint for MangaList {
    async fn get() -> Result<Self, ErrorResponse> {
        get_api_unwrap!("https://api.mangadex.org/manga")
    }
}
