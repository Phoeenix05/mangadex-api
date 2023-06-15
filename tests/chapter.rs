mod get {
    use mangadex_api::prelude::*;

    #[tokio::test]
    async fn chapter() {
        let res = Chapter::get_uuid(uuid!("af456519-3791-47c3-af8a-23ed894b5dd8")).await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn chapter_list() {
        let res = ChapterList::get().await;
        assert!(res.is_ok(), "{res:#?}")
    }
}

mod post {}

mod put {}

mod delete {}
