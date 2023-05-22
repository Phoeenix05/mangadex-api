mod prelude {
    use super::*;

    pub use config::*;
    pub use wrapper::*;
}

pub mod config;
pub mod json;
pub mod wrapper;

#[cfg(test)]
mod tests {
    use reqwest::{Response, Result};

    pub async fn fetch(url: &str) -> Result<Response> {
        let client: reqwest::Client = reqwest::Client::new();
        let res = client.get(url).send().await;
        res
    }
}
