mod get {
    use mangadex_api::prelude::*;

    #[tokio::test]
    async fn chapter_statistics() {
        let res = Chapter::get_statistics(uuid!("af456519-3791-47c3-af8a-23ed894b5dd8")).await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn chapter_list_statistics() {
        let res = ChapterList::get().await;
        assert!(res.is_ok(), "{res:#?}");

        let uuid_list = res
            .unwrap()
            .data
            .into_iter()
            .map(|chapter| chapter.id)
            .collect::<Vec<_>>();
        let res = ChapterList::get_statistics_list(uuid_list).await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn manga_statistics() {
        let res = Manga::get_statistics(uuid!("77bee52c-d2d6-44ad-a33a-1734c1fe696a")).await;
        assert!(res.is_ok(), "{res:#?}")
    }

    #[tokio::test]
    async fn manga_list_statistics() {
        let res = MangaList::get().await;
        assert!(res.is_ok(), "{res:#?}");

        let uuid_list = res
            .unwrap()
            .data
            .into_iter()
            .map(|chapter| chapter.id)
            .collect::<Vec<_>>();
        let res = MangaList::get_statistics_list(uuid_list).await;
        assert!(res.is_ok(), "{res:#?}")
    }
}
