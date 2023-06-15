mod get {
    use mangadex_api::types::get::{Author, AuthorList};
    use mangadex_api::wrapper::Endpoint;
    use uuid::uuid;

    #[tokio::test]
    async fn author() {
        let res = Author::get_uuid(uuid!("c6c278e1-268b-4b7b-84ec-3289bd0c08f0")).await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn author_list() {
        let res = AuthorList::get().await;
        assert!(res.is_ok(), "{res:#?}")
    }
}

mod post {
    // #[tokio::test]
    // async fn create_author() {}
}

mod put {
    // #[tokio::test]
    // async fn update_author() {}
}

mod delete {
    // #[tokio::test]
    // async fn delete_author() {}
}
