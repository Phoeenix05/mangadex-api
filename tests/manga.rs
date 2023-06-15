mod get {
    use mangadex_api::prelude::*;

    #[tokio::test]
    async fn manga_list() {
        let res = MangaList::get().await;
        assert!(res.is_ok(), "{res:#?}")
    }

    // #[tokio::test]
    // async fn volumes_and_chapters() {}

    #[tokio::test]
    async fn manga() {
        let res = Manga::get_uuid(uuid!("77bee52c-d2d6-44ad-a33a-1734c1fe696a")).await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn manga_feed() {
        let res = MangaFeed::get_uuid(uuid!("77bee52c-d2d6-44ad-a33a-1734c1fe696a")).await;
        assert!(res.is_ok(), "{res:#?}")
    }

    // #[tokio::test]
    // async fn random_manga() {}

    // #[tokio::test]
    // async fn tag_list() {}

    // #[tokio::test]
    // async fn all_manga_reading_status_user() {}

    // #[tokio::test]
    // async fn manga_reading_status() {}

    // #[tokio::test]
    // async fn specific_manga_draft() {}

    // #[tokio::test]
    // async fn manga_drafts_list() {}

    // #[tokio::test]
    // async fn manga_relation_list() {}
}

mod post {
    // #[tokio::test]
    // async fn follow_manga() {}

    // #[tokio::test]
    // async fn update_manga_reading_status() {}

    // #[tokio::test]
    // async fn submit_manga_draft() {}

    // #[tokio::test]
    // async fn create_manga_relation() {}
}

mod put {
    // #[tokio::test]
    // async fn update_manga() {}
}

mod delete {
    // #[tokio::test]
    // async fn delete_manga() {}

    // #[tokio::test]
    // async fn unfollow_manga() {}

    // #[tokio::test]
    // async fn delete_manga_relation() {}
}
