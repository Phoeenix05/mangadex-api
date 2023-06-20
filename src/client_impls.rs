use std::str::FromStr;

use crate::{prelude::*, unwrap_api_results, util::CLIENT, uuid_or_err};
use async_trait::async_trait;
use url::Url;

#[async_trait]
impl ApiRoute<AtHomeServer> for Client<AtHomeServer> {
    async fn get(mut self) -> Result<AtHomeServer, ClientError> {
        let uuid = uuid_or_err!(self.uuid());

        let mut url = Url::from_str("https://api.mangadex.org").unwrap();
        url.set_path(format!("/at-home/server/{uuid}").as_str());

        let res = CLIENT.get(url).send().await.unwrap();
        unwrap_api_results!(res)
    }
}

#[async_trait]
impl ApiRoute<Cover> for Client<Cover> {
    async fn get(mut self) -> Result<Cover, ClientError> {
        todo!()
    }
}

#[async_trait]
impl ApiRoute<CoverList> for Client<CoverList> {
    async fn get(mut self) -> Result<CoverList, ClientError> {
        todo!()
    }
}

#[async_trait]
impl ApiRoute<Chapter> for Client<Chapter> {
    async fn get(mut self) -> Result<Chapter, ClientError> {
        let uuid = uuid_or_err!(self.uuid());

        let mut url = Url::from_str("https://api.mangadex.org").unwrap();
        url.set_path(format!("/chapter/{uuid}").as_str());

        let res = CLIENT.get(url).send().await.unwrap();
        unwrap_api_results!(res)
    }
}

#[async_trait]
impl Statistics<ChapterStatistics> for Client<ChapterStatistics> {
    async fn get_statistics(mut self) -> Result<ChapterStatistics, ClientError> {
        todo!()
    }
}

#[async_trait]
impl ApiRoute<ChapterList> for Client<ChapterList> {
    async fn get(mut self) -> Result<ChapterList, ClientError> {
        let url = Url::from_str("https://api.mangadex.org/chapter").unwrap();

        let res = CLIENT.get(url).send().await.unwrap();
        unwrap_api_results!(res)
    }
}

#[async_trait]
impl Statistics<ChapterListStatistics> for Client<ChapterListStatistics> {
    async fn get_statistics(mut self) -> Result<ChapterListStatistics, ClientError> {
        todo!()
    }
}

#[async_trait]
impl ApiRoute<Manga> for Client<Manga> {
    async fn get(mut self) -> Result<Manga, ClientError> {
        let uuid = uuid_or_err!(self.uuid());

        let mut url = Url::from_str("https://api.mangadex.org").unwrap();
        url.set_path(format!("/manga/{uuid}").as_str());

        let res = CLIENT.get(url).send().await.unwrap();
        unwrap_api_results!(res)
    }
}

#[async_trait]
impl Statistics<Manga> for Client<Manga> {
    async fn get_statistics(mut self) -> Result<Manga, ClientError> {
        let err = ClientError {
            msg: "Not implemented".to_string(),
            api_msg: None,
        };
        Err(err)
    }
}

#[async_trait]
impl ApiRoute<MangaFeed> for Client<MangaFeed> {
    async fn get(mut self) -> Result<MangaFeed, ClientError> {
        let uuid = uuid_or_err!(self.uuid());

        let mut url = Url::from_str("https://api.mangadex.org").unwrap();
        url.set_path(format!("/manga/{uuid}/feed").as_str());

        let res = CLIENT.get(url).send().await.unwrap();
        unwrap_api_results!(res)
    }
}

#[async_trait]
impl ApiRoute<MangaList> for Client<MangaList> {
    async fn get(mut self) -> Result<MangaList, ClientError> {
        let url = Url::from_str("https://api.mangadex.org/manga").unwrap();

        let res = CLIENT.get(url).send().await.unwrap();
        unwrap_api_results!(res)
    }
}

#[async_trait]
impl Statistics<MangaListStatistics> for Client<MangaListStatistics> {
    async fn get_statistics(mut self) -> Result<MangaListStatistics, ClientError> {
        todo!()
    }
}
