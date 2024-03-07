use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AwsConfig {
    pub region: String,
    pub aws_endpoint: String,
}
