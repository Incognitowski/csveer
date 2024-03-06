use reqwest::StatusCode;

mod common;

#[tokio::test]
async fn test_should_succesfully_create_file_destination() {
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/create_file_destination_0001.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_should_succesfully_create_file_destination_without_grouping() {
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/create_file_destination_0002.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_should_succesfully_create_file_destination_without_batching() {
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/create_file_destination_0003.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_should_succesfully_create_file_destination_without_either_grouping_or_batching() {
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/create_file_destination_0004.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_should_fail_to_create_destination_given_empty_identifier() {
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/invalid_file_destination_0001.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_should_fail_to_create_destination_given_identifier_with_invalid_characters() {
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/invalid_file_destination_0002.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_should_fail_to_create_sqs_destination_given_empty_queue_url() {
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/invalid_file_destination_0003.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_should_fail_to_create_sqs_destination_given_invalid_queue_url() {
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/invalid_file_destination_0004.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_should_fail_to_create_sqs_destination_given_empty_column_grouping() {
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/invalid_file_destination_0005.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_should_fail_to_create_sqs_destination_given_column_grouping_with_negative_index() {
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/invalid_file_destination_0006.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_should_fail_to_create_sqs_destination_given_column_grouping_with_duplicated_indices()
{
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/invalid_file_destination_0007.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_should_fail_to_create_sqs_destination_given_fixed_batching_with_size_0() {
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/invalid_file_destination_0008.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_should_fail_to_create_sqs_destination_given_fixed_batching_with_negative_size() {
    let addr = common::prepare_for_test().await;
    common::create_file_source(&addr).await;

    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "http://{}/banking/daily-transfer-csv/destination",
            addr
        ))
        .header("Content-Type", "application/json")
        .body(include_str!(
            "requests/file_destination/invalid_file_destination_0009.json"
        ))
        .send()
        .await
        .unwrap();

    // let result_str = &res.text().await.unwrap();
    // eprintln!("Result string: {}", result_str);

    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
