use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::JsonValue, PgConnection};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum DestinationConfiguration {
    SQS { queue_url: String },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GroupingConfiguration {
    GroupedByColumns { columns: Vec<i32> },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BatchingConfiguration {
    Fixed { batch_size: i32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDestination {
    pub id: i32,
    pub file_source_id: i32,
    pub identifier: String,
    pub destination: DestinationConfiguration,
    pub include_headers: bool,
    pub grouping: Option<GroupingConfiguration>,
    pub batching: Option<BatchingConfiguration>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDestinationCreation {
    pub identifier: String,
    pub destination: DestinationConfiguration,
    pub include_headers: bool,
    pub grouping: Option<GroupingConfiguration>,
    pub batching: Option<BatchingConfiguration>,
}

struct FileDestinationEntity {
    pub id: i32,
    pub file_source_id: i32,
    pub identifier: String,
    pub destination: JsonValue,
    pub include_headers: bool,
    pub grouping: JsonValue,
    pub batching: JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl FileDestinationEntity {
    fn to_domain(self) -> FileDestination {
        return FileDestination {
            id: self.id,
            file_source_id: self.file_source_id,
            identifier: self.identifier,
            destination: serde_json::from_value(self.destination).unwrap(),
            include_headers: self.include_headers,
            grouping: serde_json::from_value(self.grouping).unwrap(),
            batching: serde_json::from_value(self.batching).unwrap(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        };
    }
}

pub async fn insert_file_destination(
    file_source_id: &i32,
    file_destination_creation: FileDestinationCreation,
    executor: &mut PgConnection,
) -> anyhow::Result<FileDestination> {
    let created_file_destination = sqlx::query_as!(
        FileDestinationEntity,
        r#"        
            INSERT INTO file_destination(file_source_id, identifier, destination, include_headers, "grouping", batching, created_at)
            VALUES($1, $2, $3, $4, $5, $6, NOW()) RETURNING *
        "#,
        file_source_id.clone(),
        file_destination_creation.identifier,
        serde_json::to_value(file_destination_creation.destination)?,
        file_destination_creation.include_headers,
        serde_json::to_value(file_destination_creation.grouping)?,
        serde_json::to_value(file_destination_creation.batching)?
    )
    .fetch_one(executor)
    .await
    .context("Failed to insert file destination into database")?
    .to_domain();

    Ok(created_file_destination)
}

pub async fn list_by_file_source_id(
    file_source_id: &i32,
    executor: &mut PgConnection,
) -> anyhow::Result<Vec<FileDestination>> {
    let file_destinations = sqlx::query_as!(
        FileDestinationEntity,
        r#"        
            SELECT * FROM file_destination fd WHERE fd.file_source_id = $1
        "#,
        file_source_id.clone(),
    )
    .fetch_all(executor)
    .await
    .with_context(|| {
        format!(
            "Searching for file destinations for file source {}",
            file_source_id
        )
    })?
    .into_iter()
    .map(|i| i.to_domain())
    .collect();

    Ok(file_destinations)
}
