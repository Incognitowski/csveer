use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum SourceType {
    HttpPassive,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CompressionType {
    NoCompression(bool),
    Compressed {
        #[serde(rename = "type")]
        kind: String,
        password: Option<String>,
    },
}

#[derive(Serialize, Deserialize)]
pub struct FileSourceCreation {
    pub context: String,
    pub identifier: String,
    pub description: String,
    pub source: SourceType,
    pub headers: bool,
    pub compression: Option<CompressionType>,
    pub hide_columns: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FileSource {
    id: i32,
    context: String,
    identifier: String,
    description: String,
    source: Option<SourceType>,
    headers: bool,
    compression: Option<CompressionType>,
    hide_columns: Option<Vec<i32>>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

pub struct FileSourceEntity {
    id: i32,
    context: String,
    identifier: String,
    description: String,
    source: sqlx::types::JsonValue,
    headers: bool,
    compression: sqlx::types::JsonValue,
    hide_columns: Option<Vec<i32>>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl FileSourceEntity {
    fn to_domain(self) -> FileSource {
        FileSource {
            id: self.id,
            context: self.context,
            identifier: self.identifier,
            description: self.description,
            source: serde_json::from_value(self.source).unwrap(),
            headers: self.headers,
            compression: serde_json::from_value(self.compression).unwrap(),
            hide_columns: self.hide_columns,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

pub async fn insert_file_source(
    creatable_file_source: FileSourceCreation,
    executor: &mut PgConnection,
) -> anyhow::Result<FileSource> {
    let created_file_source = sqlx::query_as!(
        FileSourceEntity,
        r#"
           INSERT INTO file_source(context, identifier, description, "source", headers, compression, hide_columns, created_at)
           VALUES($1, $2, $3, $4, $5, $6, $7, NOW()) RETURNING *
        "#,
        creatable_file_source.context,
        creatable_file_source.identifier,
        creatable_file_source.description,
        serde_json::to_value(creatable_file_source.source)?,
        creatable_file_source.headers,
        serde_json::to_value(creatable_file_source.compression)?,
        creatable_file_source.hide_columns.as_deref()
    )
    .fetch_one(executor)
    .await
    .context("Failed to insert file source into database")?
    .to_domain();

    Ok(created_file_source)
}
