use aws_config::BehaviorVersion;
use aws_sdk_s3::{config::Region as S3Region, Client as S3Client};
use aws_sdk_sqs::{config::Region as SQSRegion, Client as SQSClient};
use serde::Deserialize;

use super::server::AppError;

#[derive(Clone, Debug, Deserialize)]
pub struct AwsConfig {
    pub region: String,
    pub aws_endpoint: String,
}

pub async fn get_sqs_client(aws_config: &AwsConfig) -> anyhow::Result<SQSClient, AppError> {
    let region = SQSRegion::new(aws_config.region.clone());
    let shared_config = aws_config::defaults(BehaviorVersion::v2023_11_09())
        .region(region)
        .endpoint_url(aws_config.aws_endpoint.clone())
        .load()
        .await;
    Ok(SQSClient::new(&shared_config))
}

pub async fn get_s3_client(aws_config: &AwsConfig) -> anyhow::Result<S3Client, AppError> {
    let region = S3Region::new(aws_config.region.clone());
    let config = aws_sdk_s3::Config::builder()
        .region(region)
        .endpoint_url(aws_config.aws_endpoint.clone())
        .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest())
        .force_path_style(true)
        .build();
    Ok(S3Client::from_conf(config))
}
