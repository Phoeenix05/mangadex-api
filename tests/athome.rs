mod get {
    use mangadex_api::prelude::*;
    use uuid::uuid;

    #[tokio::test]
    async fn athome_server() {
        let res = AtHomeServer::get_uuid(uuid!("af456519-3791-47c3-af8a-23ed894b5dd8")).await;
        assert!(res.is_ok(), "{res:#?}")
    }
}
