//! Stub module for remote_server after removal.
//!
//! This module provides stub types and functions to maintain API compatibility
//! after the remote_server crate was removed. All implementations panic or
//! return stub values.

pub mod codebase_index_model;
pub mod codebase_index_proto;
pub mod proto;
pub mod client;
pub mod setup;
pub mod transport;

// Re-export commonly used types
