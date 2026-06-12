//! Stub for diff state proto types after remote_server removal.

use anyhow::Result;

/// Stub function for try_decode_file_delta.
pub fn try_decode_file_delta(_data: &[u8]) -> Result<()> {
    anyhow::bail!("remote_server has been removed")
}

/// Stub function for try_decode_snapshot.
pub fn try_decode_snapshot(_data: &[u8]) -> Result<()> {
    anyhow::bail!("remote_server has been removed")
}
