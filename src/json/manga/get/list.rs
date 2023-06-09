use crate::json::{Data, Response};

use super::Attributes;

pub type MangaList = Response<Vec<Data<Attributes>>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fetch;

    #[tokio::test]
    async fn test() {
        let res = fetch("https://api.mangadex.org/manga")
            .await
            .unwrap()
            .json::<MangaList>()
            .await;
        assert!(res.is_ok(), "{res:#?}")
    }
}
