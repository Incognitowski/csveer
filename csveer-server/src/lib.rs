use axum::{extract::MatchedPath, http::Request, routing::post, Router};
use sqlx::{migrate::Migrator, postgres::PgPoolOptions, Pool, Postgres};
use std::error::Error;
use tower_http::trace::TraceLayer;
use tracing::info_span;
use ulid::Ulid;

mod app;
mod config;
mod data;

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn get_db_pool(db_uri: String) -> Result<Pool<Postgres>, Box<dyn Error>> {
    Ok(PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_uri)
        .await?)
}

pub async fn build_app(db_pool: Pool<Postgres>) -> Result<Router, Box<dyn Error>> {
    MIGRATOR.run(&db_pool).await?;

    Ok(Router::new()
        .route("/context", post(app::context::create_context))
        .route("/source", post(app::file_source::create_file_source))
        .route(
            "/:context/:file_source/destination",
            post(app::file_destination::create_file_destination),
        )
        .with_state(db_pool)
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
