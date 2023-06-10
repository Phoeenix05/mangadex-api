use std::str::FromStr;

use reqwest::{Client, Url};
use uuid::Uuid;

use crate::json::*;

const BASE: &str = "https://api.mangadex.org";

fn url(path: String, query: Option<&str>) -> Url {
    let mut url = Url::from_str(BASE).unwrap();
    url.set_path(path.as_str());
    url.set_query(query);
    url
}

async fn fetch<T>(url: Url) -> Result<T, Err40X>
where
    T: serde::de::DeserializeOwned,
{
    let client = Client::new();
    let res = client.get(url).send().await.unwrap();
    if res.status().is_success() {
        let json = res.json::<T>().await.unwrap();
        Ok(json)
    } else {
        let json = res.json::<Err40X>().await.unwrap();
        Err(json)
    }
}

pub async fn get_athome_server(uuid: Uuid) -> Result<AtHomeServer, Err40X> {
    let url = url(format!("/at-home/server/{uuid}"), None);
    let res = fetch::<AtHomeServer>(url).await;
    res
}

pub async fn get_manga(uuid: Uuid) -> Result<Manga, Err40X> {
    let url = url(format!("/manga/{uuid}"), None);
    let res = fetch::<Manga>(url).await;
    res
}

pub async fn get_manga_feed(uuid: Uuid) -> Result<MangaFeed, Err40X> {
    let url = url(format!("/manga/{uuid}/feed"), None);
    let res = fetch::<MangaFeed>(url).await;
    res
}

pub async fn get_manga_list() -> Result<MangaList, Err40X> {
    let url = url("/manga".into(), None);
    let res = fetch::<MangaList>(url).await;
    res
}

pub async fn get_chapter(uuid: Uuid) -> Result<Chapter, Err40X> {
    let url = url(format!("/chapter/{uuid}"), None);
    let res = fetch::<Chapter>(url).await;
    res
}

pub async fn get_chapter_list() -> Result<ChapterList, Err40X> {
    let url = url("/chapter".into(), None);
    let res = fetch::<ChapterList>(url).await;
    res
}

pub async fn get_author(uuid: Uuid) -> Result<Author, Err40X> {
    let url = url(format!("/author/{uuid}"), None);
    let res = fetch::<Author>(url).await;
    res
}

pub async fn get_author_list() -> Result<AuthorList, Err40X> {
    let url = url("/author".into(), None);
    let res = fetch::<AuthorList>(url).await;
    res
}

pub async fn get_cover(uuid: Uuid) -> Result<Cover, Err40X> {
    let url = url(format!("/cover/{uuid}"), None);
    let res = fetch::<Cover>(url).await;
    res
}

pub async fn get_cover_list() -> Result<CoverArtList, Err40X> {
    let url = url("/cover".into(), None);
    let res = fetch::<CoverArtList>(url).await;
    res
}
