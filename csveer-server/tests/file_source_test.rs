use reqwest::StatusCode;

mod common;

#[tokio::test]
async fn test_should_create_file_source() {
    let addr = common::prepare_for_test().await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/source", addr))
        .header("Content-Type", "application/json")
        .body(include_str!("requests/file_source/create_file_source.json"))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_should_fail_with_invalid_character_on_source_identifier() {
    let addr = common::prepare_for_test().await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/source", addr))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_source/invalid_file_source_0001.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_should_fail_with_blank_source_identifier() {
    let addr = common::prepare_for_test().await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/source", addr))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_source/invalid_file_source_0002.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_should_fail_with_invalid_hide_column_value() {
    let addr = common::prepare_for_test().await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://{}/source", addr))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_source/invalid_file_source_0003.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
