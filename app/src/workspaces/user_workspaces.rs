// Minimal stub for user workspaces functionality
// This module provides stub definitions for workspace-related types

use serde::{Deserialize, Serialize};

/// Minimal stub for WorkspacesMetadataResponse
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WorkspacesMetadataResponse {
    pub workspaces: Vec<WorkspaceMetadata>,
    pub joinable_teams: Vec<TeamMetadata>,
    pub feature_model_choices: Option<FeatureModelChoices>,
}

/// Minimal stub for WorkspaceMetadata
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WorkspaceMetadata {
    pub uid: String,
    pub name: String,
    pub teams: Vec<TeamMetadata>,
}

/// Minimal stub for TeamMetadata
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TeamMetadata {
    pub uid: String,
    pub name: String,
    pub billing_metadata: Option<BillingMetadata>,
}

/// Minimal stub for BillingMetadata
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct BillingMetadata {}

/// Minimal stub for FeatureModelChoices
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FeatureModelChoices {}