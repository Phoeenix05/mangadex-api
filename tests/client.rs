use mangadex_api::prelude::*;

#[tokio::test]
async fn create_client() {
    let res = Client::<Manga>::new()
        .set_uuid(uuid!("c288b108-5162-4065-aa3a-5857ec38c8d9"))
        .get()
        .await;
    assert!(res.is_ok(), "{:#?}", res.unwrap_err())
}
