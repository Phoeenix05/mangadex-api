mod get {
    use mangadex_api::types::get::{Cover, CoverArtList};
    use mangadex_api::wrapper::Endpoint;
    use uuid::uuid;

    #[tokio::test]
    async fn cover() {
        let res = Cover::get_uuid(uuid!("77bee52c-d2d6-44ad-a33a-1734c1fe696a")).await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn cover_art_list() {
        let res = CoverArtList::get().await;
        assert!(res.is_ok(), "{res:#?}")
    }
}

mod post {}

mod put {}

mod delete {}
