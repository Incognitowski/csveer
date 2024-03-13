use std::{env, error::Error};

use dotenv::dotenv;
use tokio::signal;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{error, info_span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    start_tracing();

    let main_span = info_span!("main");
    let _span_guard = main_span.enter();

    let tracker = TaskTracker::new();
    let cancellation_token = CancellationToken::new();

    let db_uri = match env::var("DATABASE_URL") {
        Ok(db_uri) => db_uri,
        Err(_) => {
            let message = "DATABASE_URL environment variable is not set.";
            error!(message);
            panic!("{}", message);
        }
    };

    let aws_config = csveer_server::get_aws_config().expect("Failed to parse AWS config");

    let db_pool = csveer_server::get_db_pool(db_uri)
        .await
        .expect("Failed to get database pool");
    let s3_client = csveer_server::get_s3_client(&aws_config)
        .await
        .expect("Failed to create S3 client");
    let sqs_client = csveer_server::get_sqs_client(&aws_config)
        .await
        .expect("Failed to create SQS client");

    let app =
        csveer_server::build_app(db_pool.clone(), s3_client.clone(), sqs_client.clone()).await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7000").await?;

    csveer_server::start_file_ingestion_listener(
        &tracker,
        cancellation_token.clone(),
        db_pool.clone(),
    )
    .await;

    tracker.close();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(cancellation_token.clone()))
        .await?;

    tracker.wait().await;

    println!("Application was shut down gracefully");

    Ok(())
}

fn start_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "csveer_server=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn shutdown_signal(cancelation_token: CancellationToken) {
    // This starts a listener on Ctrl + C input
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    // This starts a listener on the Unix terminate signal
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // Whichever signal is received first, cancels our token.
    tokio::select! {
        _ = ctrl_c => { cancelation_token.cancel() },
        _ = terminate => { cancelation_token.cancel() },
    }
}
