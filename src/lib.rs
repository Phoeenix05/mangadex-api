mod at_home;
mod auth;
mod author;
mod captcha;
mod chapter;
mod cover;
mod customlist;
mod feed;
mod follows;
mod forums;
mod infrastructure;
mod legacy;
mod manga;
mod rating;
mod readmarker;
mod report;
mod scanlationgroup;
mod settings;
mod upload;
mod user;

pub use manga::List;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Err400 {
    pub result: String,
    pub errors: Vec<Error>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub id: String,
    pub status: u64,
    pub title: String,
    pub detail: String,
}
