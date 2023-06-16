use uuid::Uuid;

use crate::util::{BASE_URL, CLIENT};
use crate::{json::error::ErrorResponse, types::get::*};

macro_rules! unwrap_results {
    ($res:expr) => {
        if $res.status().is_success() {
            let json = $res.json().await.unwrap();
            Ok(json)
        } else {
            let json = $res.json().await.unwrap();
            Err(json)
        }
    };
}

macro_rules! get_api_unwrap {
    ($url:expr) => {{
        let res = CLIENT.get($url).send().await.unwrap();
        unwrap_results!(res)
    }};
}

#[allow(unused_macros)]
macro_rules! post_put_api_unwrap {
    ($url:expr, $body:expr) => {{
        let res = CLIENT.post($url).body($body).send().await.unwrap();
        unwrap_results!(res)
    }};
}

#[allow(unused_macros)]
macro_rules! del_api_unwrap {
    ($url:expr) => {{
        let res = CLIENT.delete($url).send().await.unwrap();
        unwrap_results!(res)
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

mod athome {
    use super::*;

    #[cfg(any(feature = "wrapper", feature = "dl"))]
    #[async_trait::async_trait]
    impl Endpoint for AtHomeServer {
        async fn get_uuid(chapter_uuid: Uuid) -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/at-home/server/{chapter_uuid}"))
        }
    }
}

mod author {
    use super::*;

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for Author {
        async fn get_uuid(author_uuid: Uuid) -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/author/{author_uuid}"))
        }
    }

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for AuthorList {
        async fn get() -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/author"))
        }
    }
}

mod chapter {
    use super::*;

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for Chapter {
        async fn get_uuid(chapter_uuid: Uuid) -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/chapter/{chapter_uuid}"))
        }
    }

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for ChapterList {
        async fn get() -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/chapter"))
        }
    }
}

mod cover {
    use super::*;

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for Cover {
        async fn get_uuid(manga_uuid: Uuid) -> Result<Self, ErrorResponse> {
            let res = Manga::get_uuid(manga_uuid).await;

            let cover_uuid = if res.is_ok() {
                let json = res.unwrap();
                json.data
                    .relationships
                    .into_iter()
                    .find(|p| p.data_type == "cover_art")
                    .unwrap()
                    .id
            } else {
                let json = res.unwrap_err();
                return Err(json);
            };

            get_api_unwrap!(format!("{BASE_URL}/cover/{cover_uuid}"))
        }
    }

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for CoverArtList {
        async fn get() -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/cover"))
        }
    }
}

mod manga {
    use super::*;

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for Manga {
        async fn get_uuid(manga_uuid: Uuid) -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/manga/{manga_uuid}"))
        }
    }

    #[cfg(any(feature = "wrapper", feature = "dl"))]
    #[async_trait::async_trait]
    impl Endpoint for MangaFeed {
        async fn get_uuid(manga_uuid: Uuid) -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/manga/{manga_uuid}/feed"))
        }
    }

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for MangaList {
        async fn get() -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/manga"))
        }
    }
}

mod statistics {
    use super::*;

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for ChapterStatistics {
        async fn get_uuid(chapter_uuid: Uuid) -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/statistics/chapter/{chapter_uuid}"))
        }
    }
}
