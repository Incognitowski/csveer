use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

#[derive(Serialize, Deserialize, Debug)]
pub enum CompressionMechanism {
    GZIP,
    ZIP,
}

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
        kind: CompressionMechanism,
        password: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
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
    pub id: i32,
    pub context: String,
    pub identifier: String,
    pub description: String,
    pub source: Option<SourceType>,
    pub headers: bool,
    pub compression: Option<CompressionType>,
    pub hide_columns: Option<Vec<i32>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
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

impl Into<FileSource> for FileSourceEntity {
    fn into(self) -> FileSource {
        return FileSource {
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
        };
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
    .context("Inserting file source into database")?
    .into();

    Ok(created_file_source)
}

pub async fn find_by_context_and_identifier(
    context: &str,
    identifier: &str,
    executor: &mut PgConnection,
) -> anyhow::Result<Option<FileSource>> {
    let file_source_entity = sqlx::query_as!(
        FileSourceEntity,
        r#"
            SELECT * FROM file_source fs WHERE fs.context = $1 AND fs.identifier = $2
        "#,
        context,
        identifier,
    )
    .fetch_optional(executor)
    .await
    .with_context(|| {
        format!(
            "Searching file sources with context {} and identifier {}",
            context, identifier
        )
    })?;

    let file_source = match file_source_entity {
        Some(entity) => Some(entity.into()),
        None => None,
    };

    Ok(file_source)
}
