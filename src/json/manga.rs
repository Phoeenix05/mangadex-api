use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{prelude::*, unwrap_api_results, util::CLIENT, uuid_or_err};

////////////////////////////////////////////////////////////////
/// Manga
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Manga {
    result: String,
    response: String,
    data: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaStatistics;

impl Client<Manga> {
    pub fn new() -> Self {
        todo!()
    }

    pub async fn get(self) -> Result<Manga, ClientError> {
        let uuid = uuid_or_err!(self.get_uuid()).unwrap();

        let mut url = Url::from_str("https://api.mangadex.org").unwrap();
        url.set_path(format!("/manga/{uuid}").as_str());

        let res = CLIENT.get(url).send().await.unwrap();
        unwrap_api_results!(res)
    }
}

impl Client<Manga> {
    pub async fn get_statistics(self) -> Result<Manga, ClientError> {
        let err = ClientError {
            msg: "Not implemented".to_string(),
            api_msg: None,
        };
        Err(err)
    }
}

////////////////////////////////////////////////////////////////
/// Manga feed
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaFeed;

impl Client<MangaFeed> {
    pub fn new() -> Self {
        todo!()
    }

    pub async fn get(self) -> Result<MangaFeed, ClientError> {
        let uuid = uuid_or_err!(self.get_uuid()).unwrap();

        let mut url = Url::from_str("https://api.mangadex.org").unwrap();
        url.set_path(format!("/manga/{uuid}/feed").as_str());

        let res = CLIENT.get(url).send().await.unwrap();
        unwrap_api_results!(res)
    }
}

////////////////////////////////////////////////////////////////
/// Manga list
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaList;

impl Client<MangaList> {
    pub fn new() -> Self {
        todo!()
    }

    pub async fn get(self) -> Result<MangaList, ClientError> {
        let url = Url::from_str("https://api.mangadex.org/manga").unwrap();

        let res = CLIENT.get(url).send().await.unwrap();
        unwrap_api_results!(res)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MangaListStatistics;

impl Client<MangaListStatistics> {
    pub fn new() -> Self {
        todo!()
    }

    pub async fn get_statistics(self) -> Result<MangaListStatistics, ClientError> {
        todo!()
    }
}
