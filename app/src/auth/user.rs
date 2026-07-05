use serde::{Deserialize, Serialize};
pub use cute_server_client::auth::{TEST_USER_EMAIL, TEST_USER_UID};

use super::UserUid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AnonymousUserType {
    NativeClientAnonymousUser,
    NativeClientAnonymousUserFeatureGated,
    WebClientAnonymousUser,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PrincipalType {
    #[default]
    User,
    ServiceAccount,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct PersonalObjectLimits {
    pub env_var_limit: usize,
    pub notebook_limit: usize,
    pub workflow_limit: usize,
}

#[derive(Debug, Clone)]
pub struct User {
    pub local_id: UserUid,
    pub metadata: UserMetadata,
    pub is_onboarded: bool,
    pub needs_sso_link: bool,
    pub anonymous_user_type: Option<AnonymousUserType>,
    pub is_on_work_domain: bool,
    pub linked_at: Option<()>,
    pub personal_object_limits: Option<PersonalObjectLimits>,
    pub principal_type: PrincipalType,
    pub global_skills: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct UserMetadata {
    pub email: String,
    pub display_name: Option<String>,
    pub photo_url: Option<String>,
}

impl User {
    pub fn username_for_display(&self) -> &str {
        let user_metadata = &self.metadata;
        user_metadata
            .display_name
            .as_deref()
            .unwrap_or(user_metadata.email.as_str())
    }

    pub fn display_name(&self) -> Option<String> {
        self.metadata.display_name.clone()
    }

    pub fn test() -> Self {
        Self {
            local_id: UserUid::new(TEST_USER_UID),
            metadata: UserMetadata {
                email: TEST_USER_EMAIL.to_string(),
                display_name: None,
                photo_url: None,
            },
            is_onboarded: true,
            needs_sso_link: false,
            anonymous_user_type: None,
            is_on_work_domain: false,
            linked_at: None,
            personal_object_limits: None,
            principal_type: PrincipalType::User,
            global_skills: Vec::new(),
        }
    }

    pub fn is_user_anonymous(&self) -> bool {
        self.anonymous_user_type().is_some() && self.linked_at().is_none()
    }

    pub fn anonymous_user_type(&self) -> Option<AnonymousUserType> {
        self.anonymous_user_type
    }

    pub fn personal_object_limits(&self) -> Option<PersonalObjectLimits> {
        self.personal_object_limits
    }

    pub fn linked_at(&self) -> Option<()> {
        self.linked_at
    }
}

#[cfg(test)]
#[path = "user_tests.rs"]
mod tests;