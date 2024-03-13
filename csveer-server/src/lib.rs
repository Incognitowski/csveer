use aws_sdk_s3::Client as S3Client;
use aws_sdk_sqs::Client as SQSClient;
use axum::{extract::MatchedPath, http::Request, routing::post, Router};
use config::{
    aws::AwsConfig,
    server::{AppError, AppState},
};
use sqlx::{migrate::Migrator, postgres::PgPoolOptions, Pool, Postgres};
use std::error::Error;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tower_http::trace::TraceLayer;
use tracing::{error, info, info_span, instrument, Instrument};
use ulid::Ulid;

mod app;
mod commons;
mod config;
mod data;

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn get_db_pool(db_uri: String) -> Result<Pool<Postgres>, Box<dyn Error>> {
    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_uri)
        .await?)
}

pub fn get_aws_config() -> anyhow::Result<AwsConfig> {
    Ok(envy::from_env::<AwsConfig>()?)
}

pub async fn build_app(
    db_pool: Pool<Postgres>,
    s3_client: S3Client,
    sqs_client: SQSClient,
) -> Result<Router, Box<dyn Error>> {
    MIGRATOR.run(&db_pool).await?;

    let app_state = AppState {
        db_pool,
        s3_client,
        sqs_client,
    };

    Ok(Router::new()
        .route("/context", post(app::context::create_context))
        .route("/source", post(app::file_source::create_file_source))
        .route(
            "/:context/:file_source/destination",
            post(app::file_destination::create_file_destination),
        )
        .route(
            "/:context/:file_source/upload",
            post(app::file_input::file_upload),
        )
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path,
                    context = Ulid::new().to_string(),
                )
            }),
        ))
}

#[instrument(skip(tracker, token, db_pool))]
pub async fn start_file_ingestion_listener(
    tracker: &TaskTracker,
    token: CancellationToken,
    db_pool: Pool<Postgres>,
) {
    let listener_span = info_span!("file-ingestion-listener");
    let _span_guard = listener_span.enter();
    tracker.spawn(
        async move {
            tokio::select! {
                res = app::file_ingestion_queue::listen_file_ingestion(&token, &db_pool) => {
                    match res {
                        Ok(_) => info!("File ingestion listener was shut down."),
                        Err(err) => error!("Listener execution failed. {:?}", err),
                    }

                }
            }
        }
        .in_current_span(),
    );
}

pub async fn get_s3_client(aws_config: &AwsConfig) -> anyhow::Result<S3Client, AppError> {
    Ok(config::aws::get_s3_client(aws_config).await?)
}

pub async fn get_sqs_client(aws_config: &AwsConfig) -> anyhow::Result<SQSClient, AppError> {
    Ok(config::aws::get_sqs_client(aws_config).await?)
}
