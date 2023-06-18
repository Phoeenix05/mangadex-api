use uuid::Uuid;

use crate::endpoint::Endpoint;
use crate::json::{error::ErrorResponse, statistics::Statistics};
use crate::types::get::*;
use crate::util::{BASE_URL, CLIENT};

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

mod athome {
    use super::*;

    #[cfg(any(feature = "wrapper", feature = "dl"))]
    #[async_trait::async_trait]
    impl Endpoint for AtHomeServer {
        async fn get() -> Result<Self, ErrorResponse> {
            unimplemented!()
        }

        async fn get_uuid(chapter_uuid: Uuid) -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/at-home/server/{chapter_uuid}"))
        }

        async fn get_statistics(_uuid: Uuid) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }

        async fn get_statistics_list(_uuid: Vec<Uuid>) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }
    }
}

mod author {
    use super::*;

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for Author {
        async fn get() -> Result<Self, ErrorResponse> {
            unimplemented!()
        }

        async fn get_uuid(author_uuid: Uuid) -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/author/{author_uuid}"))
        }

        async fn get_statistics(_uuid: Uuid) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }

        async fn get_statistics_list(_uuid: Vec<Uuid>) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }
    }

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for AuthorList {
        async fn get() -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/author"))
        }

        async fn get_uuid(_uuid: Uuid) -> Result<Self, ErrorResponse> {
            unimplemented!()
        }

        async fn get_statistics(_uuid: Uuid) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }

        async fn get_statistics_list(_uuid: Vec<Uuid>) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }
    }
}

mod chapter {
    use super::*;

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for Chapter {
        async fn get() -> Result<Self, ErrorResponse> {
            unimplemented!()
        }

        async fn get_uuid(chapter_uuid: Uuid) -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/chapter/{chapter_uuid}"))
        }

        async fn get_statistics(chapter_uuid: Uuid) -> Result<Statistics, ErrorResponse> {
            let url = format!("{BASE_URL}/statistics/chapter/{chapter_uuid}");
            get_api_unwrap!(url)
        }

        async fn get_statistics_list(_uuid: Vec<Uuid>) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }
    }

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for ChapterList {
        async fn get() -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/chapter"))
        }

        async fn get_uuid(_uuid: Uuid) -> Result<Self, ErrorResponse> {
            unimplemented!()
        }

        async fn get_statistics(_uuid: Uuid) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }

        async fn get_statistics_list(uuid_list: Vec<Uuid>) -> Result<Statistics, ErrorResponse> {
            let uuid_string = uuid_list
                .into_iter()
                .map(|u| u.to_string())
                .collect::<Vec<_>>()
                .join("&chapter[]=");

            let url = format!("{BASE_URL}/statistics/chapter?chapter[]={uuid_string}");
            get_api_unwrap!(url)
        }
    }
}

mod cover {
    use super::*;

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for Cover {
        async fn get() -> Result<Self, ErrorResponse> {
            unimplemented!()
        }

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

        async fn get_statistics(_uuid: Uuid) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }

        async fn get_statistics_list(_uuid: Vec<Uuid>) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }
    }

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for CoverArtList {
        async fn get() -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/cover"))
        }

        async fn get_uuid(_uuid: Uuid) -> Result<Self, ErrorResponse> {
            unimplemented!()
        }

        async fn get_statistics(_uuid: Uuid) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }

        async fn get_statistics_list(_uuid: Vec<Uuid>) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }
    }
}

mod manga {
    use super::*;

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for Manga {
        async fn get() -> Result<Self, ErrorResponse> {
            unimplemented!()
        }

        async fn get_uuid(manga_uuid: Uuid) -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/manga/{manga_uuid}"))
        }

        async fn get_statistics(manga_uuid: Uuid) -> Result<Statistics, ErrorResponse> {
            let url = format!("{BASE_URL}/statistics/manga/{manga_uuid}");
            get_api_unwrap!(url)
        }

        async fn get_statistics_list(_uuid: Vec<Uuid>) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }
    }

    // #[cfg(any(feature = "wrapper", feature = "dl"))]
    // #[async_trait::async_trait]
    // impl Endpoint for MangaFeed {
    //     async fn get() -> Result<Self, ErrorResponse> {
    //         unimplemented!()
    //     }

    //     async fn get_uuid(manga_uuid: Uuid) -> Result<Self, ErrorResponse> {
    //         get_api_unwrap!(format!("{BASE_URL}/manga/{manga_uuid}/feed"))
    //     }

    //     async fn get_statistics(_uuid: Uuid) -> Result<Statistics, ErrorResponse> {
    //         unimplemented!()
    //     }

    //     async fn get_statistics_list(_uuid: Vec<Uuid>) -> Result<Statistics, ErrorResponse> {
    //         unimplemented!()
    //     }
    // }

    #[cfg(any(feature = "wrapper"))]
    #[async_trait::async_trait]
    impl Endpoint for MangaList {
        async fn get() -> Result<Self, ErrorResponse> {
            get_api_unwrap!(format!("{BASE_URL}/manga"))
        }

        async fn get_uuid(_uuid: Uuid) -> Result<Self, ErrorResponse> {
            unimplemented!()
        }

        async fn get_statistics(_uuid: Uuid) -> Result<Statistics, ErrorResponse> {
            unimplemented!()
        }

        async fn get_statistics_list(uuid_list: Vec<Uuid>) -> Result<Statistics, ErrorResponse> {
            let uuid_string = uuid_list
                .into_iter()
                .map(|u| u.to_string())
                .collect::<Vec<_>>()
                .join("&manga[]=");

            let url = format!("{BASE_URL}/statistics/manga?manga[]={uuid_string}");
            get_api_unwrap!(url)
        }
    }
}
