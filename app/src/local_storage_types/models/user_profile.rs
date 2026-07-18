use cute_server_client::{UserUid, ids::ServerId};
// Cute: session-sharing-protocol已删除，使用cute_server_client中定义的stub类型
use cute_server_client::drive::sharing::ProfileData;

/// Public struct for storing all the UserProfile data that's fed in from either sqlite or the server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserProfileWithUID {
    pub firebase_uid: UserUid,
    pub display_name: Option<String>,
    pub email: String,
    pub photo_url: String,
}

impl From<ProfileData> for UserProfileWithUID {
    fn from(data: ProfileData) -> Self {
        Self {
            firebase_uid: UserUid::new(&data.firebase_uid),
            display_name: data.display_name,
            email: data.email.unwrap_or_default(),
            photo_url: data.photo_url.unwrap_or_default(),
        }
    }
}

impl From<cute_graphql::user::PublicUserProfile> for UserProfileWithUID {
    fn from(value: cute_graphql::user::PublicUserProfile) -> Self {
        UserProfileWithUID {
            firebase_uid: UserUid::new(&value.uid),
            display_name: value.display_name,
            email: value.email.unwrap_or_default(),
            photo_url: value.photo_url.unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserProfileIdAndName {
    pub user_uid: UserUid,
    pub display_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TeamProfileIdAndName {
    pub team_uid: ServerId,
    pub display_name: String,
}
