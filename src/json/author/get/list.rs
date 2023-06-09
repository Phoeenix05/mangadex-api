use crate::{json::Data, json::Response};

use super::Attributes;

pub type AuthorList = Response<Vec<Data<Attributes>>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fetch;

    #[tokio::test]
    async fn test() {
        let res = fetch("https://api.mangadex.org/author")
            .await
            .unwrap()
            .json::<AuthorList>()
            .await;

        assert!(res.is_ok(), "{res:#?}")
    }
}
