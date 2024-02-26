use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SourceType {}

#[derive(Serialize, Deserialize)]
pub enum CompressionType {}

#[derive(Serialize, Deserialize)]
pub struct FileSourceCreation {
    context: String,
    identifier: String,
    description: String,
    source: SourceType,
    headers: bool,
    compression: CompressionType,
    hide_columns: Vec<i32>,
}
