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

pub async fn post_message(
    client: &Client,
    queue_url: &String,
    message: String,
) -> anyhow::Result<()> {
    let _ = client
        .send_message()
        .queue_url(queue_url)
        .message_body(message)
        .send()
        .await
        .with_context(|| format!("Sending SQS message to queue {}", queue_url))?;

    Ok(())
}

pub async fn ack_message(
    client: &Client,
    queue_url: &String,
    message_receipt: &str,
) -> anyhow::Result<()> {
    let _ = client
        .delete_message()
        .queue_url(queue_url)
        .receipt_handle(message_receipt)
        .send()
        .await
        .with_context(|| format!("Acking SQS message of queue {}", queue_url))?;

    Ok(())
}
