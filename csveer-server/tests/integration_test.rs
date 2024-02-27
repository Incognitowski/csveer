mod common;

#[tokio::test]
async fn test_should_create_context() {
    common::reset_db().await;
    let (addr, listener) = common::create_listener().await;
    common::spawn_server(listener).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/context", addr))
        .header("Content-Type", "application/json")
        .body(
            r#"
                {"name": "test"}
            "#,
        )
        .send()
        .await
        .unwrap();

    let result_str = &res.text().await.unwrap();

    eprintln!("Result string: {}", result_str);

    assert!(&result_str.contains(r#"{"id":1,"name":"test","created_at":""#));
}
