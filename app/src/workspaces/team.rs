// Simplified: team feature removed
use serde::{Deserialize, Serialize};
use crate::server::ids::ServerId;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Team {
    pub uid: ServerId,
    pub name: String,
    pub members: Vec<TeamMember>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TeamMember {
    pub uid: ServerId,
    pub name: String,
    pub role: MembershipRole,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum MembershipRole {
    #[default]
    Member,
    Admin,
    Owner,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DiscoverableTeam {
    pub uid: ServerId,
    pub name: String,
}
