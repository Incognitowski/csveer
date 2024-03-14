use aws_lambda_events::s3::S3Event;
use aws_sdk_sqs::types::Message;
use aws_sdk_sqs::Client as SQSClient;
use sqlx::{Pool, Postgres};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, info_span, instrument};
use ulid::Ulid;

use crate::{
    commons::queue_listener::poll_message,
    config::{
        listeners::FileIngestionListenerConfig,
        server::{AppError, AppState},
    },
    data::{file_destination, file_source},
};

pub async fn listen_file_ingestion(
    token: &CancellationToken,
    app_state: AppState,
) -> anyhow::Result<(), AppError> {
    let sqs_client = app_state.sqs_client;
    let file_ingestion_config = envy::from_env::<FileIngestionListenerConfig>()?;
    let mut db_pool = app_state.db_pool;
    let listener_span = info_span!(parent: None, "file-ingestion-listener");
    let _guard = listener_span.enter();

    loop {
        if token.is_cancelled() {
            info!("Shutting down file ingestion listener");
            break;
        }
        let message = poll_message(&sqs_client, &file_ingestion_config.queue_url).await?;
        if let Some(messages) = message.messages {
            let message_span =
                info_span!("file-ingestion-message", trace_id = Ulid::new().to_string());
            message_span.follows_from(listener_span.clone());
            let _guard = message_span.enter();
            // SAFETY: We can unwrap here because we've
            // guaranteed there is a message with `if let`
            let message = messages.first().unwrap();
            info!("About to process message: {}", message.body().unwrap());
            process_message(&mut db_pool, &sqs_client, message).await?;
            // TODO: Match on result to decide if will retry or discard message
        }
        debug!("listener loop");
    }
    Ok(())
}

#[instrument(skip(db_pool, sqs_client, message))]
async fn process_message(
    db_pool: &mut Pool<Postgres>,
    sqs_client: &SQSClient,
    message: &Message,
) -> anyhow::Result<(), AppError> {
    let message_body = match message.body() {
        Some(b) => b,
        None => {
            info!("Could not get event body as string. Aborting processing of message.");
            return Ok(());
        }
    };
    let s3_event = match serde_json::from_str::<S3Event>(message_body) {
        Ok(e) => e,
        Err(_) => {
            info!("Could not parse message body into event. Aborting processing of message.");
            return Ok(());
        }
    };
    process_s3_event(db_pool, sqs_client, s3_event).await?;
    Ok(())
}

#[instrument(skip(db_pool, sqs_client, s3_event))]
async fn process_s3_event(
    db_pool: &mut Pool<Postgres>,
    sqs_client: &SQSClient,
    s3_event: S3Event,
) -> anyhow::Result<(), AppError> {
    for record in s3_event.records {
        let event_name = record.event_name.unwrap();
        if !event_name.starts_with(&"ObjectCreated") {
            info!("Skipped record with event {}", event_name);
            continue;
        }
        let object_key = record.s3.object.key.unwrap();
        let (context, remainder) = object_key.split_once('/').unwrap();
        let (file_source_identifier, file_name) = remainder.split_once('/').unwrap();

        let mut tx = db_pool.begin().await?;

        let file_source = match file_source::find_by_context_and_identifier(
            context,
            file_source_identifier,
            &mut *tx,
        )
        .await?
        {
            Some(file_source) => file_source,
            None => {
                error!(
                    "Could not find file source with identifier {} on context {}.",
                    file_source_identifier, context
                );
                continue;
            }
        };

        let file_destinations =
            file_destination::list_by_file_source_id(&file_source.id, &mut *tx).await?;

        for destination in file_destinations {
            info!(
                "Sending message to data dispatch for file destination '{}'",
                destination.identifier
            );
        }

        tx.commit().await?;
    }

    Ok(())
}
