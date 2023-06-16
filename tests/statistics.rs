mod get {
    use mangadex_api::{prelude::ChapterStatistics, wrapper::Endpoint};
    use uuid::uuid;

    #[tokio::test]
    async fn chapter_statistics() {
        let res = ChapterStatistics::get_uuid(uuid!("af456519-3791-47c3-af8a-23ed894b5dd8")).await;
        assert!(res.is_ok(), "{res:#?}")
    }

    // #[tokio::test]
    // async fn chapter_list_statistics() {}
}
