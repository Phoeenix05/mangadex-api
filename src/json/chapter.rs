use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{prelude::*, unwrap_api_results, util::CLIENT, uuid_or_err};

use super::Data;

////////////////////////////////////////////////////////////////
/// Chapter
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chapter {
    result: String,
    response: String,
    data: Data<ChapterAttributes>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ChapterAttributes {
    title: Option<String>,
    volume: Option<String>,
    chapter: String,
    pages: u64,
    translated_language: String,
    uploader: Option<uuid::Uuid>,
    external_url: Option<String>,
    version: u64,
    created_at: String,
    updated_at: String,
    publish_at: String,
    readable_at: String,
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct ChapterStatistics;

impl Client<Chapter> {
    pub fn new(uuid: Uuid) -> Self {
        Self {
            uuid: Some(uuid),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<Chapter, ClientError> {
        let uuid = uuid_or_err!(self.get_uuid()).unwrap();
        let res = CLIENT
            .get(ClientUtil::construct_url(format!("/chapter/{uuid}"), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

// impl Client<ChapterStatistics> {
//     pub async fn get_statistics(self) -> Result<ChapterStatistics, ClientError> {
//         todo!()
//     }
// }

////////////////////////////////////////////////////////////////
/// Manga list
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChapterList {
    result: String,
    response: String,
    data: Vec<Data<ChapterAttributes>>,
    limit: u64,
    offset: u64,
    total: u64,
}

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct ChapterListStatistics;

impl Client<ChapterList> {
    pub fn new() -> Self {
        Self {
            uuid: None,
            // url: todo!(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub async fn get(self) -> Result<ChapterList, ClientError> {
        let res = CLIENT
            .get(ClientUtil::construct_url("/chapter".into(), None))
            .send()
            .await
            .unwrap();
        unwrap_api_results!(res)
    }
}

// impl Client<ChapterListStatistics> {
//     pub fn new() -> Self {
//         todo!()
//     }

//     pub async fn get_statistics(self) -> Result<ChapterListStatistics, ClientError> {
//         todo!()
//     }
// }
