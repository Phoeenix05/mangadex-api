use serde::{Deserialize, Serialize};

use crate::{prelude::*, unwrap_api_results, util::CLIENT, uuid_or_err};

////////////////////////////////////////////////////////////////
/// Chapter
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chapter;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChapterStatistics;

impl Client<Chapter> {
    pub fn new() -> Self {
        todo!()
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

impl Client<ChapterStatistics> {
    pub async fn get_statistics(self) -> Result<ChapterStatistics, ClientError> {
        todo!()
    }
}
////////////////////////////////////////////////////////////////
/// Manga list
////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChapterList;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChapterListStatistics;

impl Client<ChapterList> {
    pub fn new() -> Self {
        todo!()
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

impl Client<ChapterListStatistics> {
    pub fn new() -> Self {
        todo!()
    }

    pub async fn get_statistics(self) -> Result<ChapterListStatistics, ClientError> {
        todo!()
    }
}
