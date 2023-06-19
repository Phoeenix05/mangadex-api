use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiError;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AtHomeServer;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cover;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CoverList;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chapter;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChapterStatistics;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChapterList;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChapterListStatistics;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manga {
    result: String,
    response: String,
    data: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaStatistics;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaFeed;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaList;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaListStatistics;
