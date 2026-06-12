//! Stub for RemoteServerClient after remote_server removal.

use std::sync::Arc;

/// Stub RemoteServerClient. All methods panic since remote_server has been removed.
#[derive(Clone)]
pub struct RemoteServerClient {
    _phantom: (),
}

impl RemoteServerClient {
    pub fn new() -> Self {
        Self { _phantom: () }
    }
}
