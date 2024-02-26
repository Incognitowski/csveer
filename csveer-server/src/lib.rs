use axum::{routing::post, Router};
use sqlx::{migrate::Migrator, postgres::PgPoolOptions};
use std::error::Error;

mod app;
mod config;
mod data;

static MIGRATOR: Migrator = sqlx::migrate!();

pub async fn build_app() -> Result<Router, Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:root@localhost/postgres")
        .await?;

    MIGRATOR.run(&pool).await?;

    Ok(Router::new()
        .route("/context", post(app::context::create_context))
        .with_state(pool))
}
