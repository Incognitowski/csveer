use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

#[derive(Debug, Serialize)]
pub struct SourceContext {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatableContext {
    pub name: String,
}

pub async fn insert_context(
    creatable_context: CreatableContext,
    executor: &mut PgConnection,
) -> anyhow::Result<SourceContext> {
    let created_context = sqlx::query_as!(
        SourceContext,
        r#"
            INSERT INTO context("name", created_at)
            VALUES($1, NOW()) RETURNING *
        "#,
        creatable_context.name,
    )
    .fetch_one(executor)
    .await
    .context("Failed to insert context into database")?;

    Ok(created_context)
}

pub async fn get_context_by_name(
    name: &String,
    executor: &mut PgConnection,
) -> Option<SourceContext> {
    let context = sqlx::query_as!(
        SourceContext,
        r#"
            SELECT * FROM context WHERE name = $1
        "#,
        name
    )
    .fetch_optional(executor)
    .await
    .with_context(|| format!("Failed to search context with name {}", name))
    .unwrap();

    context
}
