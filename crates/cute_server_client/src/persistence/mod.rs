//! Persistence utilities for cloud objects.

/// The sqlite id of a cloud object.
pub type CloudObjectId = i32;

/// Stub function for encoding guests.
/// This function has been removed as part of cloud functionality removal.
pub fn encode_guests<T>(_guests: &[T]) -> String {
    String::new()
}

/// Stub function for upserting cloud objects.
/// This function has been removed as part of cloud functionality removal.
pub fn upsert_cloud_object() -> ! {
    panic!("upsert_cloud_object has been removed - cloud functionality disabled")
}
