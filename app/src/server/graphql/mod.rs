pub mod schema;

use cute_graphql::client::RequestOptions;
pub use cute_graphql::client::GraphQLError;

/// Returns the default [`RequestOptions`] that should be used for a GraphQL request.
pub fn default_request_options() -> RequestOptions {
    RequestOptions {
        #[cfg(feature = "agent_mode_evals")]
        path_prefix: Some("/agent-mode-evals".to_string()),
        ..Default::default()
    }
}
