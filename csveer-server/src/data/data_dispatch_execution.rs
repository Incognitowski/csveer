use anyhow::Context;
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::PgConnection;

#[derive(Serialize)]
pub enum DataDispatchExecutionStatus {
    Success,
    Failure,
}

impl ToString for DataDispatchExecutionStatus {
    fn to_string(&self) -> String {
        match self {
            DataDispatchExecutionStatus::Success => "Success".to_string(),
            DataDispatchExecutionStatus::Failure => "Failure".to_string(),
        }
    }
}

impl TryFrom<String> for DataDispatchExecutionStatus {
    type Error = String;

    fn try_from(value: String) -> Result<Self, String> {
        match value.as_str() {
            "Failure" => Ok(Self::Failure),
            "Success" => Ok(Self::Success),
            _ => Err(format!(
                "{} is not a valid DataDispatchExecutionStatus value",
                value
            )),
        }
    }
}

struct DataDispatchExecutionCreation {
    pub data_dispatch_id: i32,
    pub status: DataDispatchExecutionStatus,
    pub message: String,
}

struct DataDispatchExecution {
    pub id: i32,
    pub data_dispatch_id: i32,
    pub status: DataDispatchExecutionStatus,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

struct DataDispatchExecutionEntity {
    id: i32,
    data_dispatch_id: i32,
    status: String,
    message: String,
    created_at: DateTime<Utc>,
}

impl Into<DataDispatchExecution> for DataDispatchExecutionEntity {
    fn into(self) -> DataDispatchExecution {
        return DataDispatchExecution {
            id: self.id,
            data_dispatch_id: self.data_dispatch_id,
            status: self.status.try_into().unwrap(),
            message: self.message,
            created_at: self.created_at,
        };
    }
}

pub async fn insert_data_dispatch_execution(
    data_dispatch_execution_creation: DataDispatchExecutionCreation,
    executor: &mut PgConnection,
) -> anyhow::Result<DataDispatchExecution> {
    let created_execution = sqlx::query_as!(
        DataDispatchExecutionEntity,
        r#"
            INSERT INTO data_dispatch_execution(data_dispatch_id, status, message, created_at)
            VALUES($1, $2, $3, NOW()) RETURNING *
        "#,
        data_dispatch_execution_creation.data_dispatch_id.clone(),
        data_dispatch_execution_creation.status.to_string(),
        data_dispatch_execution_creation.message,
    )
    .fetch_one(executor)
    .await
    .context("Inserting data dispatch execution")?
    .into();

    Ok(created_execution)
}
