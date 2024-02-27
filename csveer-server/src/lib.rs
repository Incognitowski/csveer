use axum::{routing::post, Router};
use sqlx::{migrate::Migrator, postgres::PgPoolOptions, Pool, Postgres};
use std::error::Error;

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
        .with_state(db_pool))
}
