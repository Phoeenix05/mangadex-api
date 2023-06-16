pub mod get {
    use crate::json::statistics::Statistics;
    use crate::json::*;

    use serde::Deserialize;
    use url::Url;

    /// API route AtHomeServer [`/at-home/server/{chapterId}`](https://api.mangadex.org/at-home/server/af456519-3791-47c3-af8a-23ed894b5dd8)
    ///
    /// [MangaDex Docs](https://api.mangadex.org/docs/redoc.html#tag/AtHome/operation/get-at-home-server-chapterId)
    ///
    /// **NOTE:**
    /// `AtHomeServer` is defined here in `types.rs` as it is currently the only type
    /// that can't be defined with `MangaDexAPIResponse` struct.
    #[derive(Debug, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AtHomeServer {
        pub result: String,
        pub base_url: Url,
        pub chapter: Images,
    }

    #[derive(Debug, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Images {
        pub hash: String,
        pub data: Vec<String>,
        pub data_saver: Vec<String>,
    }

    /// API route Author [`/author/{authorId}`](http://api.mangadex.org/author/c6c278e1-268b-4b7b-84ec-3289bd0c08f0)
    ///
    /// [MangaDex Docs](https://api.mangadex.org/docs/redoc.html#tag/Author/operation/get-author-id)
    pub type Author = MangaDexAPIResponse<author::Author>;

    /// API route Author [`/author`](http://api.mangadex.org/author/)
    ///
    /// [MangaDex Docs](https://api.mangadex.org/docs/redoc.html#tag/Author/operation/get-author)
    pub type AuthorList = MangaDexAPIResponse<author::AuthorList>;

    /// API route Chapter [`/chapter/{chapterId}`](http://api.mangadex.org/chapter/af456519-3791-47c3-af8a-23ed894b5dd8)
    ///
    /// [MangaDex Docs](https://api.mangadex.org/docs/redoc.html#tag/Chapter)
    pub type Chapter = MangaDexAPIResponse<chapter::Chapter>;

    /// API route Chapter [`/chapter`](http://api.mangadex.org/chapter)
    ///
    /// [MangaDex Docs](https://api.mangadex.org/docs/redoc.html#tag/Chapter)
    pub type ChapterList = MangaDexAPIResponse<chapter::ChapterList>;

    /// API route Cover [`/cover/{coverId}`](https://api.mangadex.org/cover/1e711e37-7f19-482a-b643-fe6e89b61935).
    /// Requires cover id that can be obtained from the relationships of Manga
    ///
    /// [MangaDex Docs](https://api.mangadex.org/docs/redoc.html#tag/Cover/operation/get-cover-id)
    pub type Cover = MangaDexAPIResponse<cover::Cover>;

    /// API route Cover [`/cover`](https://api.mangadex.org/cover). Returns a list of manga covers.
    ///
    /// [MangaDex Docs](https://api.mangadex.org/docs/redoc.html#tag/Cover/operation/get-cover)
    pub type CoverArtList = MangaDexAPIResponse<cover::CoverArtList>;

    /// API route Manga [`/manga/{mangaId}`](https://api.mangadex.org/manga/77bee52c-d2d6-44ad-a33a-1734c1fe696a).
    ///
    /// This API route returns full info about the specified manga.
    ///
    /// [MangaDex Docs](https://api.mangadex.org/docs/redoc.html#tag/Manga/operation/get-manga-id)
    pub type Manga = MangaDexAPIResponse<manga::Manga>;

    /// API route MangaFeed [`/manga/{mangaId}/feed`](https://api.mangadex.org/manga/77bee52c-d2d6-44ad-a33a-1734c1fe696a)
    ///
    /// This API route returns a list consisting of specified manga's chapters. The amount of chapters returned is 500 at
    /// maximum.
    ///
    /// [MangaDex Docs](https://api.mangadex.org/docs/redoc.html#tag/Manga/operation/get-manga-id-feed)
    pub type MangaFeed = MangaDexAPIResponse<serde_json::Value>;

    /// API route MangaList [`/manga`](https://api.mangadex.org/manga/)
    ///
    /// This API route returns a list of mangas. The amount of mangas returned is 500 at
    /// maximum.
    ///
    /// [MangaDex Docs](https://api.mangadex.org/docs/redoc.html#tag/Manga/operation/get-search-manga)
    pub type MangaList = MangaDexAPIResponse<manga::MangaList>;

    /// API route Statistics [`/statistics/chapter/{uuid}`](https://api.mangadex.org/statistics/chapter/af456519-3791-47c3-af8a-23ed894b5dd8)
    ///
    /// This route returns the statistics of the given chapter
    ///
    /// [MangaDex Docs](https://api.mangadex.org/docs/redoc.html#tag/Statistics/operation/get-statistics-chapter-uuid)
    pub type ChapterStatistics = Statistics;
    // //// Thinking of a way to implement `Endpoint` for these due to
    // //// conflicting implementations
    // pub type ChapterListStatistics = Statistics;
    // pub type ScanlationGroupStatistics = Statistics;
    // pub type GroupListStatistics = Statistics;
    // pub type MangaStatistics = Statistics;
    // pub type MangaListStatistics = Statistics;
}
