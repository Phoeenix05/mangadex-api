use mangadex_api::prelude::*;
use uuid::uuid;

#[tokio::test]
async fn manga() {
    let res = Client::<Manga>::new(uuid!("c288b108-5162-4065-aa3a-5857ec38c8d9"))
        .get()
        .await;
    assert!(res.is_ok(), "{:#?}", res.unwrap())
}

#[tokio::test]
async fn manga_feed() {
    let res = Client::<MangaFeed>::new(uuid!("c288b108-5162-4065-aa3a-5857ec38c8d9"))
        .get()
        .await;
    assert!(res.is_ok(), "{:#?}", res.unwrap())
}

#[tokio::test]
async fn manga_list() {
    let res = Client::<MangaList>::new().get().await;
    assert!(res.is_ok(), "{:#?}", res.unwrap())
}

#[tokio::test]
async fn athome_server() {
    let res = Client::<AtHomeServer>::new(uuid!("9e989fd3-72bf-4f99-a24c-114166be880a"))
        .get()
        .await;
    assert!(res.is_ok(), "{:#?}", res.unwrap())
}

// #[tokio::test]
// async fn author() {
//     let res = Client::<Author>::new(uuid!("c288b108-5162-4065-aa3a-5857ec38c8d9"))
//         .get()
//         .await;
//     assert!(res.is_ok(), "{:#?}", res.unwrap())
// }

// #[tokio::test]
// async fn author_list() {
//     let res = Client::<Manga>::new(uuid!("c288b108-5162-4065-aa3a-5857ec38c8d9"))
//         .get()
//         .await;
//     assert!(res.is_ok(), "{:#?}", res.unwrap())
// }

#[ignore = "not yet implemented"]
#[tokio::test]
async fn cover() {
    let res = Client::<Cover>::new(uuid!("c288b108-5162-4065-aa3a-5857ec38c8d9"))
        .get()
        .await;
    assert!(res.is_ok(), "{:#?}", res.unwrap())
}

#[ignore = "not yet implemented"]
#[tokio::test]
async fn cover_list() {
    let res = Client::<CoverList>::new().get().await;
    assert!(res.is_ok(), "{:#?}", res.unwrap())
}

#[tokio::test]
async fn chapter() {
    let res = Client::<Chapter>::new(uuid!("9e989fd3-72bf-4f99-a24c-114166be880a"))
        .get()
        .await;
    assert!(res.is_ok(), "{:#?}", res.unwrap())
}

#[tokio::test]
async fn chapter_list() {
    let res = Client::<ChapterList>::new().get().await;
    assert!(res.is_ok(), "{:#?}", res.unwrap())
}
