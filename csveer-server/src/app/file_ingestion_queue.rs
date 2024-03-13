use sqlx::{Pool, Postgres};
use tokio_util::sync::CancellationToken;
use tracing::{debug, info, instrument};

use crate::{
    commons::queue_listener::poll_message,
    config::{
        aws::{get_sqs_client, AwsConfig},
        listeners::FileIngestionListenerConfig,
        server::AppError,
    },
};

#[instrument(skip(token, db_pool))]
pub async fn listen_file_ingestion(
    token: &CancellationToken,
    db_pool: &Pool<Postgres>,
) -> anyhow::Result<(), AppError> {
    let aws_config = envy::from_env::<AwsConfig>()?;
    let file_ingestion_config = envy::from_env::<FileIngestionListenerConfig>()?;
    let sqs_client = get_sqs_client(&aws_config).await?;
    loop {
        if token.is_cancelled() {
            info!("Shutting down file ingestion listener");
            break;
        }
        let message = poll_message(&sqs_client, &file_ingestion_config.queue_url).await?;
        if let Some(messages) = message.messages {
            info!("About to process message from SQS queue.");
            // SAFETY: We can unwrap here because we've
            // guaranteed there is a message with `if let`
            let message = messages.first().unwrap();
            info!("Message received: {}", message.body().unwrap())
        }
        debug!("listener loop");
    }
    Ok(())
}
