pub mod get {
    use crate::json::MangaDexAPIResponse;
    use crate::json::*;

    pub use crate::json::athome::AtHomeServer;
    pub type Author = MangaDexAPIResponse<serde_json::Value>;
    pub type Chapter = MangaDexAPIResponse<serde_json::Value>;
    pub type Cover = MangaDexAPIResponse<serde_json::Value>;

    pub type Manga = MangaDexAPIResponse<manga::Manga>;
    pub type MangaFeed = MangaDexAPIResponse<serde_json::Value>;
    pub type MangaList = MangaDexAPIResponse<manga::MangaList>;
}
