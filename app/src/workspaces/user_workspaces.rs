// Minimal stub for user workspaces functionality
// This module provides stub definitions for workspace-related types
// Team and sharing features are removed for the local version.

use serde::{Deserialize, Serialize};

/// Minimal stub for WorkspacesMetadataResponse
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WorkspacesMetadataResponse {
    pub workspaces: Vec<WorkspaceMetadata>,
    pub feature_model_choices: Option<FeatureModelChoices>,
}

/// Minimal stub for WorkspaceMetadata
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WorkspaceMetadata {
    pub uid: String,
    pub name: String,
}

/// Minimal stub for TeamMetadata (kept for persistence/sqlite.rs compatibility)
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TeamMetadata {
    pub uid: String,
    pub name: String,
}

/// Minimal stub for BillingMetadata
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BillingMetadata {}

/// Minimal stub for FeatureModelChoices
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FeatureModelChoices {}
