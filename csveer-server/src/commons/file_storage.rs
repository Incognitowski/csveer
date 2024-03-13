use anyhow::Context;
use aws_sdk_s3::{operation::put_object::PutObjectOutput, primitives::ByteStream, Client};

pub async fn store_file_for_ingestion(
    client: &Client,
    bucket_name: &str,
    file: ByteStream,
    file_name: &str,
) -> anyhow::Result<PutObjectOutput> {
    let res = client
        .put_object()
        .set_bucket(Some(bucket_name.to_string()))
        .set_body(Some(file))
        .set_key(Some(file_name.to_string()))
        .send()
        .await
        .with_context(|| format!("Sending S3 object to bucket {}", bucket_name))?;
    Ok(res)
}
