mod get {
    use mangadex_api::types::get::AtHomeServer;
    use reqwest::Client;

    #[tokio::test]
    async fn athome_server() {
        let client = Client::new();
        let res = client
            .get("https://api.mangadex.org/at-home/server/af456519-3791-47c3-af8a-23ed894b5dd8")
            .send()
            .await
            .unwrap()
            .json::<AtHomeServer>()
            .await;
        assert!(res.is_ok(), "{res:#?}")
    }
}
