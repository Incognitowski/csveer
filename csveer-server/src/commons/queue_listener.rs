use anyhow::Context;
use aws_sdk_sqs::{operation::receive_message::ReceiveMessageOutput, Client};

pub async fn poll_message(
    client: &Client,
    queue_url: &String,
) -> anyhow::Result<ReceiveMessageOutput> {
    let result = client
        .receive_message()
        .wait_time_seconds(10)
        .max_number_of_messages(1)
        .queue_url(queue_url)
        .send()
        .await
        .with_context(|| format!("Pooling SQS message from queue {}", queue_url))?;

    Ok(result)
}
