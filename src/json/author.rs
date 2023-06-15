use serde::Deserialize;
use url::Url;

use super::Data;

pub type Author = Data<Attributes>;
pub type AuthorList = Vec<Data<Attributes>>;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub name: String,
    pub image_url: Option<String>,
    pub biography: serde_json::Value,
    pub twitter: Option<Url>,
    pub pixiv: Option<Url>,
    pub melon_book: Option<Url>,
    pub fan_box: Option<Url>,
    pub booth: Option<Url>,
    pub nico_video: Option<Url>,
    pub skeb: Option<Url>,
    pub fantia: Option<Url>,
    pub tumblr: Option<Url>,
    pub youtube: Option<Url>,
    pub weibo: Option<Url>,
    pub naver: Option<Url>,
    pub website: Option<Url>,
    pub version: u64,
    pub created_at: String,
    pub updated_at: String,
}
