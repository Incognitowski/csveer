use reqwest::StatusCode;

mod common;

#[tokio::test]
async fn test_should_create_context() {
    let addr = common::prepare_for_test().await;

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

#[tokio::test]
async fn test_should_fail_to_insert_context_with_empty_name() {
    let addr = common::prepare_for_test().await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/context", addr))
        .header("Content-Type", "application/json")
        .body(
            r#"
                {"name": ""}
            "#,
        )
        .send()
        .await
        .unwrap();

    let result_str = &res.text().await.unwrap();

    eprintln!("Result string: {}", result_str);

    assert_eq!(
        &result_str,
        &r#"{"message":"Context name should not be empty.","details":[]}"#
    );
}

#[tokio::test]
async fn test_should_fail_to_insert_context_with_invalid_char_in_name() {
    let addr = common::prepare_for_test().await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/context", addr))
        .header("Content-Type", "application/json")
        .body(
            r#"
                {"name": "test-&-context"}
            "#,
        )
        .send()
        .await
        .unwrap();

    let result_str = &res.text().await.unwrap();

    eprintln!("Result string: {}", result_str);

    assert_eq!(
        &result_str,
        &r#"{"message":"Context name should only contain numbers or charaters. Found char '&'","details":[]}"#
    );
}

#[tokio::test]
async fn test_should_fail_to_insert_context_with_duplicated_name() {
    let addr = common::prepare_for_test().await;

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

    assert_eq!(res.status(), StatusCode::CREATED);

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

    assert_eq!(
        &result_str,
        &r#"{"message":"A context with name 'test' already exists.","details":[]}"#
    );
}
