use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GenerateBlockTitleRequest {
    pub command: String,
    pub output: String,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GenerateBlockTitleResponse {
    pub title: String,
}
