use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DataDispatchMessage {
    pub context: String,
    pub file_source_identifier: String,
    pub file_destination_identifier: String,
    pub file_name: String,
}
