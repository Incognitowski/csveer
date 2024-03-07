use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileIngestionListenerConfig {
    #[serde(rename(deserialize = "pending_csv_files_queue_url"))]
    pub queue_url: String,
}
