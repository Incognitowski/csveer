use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgConnection;

#[derive(Serialize)]
pub enum DataDispatchStatus {
    PendingExecution,
    Failed,
    Finished,
}

impl TryFrom<String> for DataDispatchStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "PendingExecution" => Ok(Self::PendingExecution),
            "Failed" => Ok(Self::Failed),
            "Finished" => Ok(Self::Finished),
            _ => Err(format!("{} is not a valid DataDispatchSource", value)),
        }
    }
}

impl ToString for DataDispatchStatus {
    fn to_string(&self) -> String {
        match self {
            DataDispatchStatus::PendingExecution => "PendingExecution".to_string(),
            DataDispatchStatus::Failed => "Failed".to_string(),
            DataDispatchStatus::Finished => "Finished".to_string(),
        }
    }
}

pub struct DataDispatchCreation {
    pub file_destination_id: i32,
    pub message: String,
}

#[derive(Serialize)]
pub struct DataDispatch {
    pub id: i32,
    pub file_destination_id: i32,
    pub status: DataDispatchStatus,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

struct DataDispatchEntity {
    id: i32,
    file_destination_id: i32,
    status: String,
    message: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

impl Into<DataDispatch> for DataDispatchEntity {
    fn into(self) -> DataDispatch {
        return DataDispatch {
            id: self.id,
            file_destination_id: self.file_destination_id,
            status: self.status.try_into().unwrap(),
            message: self.message,
            created_at: self.created_at,
            updated_at: self.updated_at,
        };
    }
}

pub async fn insert_data_dispatch(
    data_dispatch_creation: DataDispatchCreation,
    executor: &mut PgConnection,
) -> anyhow::Result<DataDispatch> {
    let created_data_dispatch = sqlx::query_as!(
        DataDispatchEntity,
        r#"
            INSERT INTO data_dispatch(file_destination_id, status, message, created_at)
            VALUES($1, $2, $3, NOW()) RETURNING *
        "#,
        data_dispatch_creation.file_destination_id.clone(),
        DataDispatchStatus::PendingExecution.to_string(),
        data_dispatch_creation.message.clone(),
    )
    .fetch_one(executor)
    .await
    .context("Inserting data dispatch record")?
    .into();

    Ok(created_data_dispatch)
}
