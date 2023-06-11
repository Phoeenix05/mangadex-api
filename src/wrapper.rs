use crate::json::Err40X;

#[async_trait::async_trait]
#[allow(unused_variables)]
pub trait Endpoint<T> {
    async fn get(uuid: Option<uuid::Uuid>) -> Result<T, Err40X>;
}

//     /// Endpoint for `/at-home/server/{chapterId}`
//     AtHomeServer(Uuid),

//     /// Endpoint for `/author/{id}`
//     Author(Uuid),
//     /// Endpoint for `/author`
//     AuthorList,

//     /// Endpoint for `/chapter/{id}`
//     Chapter(Uuid),
//     /// Endpoint for `/chapter`
//     ChapterList,

//     /// Endpoint for `/cover/{mangaOrCoverId}`.
//     ///
//     /// NOTE: API documentation says `manga` or `cover` id but from my testing
//     /// this endpoint requires the `cover` id. **This process is automatic no
//     /// need to first fetch manga data and then the cover.**
//     Cover(Uuid),
//     /// Endpoint for `/cover`
//     CoverArtList,

//     /// Endpoint for `/manga/{id}`
//     Manga(Uuid),
//     /// Endpoint for `/manga/{id}/feed`. This endpoint returns a list of a manga's chapters
//     MangaFeed(Uuid),
//     /// Endpoint for `/manga`
//     MangaList,
