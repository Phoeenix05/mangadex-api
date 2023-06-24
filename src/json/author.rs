use serde::{Deserialize, Serialize};

use super::Data;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Author {
    result: String,
    response: String,
    data: AuthorAttributes,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthorList {
    result: String,
    response: String,
    data: Vec<Data<AuthorAttributes>>,
    limit: u64,
    offset: u64,
    total: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AuthorAttributes {
    name: String,
    image_url: Option<String>,
    biography: serde_json::Value,
    twitter: Option<String>,
    pixiv: Option<String>,
    melon_book: Option<String>,
    fan_box: Option<String>,
    booth: Option<String>,
    nico_video: Option<String>,
    skeb: Option<String>,
    fantia: Option<String>,
    tumblr: Option<String>,
    youtube: Option<String>,
    weibo: Option<String>,
    naver: Option<String>,
    website: Option<String>,
    version: u64,
    created_at: Option<String>,
    updated_at: Option<String>,
}
