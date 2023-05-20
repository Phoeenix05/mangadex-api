use super::*;

/// Provides types for API endpoint `/manga`
pub struct List {
    pub result: String,
    pub response: String,
    pub data: Vec<Manga>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
    pub total: Option<u64>,
}
