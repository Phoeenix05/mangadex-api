use std::str::FromStr;

use reqwest::Url;
use uuid::Uuid;

use crate::{
    config::{MangaFeedConfig, MangaListConfig},
    json::{
        at_home::AtHomeServer,
        manga::{MangaFeed, MangaList},
        Error, Result,
    },
};

pub async fn get_athome_server(chapter_id: Uuid) -> Result<AtHomeServer> {
    let client = reqwest::Client::new();
    let raw_url = format!("https://api.mangadex.org/at-home/server/{chapter_id}");
    let url = Url::from_str(&raw_url).unwrap();

    let res = client.get(url).send().await.unwrap();

    if res.status().is_success() {
        let json: AtHomeServer = res.json().await.unwrap();
        Result::Ok(json)
    } else {
        let json: Error = res.json().await.unwrap();
        Result::Err(json)
    }
}

pub async fn get_manga_list(_config: Option<MangaListConfig>) -> Result<MangaList> {
    let client = reqwest::Client::new();
    let raw_url = format!("https://api.mangadex.org/manga");
    let url = Url::from_str(&raw_url).unwrap();

    let res = client.get(url).send().await.unwrap();

    if res.status().is_success() {
        let json: MangaList = res.json().await.unwrap();
        Result::Ok(json)
    } else {
        let json: Error = res.json().await.unwrap();
        Result::Err(json)
    }
}

pub async fn get_manga_feed(manga_id: Uuid, _config: Option<MangaFeedConfig>) -> Result<MangaFeed> {
    let client = reqwest::Client::new();
    let raw_url = format!("https://api.mangadex.org/manga/{manga_id}/feed");
    let url = Url::from_str(&raw_url).unwrap();

    let res = client.get(url).send().await.unwrap();

    if res.status().is_success() {
        let json: MangaFeed = res.json().await.unwrap();
        Result::Ok(json)
    } else {
        let json: Error = res.json().await.unwrap();
        Result::Err(json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn athome_server() {
        let uuid = Uuid::from_str("029b7226-5673-41d2-9ae6-09793f200bd9").unwrap();
        let data = get_athome_server(uuid).await;
        match data {
            Result::Err(err) => panic!("{:#?}", err),
            _ => (),
        }
    }

    #[tokio::test]
    async fn manga_list() {
        let data = get_manga_list(None).await;
        match data {
            Result::Err(err) => panic!("{:#?}", err),
            _ => (),
        }
    }

    #[tokio::test]
    async fn manga_feed() {
        let test_cases = vec![
            "e6eb6bd0-0285-4fac-a6da-9bc4234ac1bb",
            "b35163ef-efbf-4cb9-bf97-0acc89200455",
            "bd6d0982-0091-4945-ad70-c028ed3c0917",
            "c7421641-dc50-4a3c-80a5-5cdcb2cae890",
            "1e3feba8-0c4d-4465-b3a3-e9a2a4451bd1",
        ];

        for case in test_cases.into_iter() {
            let data = get_manga_feed(Uuid::from_str(case).unwrap(), None).await;
            match data {
                Result::Err(err) => panic!("{:#?}", err),
                _ => (),
            }
        }
    }
}
