use std::str::FromStr;

use async_trait::async_trait;
use url::Url;

use crate::{prelude::*, util::CLIENT, uuid_or_err};

#[async_trait]
impl ApiRoute for Client<Manga> {
    async fn get(mut self) -> Result<Self, ApiError> {
        let mut url = Url::from_str("https://api.mangadex.org").unwrap();
        let uuid = uuid_or_err!(self.uuid());
        url.set_path(format!("/manga/{uuid}").as_str());

        let res = CLIENT.get(url).send().await.unwrap();
        if res.status().is_success() {
            let json = res.json().await.unwrap();
            self.data = Some(json)
        } else {
            let json = res.json().await.unwrap();
            return Err(json);
        }

        Ok(self)
    }
}

#[async_trait]
impl Statistics for Client<Manga> {
    async fn get_statistics(mut self) -> Result<Self, ApiError> {
        Err(ApiError)
    }
}
