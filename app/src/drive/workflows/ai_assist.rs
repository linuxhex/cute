use crate::workflows::workflow::Argument;

#[derive(Debug, Clone)]
pub struct GeneratedCommandMetadata {
    pub title: String,
    pub description: String,
    pub command: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug)]
pub enum GeneratedCommandMetadataError {
    ParsingError,
    RateLimited,
}

impl GeneratedCommandMetadataError {
    pub fn user_facing_message(&self) -> String {
        match self {
            GeneratedCommandMetadataError::ParsingError => "Failed to parse generated command metadata".to_string(),
            GeneratedCommandMetadataError::RateLimited => "Rate limited - please try again later".to_string(),
        }
    }
}