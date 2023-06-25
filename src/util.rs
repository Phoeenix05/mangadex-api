// #![cfg(feature = "wrapper")]

#[macro_export]
macro_rules! uuid_or_err {
    ($opt:expr) => {{
        let uuid: &Option<uuid::Uuid> = $opt;
        if uuid.is_some() {
            Ok(uuid.unwrap())
        } else {
            let err = crate::client::ClientError {
                msg: "UUID required".into(),
                api_msg: None,
            };
            Err(err)
        }
    }};
}

#[macro_export]
macro_rules! unwrap_api_results {
    ($res:expr) => {{
        let res: reqwest::Response = $res;
        if res.status().is_success() {
            let json = res.json().await.unwrap();
            Ok(json)
        } else {
            let json: $crate::json::ApiError = res.json().await.unwrap();
            let err = $crate::client::ClientError {
                msg: "Failed to fetch data from API".into(),
                api_msg: Some(json),
            };
            return Err(err);
        }
    }};
}

pub mod client {
    pub fn construct_url(path: String, query: Option<&str>) -> url::Url {
        use std::str::FromStr;
        let mut url = url::Url::from_str("https://api.mangadex.org").unwrap();
        url.set_path(path.as_str());
        url.set_query(query);
        url
    }
}
