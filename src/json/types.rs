use super::{Data, Response};

#[cfg(feature = "wrapper")]
pub use crate::wrapper::Endpoint;
pub use at_home::*;
pub use author::*;
pub use chapter::*;
pub use cover::*;
pub use manga::*;

pub mod at_home {
    pub use crate::json::at_home::AtHomeServer;

    #[cfg(feature = "wrapper")]
    use crate::json::Err40X;
    #[cfg(feature = "wrapper")]
    use crate::wrapper::Endpoint;
    #[cfg(feature = "wrapper")]
    use uuid::Uuid;

    #[cfg(feature = "wrapper")]
    #[async_trait::async_trait]
    impl Endpoint<AtHomeServer> for AtHomeServer {
        async fn get(_uuid: Option<Uuid>) -> Result<AtHomeServer, Err40X> {
            todo!()
        }
    }
}

pub mod author {
    use super::*;
    use crate::json::author::Attributes;

    #[cfg(feature = "wrapper")]
    use crate::json::Err40X;
    #[cfg(feature = "wrapper")]
    use crate::wrapper::Endpoint;
    #[cfg(feature = "wrapper")]
    use uuid::Uuid;

    pub type Author = Response<Data<Attributes>>;

    #[cfg(feature = "wrapper")]
    #[async_trait::async_trait]
    impl Endpoint<Author> for Author {
        async fn get(_uuid: Option<Uuid>) -> Result<Author, Err40X> {
            todo!()
        }
    }

    pub type AuthorList = Response<Vec<Data<Attributes>>>;

    #[cfg(feature = "wrapper")]
    #[async_trait::async_trait]
    impl Endpoint<AuthorList> for AuthorList {
        async fn get(_: Option<Uuid>) -> Result<AuthorList, Err40X> {
            todo!()
        }
    }
}

pub mod chapter {
    use super::*;
    use crate::json::chapter::Attributes;

    #[cfg(feature = "wrapper")]
    use crate::json::Err40X;
    #[cfg(feature = "wrapper")]
    use crate::wrapper::Endpoint;
    #[cfg(feature = "wrapper")]
    use uuid::Uuid;

    pub type Chapter = Response<Data<Attributes>>;

    #[cfg(feature = "wrapper")]
    #[async_trait::async_trait]
    impl Endpoint<Chapter> for Chapter {
        async fn get(_uuid: Option<Uuid>) -> Result<Chapter, Err40X> {
            todo!()
        }
    }

    pub type ChapterList = Response<Vec<Data<Attributes>>>;

    #[cfg(feature = "wrapper")]
    #[async_trait::async_trait]
    impl Endpoint<ChapterList> for ChapterList {
        async fn get(_: Option<Uuid>) -> Result<ChapterList, Err40X> {
            todo!()
        }
    }
}

pub mod cover {
    use super::*;
    use crate::json::cover::Attributes;

    #[cfg(feature = "wrapper")]
    use crate::json::Err40X;
    #[cfg(feature = "wrapper")]
    use crate::wrapper::Endpoint;
    #[cfg(feature = "wrapper")]
    use uuid::Uuid;

    pub type Cover = Response<Data<Attributes>>;

    #[cfg(feature = "wrapper")]
    #[async_trait::async_trait]
    impl Endpoint<Cover> for Cover {
        async fn get(_uuid: Option<Uuid>) -> Result<Cover, Err40X> {
            todo!()
        }
    }

    pub type CoverArtList = Response<Vec<Data<Attributes>>>;

    #[cfg(feature = "wrapper")]
    #[async_trait::async_trait]
    impl Endpoint<CoverArtList> for CoverArtList {
        async fn get(_: Option<Uuid>) -> Result<CoverArtList, Err40X> {
            todo!()
        }
    }
}

pub mod manga {
    use super::*;
    use crate::json::manga::*;

    #[cfg(feature = "wrapper")]
    use crate::json::Err40X;
    #[cfg(feature = "wrapper")]
    use crate::wrapper::Endpoint;
    #[cfg(feature = "wrapper")]
    use uuid::Uuid;

    pub type Manga = Response<Data<Attributes>>;

    #[cfg(feature = "wrapper")]
    #[async_trait::async_trait]
    impl Endpoint<Manga> for Manga {
        async fn get(_uuid: Option<Uuid>) -> Result<Manga, Err40X> {
            todo!()
        }
    }

    pub type MangaFeed = Response<Vec<Data<FeedAttributes>>>;

    #[cfg(feature = "wrapper")]
    #[async_trait::async_trait]
    impl Endpoint<MangaFeed> for MangaFeed {
        async fn get(_uuid: Option<Uuid>) -> Result<MangaFeed, Err40X> {
            todo!()
        }
    }

    pub type MangaList = Response<Vec<Data<Attributes>>>;

    #[cfg(feature = "wrapper")]
    #[async_trait::async_trait]
    impl Endpoint<MangaList> for MangaList {
        async fn get(_: Option<Uuid>) -> Result<MangaList, Err40X> {
            todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{json::Err40X, util::request};

    #[tokio::test]
    async fn fetch_errors() {
        let client = reqwest::Client::new();
        let res = client
            .get("https://api.mangadex.org/fail")
            .send()
            .await
            .unwrap()
            .json::<Err40X>()
            .await;

        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn get_athome_server() {
        let res = request("/at-home/server/af456519-3791-47c3-af8a-23ed894b5dd8")
            .send()
            .await
            .unwrap()
            .json::<AtHomeServer>()
            .await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn get_author() {
        let res = request("/author/c6c278e1-268b-4b7b-84ec-3289bd0c08f0")
            .send()
            .await
            .unwrap()
            .json::<Author>()
            .await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn get_author_list() {
        let res = request("/author")
            .send()
            .await
            .unwrap()
            .json::<AuthorList>()
            .await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn get_chapter() {
        let res = request("/chapter/af456519-3791-47c3-af8a-23ed894b5dd8")
            .send()
            .await
            .unwrap()
            .json::<Chapter>()
            .await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn get_chapter_list() {
        let res = request("/chapter")
            .send()
            .await
            .unwrap()
            .json::<ChapterList>()
            .await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn get_cover() {
        let manga_res = request("/manga/77bee52c-d2d6-44ad-a33a-1734c1fe696a")
            .send()
            .await
            .unwrap()
            .json::<Manga>()
            .await
            .unwrap();

        let cover_uuid = manga_res
            .data
            .relationships
            .into_iter()
            .find(|r| r.data_type == "cover_art")
            .unwrap()
            .uuid;

        let res = request(&format!("/cover/{cover_uuid}"))
            .send()
            .await
            .unwrap()
            .json::<Cover>()
            .await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn get_cover_list() {
        let res = request("/cover")
            .send()
            .await
            .unwrap()
            .json::<CoverArtList>()
            .await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn get_manga() {
        let res = request("/manga/77bee52c-d2d6-44ad-a33a-1734c1fe696a")
            .send()
            .await
            .unwrap()
            .json::<Manga>()
            .await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn get_manga_feed() {
        let res = request("/manga/77bee52c-d2d6-44ad-a33a-1734c1fe696a/feed")
            .send()
            .await
            .unwrap()
            .json::<MangaFeed>()
            .await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn get_manga_list() {
        let res = request("/manga")
            .send()
            .await
            .unwrap()
            .json::<MangaList>()
            .await;
        assert!(res.is_ok(), "{res:#?}")
    }
}
