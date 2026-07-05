use cute_graphql::object_permissions::OwnerType;

#[derive(Clone, Debug)]
pub enum Credentials {
    ApiKey {
        key: String,
        owner: Option<String>,
    },
    Bearer(String),
    SessionCookie,
    #[cfg(any(test, feature = "integration_tests", feature = "skip_login"))]
    Test,
}

impl Credentials {
    pub fn as_api_key(&self) -> Option<&str> {
        match self {
            Credentials::ApiKey { key, .. } => Some(key),
            Credentials::Bearer(_) => None,
            Credentials::SessionCookie => None,
            #[cfg(any(test, feature = "integration_tests", feature = "skip_login"))]
            Credentials::Test => None,
        }
    }

    pub fn api_key_owner_type(&self) -> Option<OwnerType> {
        None
    }

    pub fn is_externally_managed(&self) -> bool {
        matches!(self, Credentials::Bearer(_))
    }

    pub fn bearer_token(&self) -> AuthToken {
        match self {
            Credentials::ApiKey { key, .. } => AuthToken::ApiKey(key.clone()),
            Credentials::Bearer(token) => AuthToken::Bearer(token.clone()),
            Credentials::SessionCookie => AuthToken::NoAuth,
            #[cfg(any(test, feature = "integration_tests", feature = "skip_login"))]
            Credentials::Test => AuthToken::NoAuth,
        }
    }
}

#[derive(Debug, Clone)]
pub enum AuthToken {
    ApiKey(String),
    Bearer(String),
    #[cfg_attr(
        not(any(test, feature = "integration_tests", feature = "skip_login")),
        allow(dead_code)
    )]
    NoAuth,
}

impl AuthToken {
    pub fn as_bearer_token(&self) -> Option<&str> {
        match self {
            AuthToken::ApiKey(key) => Some(key),
            AuthToken::Bearer(token) => Some(token),
            AuthToken::NoAuth => None,
        }
    }

    pub fn bearer_token(&self) -> Option<String> {
        match self {
            AuthToken::ApiKey(key) => Some(key.clone()),
            AuthToken::Bearer(token) => Some(token.clone()),
            AuthToken::NoAuth => None,
        }
    }
}