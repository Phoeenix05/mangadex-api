use std::str::FromStr;

use reqwest::Url;
use uuid::Uuid;

use crate::json::{AtHomeServer, Err40X, MangaFeed, MangaList};
use std::result::Result;

pub async fn get_athome_server(chapter_id: Uuid) -> Result<AtHomeServer, Err40X> {
    let client = reqwest::Client::new();
    let raw_url = format!("https://api.mangadex.org/at-home/server/{chapter_id}");
    let url = Url::from_str(&raw_url).unwrap();

    let res = client.get(url).send().await.unwrap();

    if res.status().is_success() {
        let json: AtHomeServer = res.json().await.unwrap();
        Result::Ok(json)
    } else {
        let json: Err40X = res.json().await.unwrap();
        Result::Err(json)
    }
}

pub async fn get_manga_list() -> Result<MangaList, Err40X> {
    let client = reqwest::Client::new();
    let raw_url = format!("https://api.mangadex.org/manga");
    let url = Url::from_str(&raw_url).unwrap();

    let res = client.get(url).send().await.unwrap();

    if res.status().is_success() {
        let json: MangaList = res.json().await.unwrap();
        Result::Ok(json)
    } else {
        let json: Err40X = res.json().await.unwrap();
        Result::Err(json)
    }
}

pub async fn get_manga_feed(manga_id: Uuid) -> Result<MangaFeed, Err40X> {
    let client = reqwest::Client::new();
    let raw_url = format!("https://api.mangadex.org/manga/{manga_id}/feed");
    let url = Url::from_str(&raw_url).unwrap();

    let res = client.get(url).send().await.unwrap();

    if res.status().is_success() {
        let json: MangaFeed = res.json().await.unwrap();
        Result::Ok(json)
    } else {
        let json: Err40X = res.json().await.unwrap();
        Result::Err(json)
    }
}
