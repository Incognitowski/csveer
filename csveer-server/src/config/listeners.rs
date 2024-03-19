use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FileIngestionListenerConfig {
    #[serde(rename(deserialize = "pending_csv_files_queue_url"))]
    pub queue_url: String,
}

#[derive(Debug, Deserialize)]
pub struct DataDispatchListenerConfig {
    #[serde(rename(deserialize = "data_dispatch_queue_url"))]
    pub queue_url: String,
}
