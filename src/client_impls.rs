use std::str::FromStr;

use crate::{prelude::*, unwrap_api_results, util::CLIENT, uuid_or_err};
use async_trait::async_trait;
use url::Url;

#[async_trait]
impl ApiRoute for Client<AtHomeServer> {
    async fn get(mut self) -> Result<Self, ClientError> {
        let uuid = uuid_or_err!(self.uuid());

        let mut url = Url::from_str("https://api.mangadex.org").unwrap();
        url.set_path(format!("/at-home/server/{uuid}").as_str());

        let res = CLIENT.get(url).send().await.unwrap();
        self.data = unwrap_api_results!(res);
        Ok(self)
    }
}

#[async_trait]
impl ApiRoute for Client<Cover> {
    async fn get(mut self) -> Result<Self, ClientError> {
        todo!()
    }
}

#[async_trait]
impl ApiRoute for Client<CoverList> {
    async fn get(mut self) -> Result<Self, ClientError> {
        todo!()
    }
}

#[async_trait]
impl ApiRoute for Client<Chapter> {
    async fn get(mut self) -> Result<Self, ClientError> {
        let uuid = uuid_or_err!(self.uuid());

        let mut url = Url::from_str("https://api.mangadex.org").unwrap();
        url.set_path(format!("/chapter/{uuid}").as_str());

        let res = CLIENT.get(url).send().await.unwrap();
        self.data = unwrap_api_results!(res);
        Ok(self)
    }
}

#[async_trait]
impl Statistics for Client<ChapterStatistics> {
    async fn get_statistics(mut self) -> Result<Self, ClientError> {
        todo!()
    }
}

#[async_trait]
impl ApiRoute for Client<ChapterList> {
    async fn get(mut self) -> Result<Self, ClientError> {
        let url = Url::from_str("https://api.mangadex.org/chapter").unwrap();

        let res = CLIENT.get(url).send().await.unwrap();
        self.data = unwrap_api_results!(res);
        Ok(self)
    }
}

#[async_trait]
impl Statistics for Client<ChapterListStatistics> {
    async fn get_statistics(mut self) -> Result<Self, ClientError> {
        todo!()
    }
}

#[async_trait]
impl ApiRoute for Client<Manga> {
    async fn get(mut self) -> Result<Self, ClientError> {
        let uuid = uuid_or_err!(self.uuid());

        let mut url = Url::from_str("https://api.mangadex.org").unwrap();
        url.set_path(format!("/manga/{uuid}").as_str());

        let res = CLIENT.get(url).send().await.unwrap();
        self.data = unwrap_api_results!(res);
        Ok(self)
    }
}

#[async_trait]
impl Statistics for Client<Manga> {
    async fn get_statistics(mut self) -> Result<Self, ClientError> {
        let err = ClientError {
            msg: "Not implemented".to_string(),
            api_msg: None,
        };
        Err(err)
    }
}

#[async_trait]
impl ApiRoute for Client<MangaFeed> {
    async fn get(mut self) -> Result<Self, ClientError> {
        let uuid = uuid_or_err!(self.uuid());

        let mut url = Url::from_str("https://api.mangadex.org").unwrap();
        url.set_path(format!("/manga/{uuid}/feed").as_str());

        let res = CLIENT.get(url).send().await.unwrap();
        self.data = unwrap_api_results!(res);
        Ok(self)
    }
}

#[async_trait]
impl ApiRoute for Client<MangaList> {
    async fn get(mut self) -> Result<Self, ClientError> {
        let url = Url::from_str("https://api.mangadex.org/manga").unwrap();

        let res = CLIENT.get(url).send().await.unwrap();
        self.data = unwrap_api_results!(res);
        Ok(self)
    }
}

#[async_trait]
impl Statistics for Client<MangaListStatistics> {
    async fn get_statistics(mut self) -> Result<Self, ClientError> {
        todo!()
    }
}
