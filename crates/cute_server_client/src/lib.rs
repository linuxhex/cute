pub mod auth;
pub mod cloud_object;
pub mod notebooks;
pub mod ids;
#[cfg(not(target_family = "wasm"))]
pub mod persistence;

pub use auth::UserUid;

// Backward compatibility: re-export drive types from cloud_object
pub mod drive {
    pub use crate::cloud_object::CloudObjectTypeAndId;
    pub mod sharing {
        pub use crate::cloud_object::sharing::*;
    }
}
