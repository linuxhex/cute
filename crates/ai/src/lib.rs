//! Stub implementation for ai crate
//! This provides minimal types needed for onboarding and other crates

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LLMId(pub String);

impl fmt::Display for LLMId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for LLMId {
    fn from(s: &str) -> Self {
        LLMId(s.to_string())
    }
}

impl From<String> for LLMId {
    fn from(s: String) -> Self {
        LLMId(s)
    }
}

pub mod workspace {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct WorkspaceMetadata {
        pub id: String,
        pub name: String,
    }
}

pub mod index {
    pub mod full_source_code_embedding {
        pub mod manager {
            #[derive(Debug, Clone)]
            pub struct CodebaseIndexManager;
        }
    }
}

pub mod project_context {
    pub mod model {
        #[derive(Debug, Clone)]
        pub struct ProjectContextModel;
    }
}

pub mod diff_validation {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum DiffType {
        Unified,
        Split,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DiffDelta {
        pub old_start: u32,
        pub old_lines: u32,
        pub new_start: u32,
        pub new_lines: u32,
    }
}

pub mod skills {
    #[derive(Debug, Clone)]
    pub struct SkillProvider;
}

pub mod predict {
    pub mod generate_ai_input_suggestions {
        #[derive(Debug, Clone)]
        pub struct GenerateAIInputSuggestionsRequest;
    }
}

pub mod ambient_agents {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct AmbientAgentTaskId(pub String);
}

pub mod get_relevant_files {
    pub mod api {
        #[derive(Debug, Clone)]
        pub struct GetRelevantFiles;

        #[derive(Debug, Clone)]
        pub struct GetRelevantFilesResponse;
    }
}
