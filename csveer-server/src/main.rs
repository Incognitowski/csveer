use std::{error::Error, time::Duration};

use tokio::signal;
use tokio_util::{sync::CancellationToken, task::TaskTracker};
use tracing::{info, info_span, instrument, Instrument};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    let main_span = info_span!("main");
    let _guard = main_span.enter();

    let tracker = TaskTracker::new();
    let token = CancellationToken::new();

    let app = csveer_server::build_app().await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7000").await?;

    spawn_listener(&tracker, &token);

    tracker.close();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(token.clone()))
        .await?;

    tracker.wait().await;

    println!("Application was shut down gracefully");

    Ok(())
}

#[instrument(name = "Listener", skip(tracker, base_token))]
fn spawn_listener(tracker: &TaskTracker, base_token: &CancellationToken) {
    let token = base_token.clone();
    tracker.spawn(
        async move {
            info!("Starting listener");
            tokio::select! {
                _ = listen(&token) => {
                    info!("Listener gracefully exited.")
                }
            }
        }
        .in_current_span(),
    );
}

#[instrument]
async fn listen(cancelation_token: &CancellationToken) {
    loop {
        if cancelation_token.is_cancelled() {
            break;
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
        info!("Loop")
    }
}

async fn shutdown_signal(cancelation_token: CancellationToken) {
    // I'm still not sure about how this works...
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {cancelation_token.cancel()},
        _ = terminate => {cancelation_token.cancel()},
    }
}
