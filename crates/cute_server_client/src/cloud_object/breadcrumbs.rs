// Breadcrumbs module stub.
//
// The real implementation tracks the chain of parent objects for a given cloud
// object so that UIs can render a "breadcrumbs" navigation path. This stub
// exists so that downstream crates can import `cute_server_client::cloud_object::breadcrumbs`
// while the full implementation is being ported.

/// A single breadcrumb entry pointing at an ancestor cloud object.
#[derive(Debug, Clone)]
pub struct Breadcrumb {
    pub id: String,
    pub title: String,
}

/// A list of breadcrumbs describing the path from the root to a cloud object.
#[derive(Debug, Clone, Default)]
pub struct Breadcrumbs {
    pub entries: Vec<Breadcrumb>,
}

/// Represents an object that contains other objects (for breadcrumb navigation).
#[derive(Debug, Clone)]
pub struct ContainingObject {
    pub id: String,
    pub title: String,
}
