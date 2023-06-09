mod list;

pub use list::AuthorList;

use serde::{Deserialize, Serialize};

use crate::json::{Data, Response};

pub type Author = Response<Data<Attributes>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub name: String,
    pub image_url: Option<String>,
    pub biography: serde_json::Value,
    pub twitter: Option<String>,
    pub pixiv: Option<String>,
    pub melon_book: Option<String>,
    pub fan_box: Option<String>,
    pub booth: Option<String>,
    pub nico_video: Option<String>,
    pub skeb: Option<String>,
    pub fantia: Option<String>,
    pub tumblr: Option<String>,
    pub youtube: Option<String>,
    pub weibo: Option<String>,
    pub naver: Option<String>,
    pub website: Option<String>,
    pub version: u64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fetch;

    #[tokio::test]
    async fn test() {
        let res = fetch("https://api.mangadex.org/author/d7f4dd59-4ffc-44df-b938-173a6140ac55")
            .await
            .unwrap()
            .json::<Author>()
            .await;

        assert!(res.is_ok(), "{res:#?}")
    }
}
