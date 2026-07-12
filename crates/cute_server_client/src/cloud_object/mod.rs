use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use cute_graphql::object_permissions::AccessLevel;
use cute_graphql::scalars::time::ServerTimestamp;

use crate::auth::UserUid;
use crate::drive::sharing::{SharingAccessLevel, Subject, TeamKind, UserKind};
use crate::ids::{FolderId, GenericStringObjectId, ServerId, SyncId};

pub mod model;
pub mod models;
mod generic_string_model;

pub use generic_string_model::*;
/// The type of object id each ObjectType corresponds to.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ObjectIdType {
    Notebook,
    Workflow,
    Folder,
    GenericStringObject,
}

impl ObjectIdType {
    /// Returns the prefix for server IDs as we store them in sqlite. The prefix for these
    /// objects is in title case unlike how we store the object types, which is why two different
    /// APIs are needed.
    pub fn sqlite_prefix(&self) -> &'static str {
        match self {
            ObjectIdType::Notebook => "Notebook",
            ObjectIdType::Workflow => "Workflow",
            ObjectIdType::Folder => "Folder",
            ObjectIdType::GenericStringObject => "GenericStringObject",
        }
    }
}

/// A type for communicating the type of cloud object to/from the server, absent of the object itself.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize)]
pub enum ObjectType {
    Notebook,
    Workflow,
    Folder,
    EnvVarCollection,
    GenericStringObject(GenericStringObjectFormat),
}

impl ObjectType {
    /// Returns the serialized string for the object type, to be used for storing object_type in sqlite.
    pub fn sqlite_object_type_as_str(&self) -> Cow<'_, str> {
        match self {
            ObjectType::Notebook => "NOTEBOOK".into(),
            ObjectType::Workflow => "WORKFLOW".into(),
            ObjectType::Folder => "FOLDER".into(),
            ObjectType::EnvVarCollection => "ENV_VAR_COLLECTION".into(),
            ObjectType::GenericStringObject(format) => format.to_string().into(),
        }
    }
}

const NOTEBOOK_OBJECT_STRING: &str = "notebook";
const WORKFLOW_OBJECT_STRING: &str = "workflow";
const PROMPT_OBJECT_STRING: &str = "prompt";
const FOLDER_OBJECT_STRING: &str = "folder";
const ENV_VAR_COLLECTION_STRING: &str = "env-vars";

impl FromStr for ObjectType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            NOTEBOOK_OBJECT_STRING => Ok(Self::Notebook),
            WORKFLOW_OBJECT_STRING => Ok(Self::Workflow),
            PROMPT_OBJECT_STRING => Ok(Self::Workflow),
            FOLDER_OBJECT_STRING => Ok(Self::Folder),
            ENV_VAR_COLLECTION_STRING => Ok(Self::EnvVarCollection),
            _ => Err(anyhow!("Unexpected object type")),
        }
    }
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectType::Notebook => write!(f, "{NOTEBOOK_OBJECT_STRING}"),
            ObjectType::Workflow => write!(f, "{WORKFLOW_OBJECT_STRING}"),
            ObjectType::Folder => write!(f, "{FOLDER_OBJECT_STRING}"),
            ObjectType::GenericStringObject(GenericStringObjectFormat::Json(
                JsonObjectType::EnvVarCollection,
            )) => write!(f, "{ENV_VAR_COLLECTION_STRING}"),
            ObjectType::GenericStringObject(GenericStringObjectFormat::Json(
                JsonObjectType::AIFact,
            )) => write!(f, "rule"),
            ObjectType::GenericStringObject(_) => write!(f, "string_object_placeholder"), // placeholder value
            ObjectType::EnvVarCollection => write!(f, "{ENV_VAR_COLLECTION_STRING}"),
        }
    }
}

impl From<ObjectType> for ObjectIdType {
    fn from(value: ObjectType) -> Self {
        match value {
            ObjectType::Notebook => ObjectIdType::Notebook,
            ObjectType::Workflow => ObjectIdType::Workflow,
            ObjectType::Folder => ObjectIdType::Folder,
            ObjectType::GenericStringObject(_) => ObjectIdType::GenericStringObject,
            ObjectType::EnvVarCollection => ObjectIdType::GenericStringObject,
        }
    }
}

/// The object type prefix for generic string objects.
pub const GENERIC_STRING_OBJECT_PREFIX: &str = "GENERIC_STRING_";

/// The object type prefix for json objects.
pub const JSON_OBJECT_PREFIX: &str = "JSON_";

/// The data format for the generic string object type.
/// Right now we only support json, but this is left
/// open to support markdown, yaml and other text based types.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum GenericStringObjectFormat {
    Json(JsonObjectType),
}

/// Represents a unique key for a generic string object. The server enforces that
/// no two generic string objects have the same key.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GenericStringObjectUniqueKey {
    /// The unique key. E.g. for cloud prefs this is the storage key of the pref.
    pub key: String,

    /// Whether this key is unique for all generic string objects, or unique per user.
    pub unique_per: UniquePer,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum UniquePer {
    User,
}

// Temporarily suppress clippy warnings about the `ToString` impl until we
// move `ObjectType` away from using `std::fmt::Display` for serialization.
#[allow(clippy::to_string_trait_impl)]
impl ToString for GenericStringObjectFormat {
    fn to_string(&self) -> String {
        match self {
            GenericStringObjectFormat::Json(json_object_type) => format!(
                "{}{}{}",
                GENERIC_STRING_OBJECT_PREFIX,
                JSON_OBJECT_PREFIX,
                json_object_type.as_str()
            ),
        }
    }
}

/// An object sub-type for objects that implement the JsonModel trait.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum JsonObjectType {
    Preference,
    EnvVarCollection,
    WorkflowEnum,
    AIFact,
    MCPServer,
    AIExecutionProfile,
    TemplatableMCPServer,
    CloudEnvironment,
    ScheduledAmbientAgent,
    CloudAgentConfig,
}

impl JsonObjectType {
    pub fn as_str(&self) -> &'static str {
        match self {
            JsonObjectType::Preference => "PREFERENCE",
            JsonObjectType::EnvVarCollection => "ENVVARCOLLECTION",
            JsonObjectType::WorkflowEnum => "WORKFLOWENUM",
            JsonObjectType::AIFact => "AIFACT",
            JsonObjectType::MCPServer => "MCPSERVER",
            JsonObjectType::AIExecutionProfile => "AIEXECUTIONPROFILE",
            JsonObjectType::TemplatableMCPServer => "TEMPLATABLEMCPSERVER",
            JsonObjectType::CloudEnvironment => "CLOUDENVIRONMENT",
            JsonObjectType::ScheduledAmbientAgent => "SCHEDULEDAMBIENTAGENT",
            JsonObjectType::CloudAgentConfig => "CLOUDAGENTCONFIG",
        }
    }
}

impl TryFrom<&str> for JsonObjectType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "PREFERENCE" => Ok(JsonObjectType::Preference),
            "ENVVARCOLLECTION" => Ok(JsonObjectType::EnvVarCollection),
            "WORKFLOWENUM" => Ok(JsonObjectType::WorkflowEnum),
            "AIFACT" => Ok(JsonObjectType::AIFact),
            "MCPSERVER" => Ok(JsonObjectType::MCPServer),
            "AIEXECUTIONPROFILE" => Ok(JsonObjectType::AIExecutionProfile),
            "TEMPLATABLEMCPSERVER" => Ok(JsonObjectType::TemplatableMCPServer),
            "CLOUDENVIRONMENT" => Ok(JsonObjectType::CloudEnvironment),
            "SCHEDULEDAMBIENTAGENT" => Ok(JsonObjectType::ScheduledAmbientAgent),
            "CLOUDAGENTCONFIG" => Ok(JsonObjectType::CloudAgentConfig),
            _ => Err(anyhow!("could not convert unknown json object type")),
        }
    }
}

impl TryFrom<cute_graphql::object::ObjectType> for ObjectIdType {
    type Error = anyhow::Error;
    fn try_from(object_type: cute_graphql::object::ObjectType) -> Result<Self, Self::Error> {
        match object_type {
            cute_graphql::object::ObjectType::AIConversation => Err(anyhow!(
                "AIConversation is not a supported object type for this operation"
            )),
            cute_graphql::object::ObjectType::Notebook => Ok(ObjectIdType::Notebook),
            cute_graphql::object::ObjectType::Workflow => Ok(ObjectIdType::Workflow),
            cute_graphql::object::ObjectType::Folder => Ok(ObjectIdType::Folder),
            cute_graphql::object::ObjectType::GenericStringObject => {
                Ok(ObjectIdType::GenericStringObject)
            }
            cute_graphql::object::ObjectType::Unknown => {
                Err(anyhow!("could not convert unknown cloud object type"))
            }
        }
    }
}

impl From<ObjectType> for cute_graphql::object::ObjectType {
    fn from(value: ObjectType) -> Self {
        match value {
            ObjectType::Notebook => cute_graphql::object::ObjectType::Notebook,
            ObjectType::Workflow => cute_graphql::object::ObjectType::Workflow,
            ObjectType::Folder => cute_graphql::object::ObjectType::Folder,
            ObjectType::GenericStringObject(GenericStringObjectFormat::Json(
                JsonObjectType::EnvVarCollection,
            )) => cute_graphql::object::ObjectType::GenericStringObject,
            ObjectType::GenericStringObject(gso) => {
                todo!("Moving is not implemented for {:?}", gso);
            }
            ObjectType::EnvVarCollection => cute_graphql::object::ObjectType::GenericStringObject,
        }
    }
}

/// The revision timestamp at which an object was edited. This is used by the server
/// to determine if an edit to an object was at the latest revision. Edits at older
/// revisions are rejected by the server.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, PartialOrd, Ord)]
pub struct Revision(ServerTimestamp);

impl Revision {
    pub fn from_unix_timestamp_micros(ms_since_epoch: i64) -> Result<Self> {
        let ts = ServerTimestamp::from_unix_timestamp_micros(ms_since_epoch)?;
        Ok(Self(ts))
    }

    pub fn timestamp_micros(&self) -> i64 {
        self.0.timestamp_micros()
    }

    pub fn utc(&self) -> DateTime<Utc> {
        self.0.utc()
    }

    /// Returns the inner `ServerTimestamp`.
    pub fn timestamp(&self) -> ServerTimestamp {
        self.0
    }

    #[cfg(any(test, feature = "test-util"))]
    pub fn now() -> Self {
        Self(ServerTimestamp::new(Utc::now()))
    }
}

impl From<Revision> for ServerTimestamp {
    fn from(revision: Revision) -> Self {
        revision.0
    }
}

impl From<ServerTimestamp> for Revision {
    fn from(time: ServerTimestamp) -> Self {
        Revision(time)
    }
}

#[cfg(any(test, feature = "test-util"))]
impl From<DateTime<Utc>> for Revision {
    fn from(time: DateTime<Utc>) -> Self {
        Self(ServerTimestamp::new(time))
    }
}

/// The owner for a given object.
#[derive(Copy, Clone, Debug, Eq, Serialize, Deserialize, Derivative)]
#[derivative(PartialEq)]
pub enum Owner {
    /// The owner of the object is a user (the object is in their personal drive).
    User { user_uid: UserUid },
    /// The owner of the object is a team (the object is in a team drive).
    Team { team_uid: ServerId },
}

impl Default for Owner {
    fn default() -> Self {
        Owner::User {
            user_uid: UserUid::default(),
        }
    }
}

impl Owner {
    /// A mock [`Owner`] ID for testing.
    #[cfg(any(test, feature = "test-util"))]
    pub fn mock_current_user() -> Owner {
        use crate::auth::TEST_USER_UID;

        Owner::User {
            user_uid: UserUid::new(TEST_USER_UID),
        }
    }
}

impl From<Owner> for Option<ServerId> {
    fn from(owner: Owner) -> Option<ServerId> {
        match owner {
            Owner::User { .. } => None,
            Owner::Team { team_uid, .. } => Some(team_uid),
        }
    }
}

/// Server representation of an object's container. This corresponds to the `Container` GraphQL
/// type.
///
/// Containers are similar to, but not quite the same as, the [`CloudObjectLocation`] type.
/// Locations depend on object and user state - an object might currently be in the trash, or
/// it could be in one user's [shared space](Space::Shared) but another's
/// [team space](Space::Team). Containers, on the other hand, represent an object's canonical
/// parent - its one parent folder or drive that permissions are inherited from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServerObjectContainer {
    Folder { folder_uid: ServerId },
    Drive { owner: Owner },
}

/// Server representation of a user object guest, as part of [`ServerObjectGuest`].
#[derive(Clone, Debug, PartialEq)]
pub enum ServerGuestSubject {
    User { firebase_uid: String },
    PendingUser { email: Option<String> },
    Team { team_uid: ServerId },
}

/// Server representation of a link-sharing setting.
#[derive(Clone, Debug, PartialEq)]
pub struct ServerLinkSharing {
    pub access_level: AccessLevel,
    pub source: Option<ServerObjectContainer>,
}

/// Server representation of an object guest. This corresponds to the `ObjectGuest` GraphQL type.
#[derive(Clone, Debug, PartialEq)]
pub struct ServerObjectGuest {
    pub subject: ServerGuestSubject,
    pub access_level: AccessLevel,
    /// If this guest is inherited, this is the ancestor that it's inherited from.
    pub source: Option<ServerObjectContainer>,
}

/// Metadata for a cloud object that was fetched from the server.
#[derive(Clone, Debug)]
pub struct ServerMetadata {
    pub uid: ServerId,
    pub revision: Revision,
    pub metadata_last_updated_ts: ServerTimestamp,
    pub trashed_ts: Option<ServerTimestamp>,
    pub folder_id: Option<FolderId>,
    pub is_welcome_object: bool,
    pub creator_uid: Option<String>,
    pub last_editor_uid: Option<String>,
    pub current_editor_uid: Option<String>,
}

/// Permissions for a cloud object that was fetched from the server.
#[derive(Clone, Debug, PartialEq)]
pub struct ServerPermissions {
    /// The GraphQL definition of a `Space` is closer to the client's definition of an `Owner` (due
    /// to sharing). This is also going to migrate back to [ServerMetadata] as part of the
    /// `Container` migration.
    pub space: Owner,
    pub guests: Vec<ServerObjectGuest>,
    pub anyone_link_sharing: Option<ServerLinkSharing>,
    pub permissions_last_updated_ts: ServerTimestamp,
}

impl ServerPermissions {
    #[cfg(any(test, feature = "test-util"))]
    pub fn mock_personal() -> Self {
        Self {
            space: Owner::mock_current_user(),
            guests: Vec::new(),
            anyone_link_sharing: None,
            permissions_last_updated_ts: DateTime::<Utc>::default().into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NumInFlightRequests(pub usize);

#[derive(Clone, Debug)]
/// An enum representing what state a local cloud object's content changes can be in,
/// in relation to the server.
pub enum CloudObjectSyncStatus {
    /// The object's content hasn't changed from what we believe the server's representation
    /// to be.
    NoLocalChanges,
    /// The object's content has been modified locally, and is currently in the sync queue
    /// attempting to sync up with the server.
    InFlight(NumInFlightRequests),
    /// The object's content has been modified locally but has unresolved conflict with the server
    /// revision.
    InConflict,
    /// The object's content has been modified locally, but persisting the change on the server
    /// could not complete for some reason.
    Errored,
}



#[derive(Debug, Clone, PartialEq)]
pub struct CloudObjectPermissions {
    pub owner: Owner,
    pub permissions_last_updated_ts: Option<ServerTimestamp>,
    pub anyone_with_link: Option<CloudLinkSharing>,
    pub guests: Vec<CloudObjectGuest>,
}

impl CloudObjectPermissions {
    pub fn new_from_server(server_permissions: ServerPermissions) -> Self {
        let guests = if false {
            server_permissions
                .guests
                .into_iter()
                .map(CloudObjectGuest::from_server)
                .collect()
        } else {
            Vec::new()
        };

        let anyone_with_link = if false {
            server_permissions
                .anyone_link_sharing
                .map(CloudLinkSharing::from_server)
        } else {
            None
        };

        Self {
            owner: server_permissions.space,
            permissions_last_updated_ts: Some(server_permissions.permissions_last_updated_ts),
            guests,
            anyone_with_link,
        }
    }

    /// Mock permissions for a personal object.
    #[cfg(any(test, feature = "test-util"))]
    pub fn mock_personal() -> Self {
        Self {
            owner: Owner::mock_current_user(),
            permissions_last_updated_ts: Some(Utc::now().into()),
            guests: Vec::new(),
            anyone_with_link: None,
        }
    }

    /// Returns `true` if the given user has direct personal access to this object —
    /// either via an explicit user guest ACL entry or via link sharing.
    /// Returns `false` if the only access is through a team guest ACL.
    pub fn has_direct_user_access(&self, user_uid: UserUid) -> bool {
        self.anyone_with_link.is_some() || self.guests.iter().any(|g| g.subject.is_user(user_uid))
    }

    /// Updates self from new permissions information received from the server
    pub fn update_from_new_permissions_ts(&mut self, server_permissions: ServerPermissions) {
        self.owner = server_permissions.space;
        self.permissions_last_updated_ts = Some(server_permissions.permissions_last_updated_ts);
        if false {
            self.guests = server_permissions
                .guests
                .into_iter()
                .map(CloudObjectGuest::from_server)
                .collect();
            self.anyone_with_link = server_permissions
                .anyone_link_sharing
                .map(CloudLinkSharing::from_server);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CloudLinkSharing {
    pub access_level: SharingAccessLevel,
    // If this sharing setting was inherited, the `source` identifies the container it's inherited
    // from.
    pub source: Option<ServerObjectContainer>,
}

impl CloudLinkSharing {
    pub fn from_server(server_link_sharing: ServerLinkSharing) -> Self {
        Self {
            access_level: server_link_sharing.access_level.into(),
            source: server_link_sharing.source,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CloudObjectGuest {
    pub subject: Subject,
    pub access_level: SharingAccessLevel,
    /// If this guest was added to a container object, the `source` identifies that object.
    pub source: Option<ServerObjectContainer>,
}

impl CloudObjectGuest {
    pub fn from_server(server_guest: ServerObjectGuest) -> Self {
        let subject = match server_guest.subject {
            ServerGuestSubject::User { firebase_uid } => {
                Subject::User(UserKind::Account(UserUid::new(&firebase_uid)))
            }
            ServerGuestSubject::PendingUser { email } => Subject::PendingUser { email },
            ServerGuestSubject::Team { team_uid } => Subject::Team(TeamKind::Team { team_uid }),
        };

        Self {
            subject,
            access_level: server_guest.access_level.into(),
            source: server_guest.source,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CloudObjectMetadata {
    pub revision: Option<Revision>,
    pub metadata_last_updated_ts: Option<ServerTimestamp>,
    pub current_editor_uid: Option<String>,
    pub pending_changes_statuses: CloudObjectStatuses,
    pub trashed_ts: Option<ServerTimestamp>,
    pub folder_id: Option<SyncId>,
    /// Welcome objects are created on the server when a user first receives
    /// access to Warp Drive as part of onboarding.
    pub is_welcome_object: bool,
    pub last_editor_uid: Option<String>,
    pub creator_uid: Option<String>,
    /// The "last used" timestamp for this environment.
    ///
    /// This is populated via `GetCloudEnvironments` from
    /// `CloudEnvironment.lastTaskCreated.createdAt`.
    /// Only applicable for CloudEnvironment objects.
    pub last_task_run_ts: Option<ServerTimestamp>,
}

impl CloudObjectMetadata {
    pub fn new_from_server(server_metadata: ServerMetadata) -> Self {
        Self {
            revision: Some(server_metadata.revision),
            current_editor_uid: server_metadata.current_editor_uid,
            metadata_last_updated_ts: Some(server_metadata.metadata_last_updated_ts),
            pending_changes_statuses: CloudObjectStatuses {
                content_sync_status: CloudObjectSyncStatus::NoLocalChanges,
                has_pending_metadata_change: false,
                has_pending_permissions_change: false,
                pending_untrash: false,
                pending_delete: false,
            },
            trashed_ts: server_metadata.trashed_ts,
            folder_id: server_metadata.folder_id.map(|id| id.into()),
            is_welcome_object: server_metadata.is_welcome_object,
            creator_uid: server_metadata.creator_uid,
            last_editor_uid: server_metadata.last_editor_uid,
            // last_task_run_ts is populated separately via GetCloudEnvironments query
            last_task_run_ts: None,
        }
    }

    /// Creates a new set of metadata with reasonable defaults for a test:
    /// * Content and metadata timestamps set to now
    /// * No editor information
    /// * No parent folder
    /// * Not trashed
    #[cfg(any(test, feature = "test-util"))]
    pub fn mock() -> Self {
        Self {
            revision: Some(Revision::now()),
            current_editor_uid: None,
            metadata_last_updated_ts: Some(Utc::now().into()),
            pending_changes_statuses: CloudObjectStatuses::mock(),
            trashed_ts: None,
            folder_id: None,
            is_welcome_object: false,
            last_editor_uid: None,
            creator_uid: None,
            last_task_run_ts: None,
        }
    }

    pub fn has_pending_content_changes(&self) -> bool {
        !matches!(
            self.pending_changes_statuses.content_sync_status,
            CloudObjectSyncStatus::NoLocalChanges | CloudObjectSyncStatus::InConflict
        )
    }

    pub fn is_errored(&self) -> bool {
        matches!(
            self.pending_changes_statuses.content_sync_status,
            CloudObjectSyncStatus::Errored
        )
    }

    /// True iff there are unsynced online-only changes for the object.
    pub fn has_pending_online_only_change(&self) -> bool {
        self.pending_changes_statuses.has_pending_permissions_change
            || self.pending_changes_statuses.has_pending_metadata_change
            || self.pending_changes_statuses.pending_untrash
            || self.pending_changes_statuses.pending_delete
    }

    pub fn set_current_editor(&mut self, editor_uid: Option<String>) {
        self.current_editor_uid = editor_uid;
    }

    /// Updates revision and last_editor_uid from server metadata.
    ///
    /// This unconditionally updates the revision and last_editor_uid, even if
    /// there are conflicts, so callers should check for conflicts before calling
    /// this.
    pub fn update_revision_from_server(&mut self, server_metadata: &ServerMetadata) {
        self.revision = Some(server_metadata.revision.clone());
        self.last_editor_uid = server_metadata.last_editor_uid.clone();
    }

    /// Updates self from a new metadata received from the server
    pub fn update_from_new_metadata_ts(&mut self, server_metadata: ServerMetadata) {
        // Overwriting the metadata from an MetadataUpdated RTC message shouldn't overwrite
        // the versioning of the object's data: the revision timestamp, has_pending_changes, conflict_status
        // (if the object data is not being updated, the data versioning should stay the same.
        self.current_editor_uid = server_metadata.current_editor_uid;
        self.trashed_ts = server_metadata.trashed_ts;
        self.folder_id = server_metadata.folder_id.map(|folder_id| folder_id.into());
        self.creator_uid = server_metadata.creator_uid;
        self.metadata_last_updated_ts = Some(server_metadata.metadata_last_updated_ts);
    }
}

/// A struct holding the different statuses of pending changes that a cloud object might have.
/// Note that content is handled differently than permissions/metadata:
///   * Content changes go through the sync queue, and thus can exist in more states
///   * Metadata/permissions changes are synchronous operations, and thus are only either
///     in flight or synced
#[derive(Clone, Debug)]
pub struct CloudObjectStatuses {
    pub content_sync_status: CloudObjectSyncStatus,
    /// True iff there are unsynced permission changes for the object.
    /// We intentionally don't persist this value in sqlite. And if true,
    /// we don't upsert any in-memory permission changes to sqlite.
    pub has_pending_permissions_change: bool,
    /// True iff there are unsynced metadata changes for the object.
    /// We intentionally don't persist this value in sqlite. And if true,
    /// we don't upsert trashed and folder changes to sqlite.
    pub has_pending_metadata_change: bool,

    /// True iff there is an unsynced untrash operation on the object.
    pub pending_untrash: bool,

    /// True iff there is an unsynced delete operation on the object.
    pub pending_delete: bool,
}

impl CloudObjectStatuses {
    /// Empty statuses with no in-flight changes, for use in tests.
    #[cfg(any(test, feature = "test-util"))]
    pub fn mock() -> Self {
        Self {
            content_sync_status: CloudObjectSyncStatus::NoLocalChanges,
            has_pending_permissions_change: false,
            has_pending_metadata_change: false,
            pending_untrash: false,
            pending_delete: false,
        }
    }


}

// Used for event tracking purposes, matches
// up with GraphQL enum of the same name.
#[derive(Copy, Default, Clone, Debug, Eq, PartialEq)]
pub enum CloudObjectEventEntrypoint {
    TeamSettings,
    ResourceCenter,
    UniversalSearch,
    ManagementUI,
    Blocklist,
    ImportModal,
    Onboarding,
    #[default]
    Unknown,
}

// A newtype for a serialized model that wraps a plain string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerializedModel(String);

/// A trait for models that represent server objects.
pub trait ServerObjectModel: std::fmt::Debug + Clone + Send + Sync + 'static {
    fn object_type(&self) -> ObjectType;
}

/// Status of conflicts for cloud objects.
#[derive(Clone, Debug, Default)]
pub enum ConflictStatus<T> {
    /// No conflicts detected.
    #[default]
    NoConflicts,
    /// There are conflicting changes with the server version.
    ConflictingChanges { object: std::sync::Arc<T> },
}

impl<T> ConflictStatus<T> {
    /// Returns true if there are conflicts.
    pub fn has_conflicts(&self) -> bool {
        matches!(self, ConflictStatus::ConflictingChanges { .. })
    }
}

impl SerializedModel {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn model_as_str(&self) -> &str {
        &self.0
    }

    pub fn take(self) -> String {
        self.0
    }
}

impl From<String> for SerializedModel {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RevisionAndLastEditor {
    pub revision: Revision,
    pub last_editor_uid: Option<String>,
}

// GraphQL conversion impls.

impl From<GenericStringObjectFormat>
    for cute_graphql::generic_string_object::GenericStringObjectFormat
{
    fn from(format: GenericStringObjectFormat) -> Self {
        use cute_graphql::generic_string_object::GenericStringObjectFormat as GraphQLFormat;
        match format {
            GenericStringObjectFormat::Json(JsonObjectType::Preference) => {
                GraphQLFormat::JsonPreference
            }
            GenericStringObjectFormat::Json(JsonObjectType::EnvVarCollection) => {
                GraphQLFormat::JsonEnvVarCollection
            }
            GenericStringObjectFormat::Json(JsonObjectType::WorkflowEnum) => {
                GraphQLFormat::JsonWorkflowEnum
            }
            GenericStringObjectFormat::Json(JsonObjectType::AIFact) => GraphQLFormat::JsonAIFact,
            GenericStringObjectFormat::Json(JsonObjectType::MCPServer) => {
                GraphQLFormat::JsonMCPServer
            }
            GenericStringObjectFormat::Json(JsonObjectType::AIExecutionProfile) => {
                GraphQLFormat::JsonAIExecutionProfile
            }
            GenericStringObjectFormat::Json(JsonObjectType::TemplatableMCPServer) => {
                GraphQLFormat::JsonTemplatableMCPServer
            }
            GenericStringObjectFormat::Json(JsonObjectType::CloudEnvironment) => {
                GraphQLFormat::JsonCloudEnvironment
            }
            GenericStringObjectFormat::Json(JsonObjectType::ScheduledAmbientAgent) => {
                GraphQLFormat::JsonScheduledAmbientAgent
            }
            GenericStringObjectFormat::Json(JsonObjectType::CloudAgentConfig) => {
                unreachable!("JsonCloudAgentConfig is no longer present in GraphQL schema")
            }
        }
    }
}

impl From<CloudObjectEventEntrypoint> for cute_graphql::object::CloudObjectEventEntrypoint {
    fn from(entrypoint: CloudObjectEventEntrypoint) -> Self {
        use cute_graphql::object::CloudObjectEventEntrypoint as GraphQLEntrypoint;
        match entrypoint {
            CloudObjectEventEntrypoint::TeamSettings => GraphQLEntrypoint::TeamSettings,
            CloudObjectEventEntrypoint::ResourceCenter => GraphQLEntrypoint::ResourceCenter,
            CloudObjectEventEntrypoint::UniversalSearch => GraphQLEntrypoint::UniversalSearch,
            CloudObjectEventEntrypoint::ManagementUI => GraphQLEntrypoint::DriveIndex,
            CloudObjectEventEntrypoint::Blocklist => GraphQLEntrypoint::Blocklist,
            CloudObjectEventEntrypoint::ImportModal => GraphQLEntrypoint::ImportModal,
            CloudObjectEventEntrypoint::Onboarding => GraphQLEntrypoint::Onboarding,
            CloudObjectEventEntrypoint::Unknown => GraphQLEntrypoint::Unknown,
        }
    }
}

impl From<GenericStringObjectUniqueKey>
    for cute_graphql::generic_string_object::GenericStringObjectUniqueKey
{
    fn from(key: GenericStringObjectUniqueKey) -> Self {
        use cute_graphql::generic_string_object::GenericStringObjectUniqueKey as GraphQLKey;
        GraphQLKey {
            key: key.key,
            unique_per: key.unique_per.into(),
        }
    }
}

impl From<UniquePer> for cute_graphql::generic_string_object::UniquePer {
    fn from(unique_per: UniquePer) -> Self {
        use cute_graphql::generic_string_object::UniquePer as GraphQLUniquePer;
        match unique_per {
            UniquePer::User => GraphQLUniquePer::User,
        }
    }
}

impl TryFrom<cute_graphql::object::ObjectMetadata> for ServerMetadata {
    type Error = anyhow::Error;

    fn try_from(value: cute_graphql::object::ObjectMetadata) -> Result<Self, Self::Error> {
        let folder_id: Option<FolderId> = match value.parent {
            cute_graphql::object::Container::FolderContainer(folder_container) => {
                Some(folder_container.folder_uid.into_inner().into())
            }
            _ => None,
        };
        let metadata = ServerMetadata {
            uid: ServerId::from_string_lossy(value.uid.inner()),
            revision: value.revision_ts.into(),
            metadata_last_updated_ts: value.metadata_last_updated_ts,
            trashed_ts: value.trashed_ts,
            folder_id,
            is_welcome_object: value.is_welcome_object,
            creator_uid: value.creator_uid.map(|uid| uid.into_inner()),
            last_editor_uid: value.last_editor_uid.map(|uid| uid.into_inner()),
            current_editor_uid: value.current_editor_uid.map(|uid| uid.into_inner()),
        };
        Ok(metadata)
    }
}

impl TryFrom<cute_graphql::object_permissions::ObjectPermissions> for ServerPermissions {
    type Error = anyhow::Error;

    fn try_from(
        value: cute_graphql::object_permissions::ObjectPermissions,
    ) -> Result<Self, Self::Error> {
        let server_object_guests: Result<Vec<ServerObjectGuest>, _> = value
            .guests
            .into_iter()
            .map(|guest| guest.try_into())
            .collect();
        let object_permissions = ServerPermissions {
            space: value.space.try_into()?,
            guests: server_object_guests?,
            anyone_link_sharing: match value.anyone_link_sharing {
                Some(sharing) => Some(sharing.try_into()?),
                None => None,
            },
            permissions_last_updated_ts: value.last_updated_ts,
        };
        Ok(object_permissions)
    }
}

impl TryFrom<cute_graphql::object_permissions::ObjectGuest> for ServerObjectGuest {
    type Error = anyhow::Error;

    fn try_from(value: cute_graphql::object_permissions::ObjectGuest) -> Result<Self, Self::Error> {
        let object_guest = ServerObjectGuest {
            subject: value.subject.try_into()?,
            access_level: value.access_level,
            source: match value.source {
                Some(container) => Some(container.try_into()?),
                None => None,
            },
        };
        Ok(object_guest)
    }
}

impl TryFrom<cute_graphql::object_permissions::GuestSubject> for ServerGuestSubject {
    type Error = anyhow::Error;

    fn try_from(
        value: cute_graphql::object_permissions::GuestSubject,
    ) -> Result<Self, Self::Error> {
        match value {
            cute_graphql::object_permissions::GuestSubject::UserGuest(user_guest) => {
                let guest_subject = ServerGuestSubject::User {
                    firebase_uid: user_guest.firebase_uid.into_inner(),
                };
                Ok(guest_subject)
            }
            cute_graphql::object_permissions::GuestSubject::PendingUserGuest(guest) => {
                Ok(ServerGuestSubject::PendingUser { email: guest.email })
            }
            cute_graphql::object_permissions::GuestSubject::TeamGuest(team_guest) => {
                Ok(ServerGuestSubject::Team {
                    team_uid: ServerId::from_string_lossy(team_guest.uid.inner()),
                })
            }
            cute_graphql::object_permissions::GuestSubject::Unknown => {
                anyhow::bail!("Unknown GuestSubject type")
            }
        }
    }
}

impl TryFrom<cute_graphql::object_permissions::LinkSharing> for ServerLinkSharing {
    type Error = anyhow::Error;

    fn try_from(value: cute_graphql::object_permissions::LinkSharing) -> Result<Self, Self::Error> {
        Ok(ServerLinkSharing {
            access_level: value.access_level,
            source: value.source.map(TryInto::try_into).transpose()?,
        })
    }
}

impl TryFrom<cute_graphql::object::Container> for ServerObjectContainer {
    type Error = anyhow::Error;

    fn try_from(value: cute_graphql::object::Container) -> Result<Self, Self::Error> {
        match value {
            cute_graphql::object::Container::FolderContainer(folder) => {
                Ok(ServerObjectContainer::Folder {
                    folder_uid: ServerId::from_string_lossy(folder.folder_uid.inner()),
                })
            }
            cute_graphql::object::Container::Space(space) => Ok(ServerObjectContainer::Drive {
                owner: space.try_into()?,
            }),
            cute_graphql::object::Container::Unknown => {
                anyhow::bail!("Unknown Container type")
            }
        }
    }
}

impl TryFrom<cute_graphql::object::Space> for Owner {
    type Error = anyhow::Error;

    fn try_from(value: cute_graphql::object::Space) -> Result<Self, Self::Error> {
        let owner = match value.type_ {
            cute_graphql::object::SpaceType::Team => Owner::Team {
                team_uid: ServerId::from_string_lossy(value.uid.inner()),
            },
            cute_graphql::object::SpaceType::User => Owner::User {
                user_uid: UserUid::new(value.uid.inner()),
            },
        };
        Ok(owner)
    }
}

impl From<Owner> for cute_graphql::object_permissions::Owner {
    fn from(owner: Owner) -> Self {
        use cute_graphql::object_permissions::{Owner as GraphQLOwner, OwnerType};
        match owner {
            Owner::User { user_uid } => GraphQLOwner {
                type_: OwnerType::User,
                uid: Some(cynic::Id::new(user_uid.to_string())),
            },
            Owner::Team { team_uid, .. } => GraphQLOwner {
                type_: OwnerType::Team,
                uid: Some(cynic::Id::new(team_uid)),
            },
        }
    }
}

// ===== Cloud Object Types (restored signatures) =====
// Type definitions restored from the original cloud_object submodules so that signatures
// (fields, variants, generics) match what the app expects. Network/sync methods remain
// stubbed (return Err) at the CloudModelType trait level.

use std::marker::PhantomData;
use std::sync::Arc;

use crate::ids::{ClientId, ServerIdAndType};

/// A portable payload for persisting or otherwise upserting a cloud object without app-local
/// event types.
#[derive(Clone, Debug)]
pub struct CloudObjectUpsertParams<M> {
    pub id: SyncId,
    pub object_type: ObjectType,
    pub metadata: CloudObjectMetadata,
    pub permissions: CloudObjectPermissions,
    pub model: M,
}

/// An object that maps directly to the data returned from the server for a given model and id
/// type.
pub struct GenericServerObject<K, M> {
    pub id: SyncId,
    pub model: M,
    pub metadata: ServerMetadata,
    pub permissions: ServerPermissions,
    _marker: PhantomData<fn() -> K>,
}

impl<K, M> Clone for GenericServerObject<K, M>
where
    M: Clone,
{
    fn clone(&self) -> Self {
        Self::new(
            self.id,
            self.model.clone(),
            self.metadata.clone(),
            self.permissions.clone(),
        )
    }
}

impl<K, M> std::fmt::Debug for GenericServerObject<K, M>
where
    M: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericServerObject")
            .field("id", &self.id)
            .field("model", &self.model)
            .field("metadata", &self.metadata)
            .field("permissions", &self.permissions)
            .finish()
    }
}

impl<K, M> GenericServerObject<K, M> {
    /// Constructs a server object from its server-provided parts.
    pub fn new(
        id: SyncId,
        model: M,
        metadata: ServerMetadata,
        permissions: ServerPermissions,
    ) -> Self {
        Self {
            id,
            model,
            metadata,
            permissions,
            _marker: PhantomData,
        }
    }

    /// Gets a reference to the model held by the object.
    pub fn model(&self) -> &M {
        &self.model
    }
}

/// A generic implementation of cloud objects that can be used for any model and id types.
///
/// Rather than directly implementing the CloudObject trait, CloudObjects can implement
/// GenericCloudObject<K, M> where K is their id type and M is their model type.
#[derive(Clone, Debug)]
pub struct GenericCloudObject<K, M> {
    pub id: SyncId,
    pub metadata: CloudObjectMetadata,
    pub permissions: CloudObjectPermissions,
    /// Tracks whether this object has a conflict with the server version. Runtime state.
    pub conflict_status: ConflictStatus<GenericServerObject<K, M>>,
    // Intentionally private to prevent holding references outside this struct. Arc enables
    // clone-on-write semantics for the model.
    model: Arc<M>,
}

impl<K, M> PartialEq for GenericCloudObject<K, M>
where
    M: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.model() == other.model()
    }
}

impl<K, M> GenericCloudObject<K, M> {
    /// Gets a reference to the model held by the object.
    pub fn model(&self) -> &M {
        &self.model
    }

    /// Returns a shared handle to the model.
    pub fn shared_model(&self) -> Arc<M> {
        self.model.clone()
    }

    /// Sets a new version of the model on the object, replacing the old version.
    pub fn set_model(&mut self, model: M) {
        self.model = model.into();
    }

    /// Constructs a new instance with the given id, model, metadata and permissions.
    pub fn new(
        id: SyncId,
        model: M,
        metadata: CloudObjectMetadata,
        permissions: CloudObjectPermissions,
    ) -> Self {
        Self {
            id,
            model: model.into(),
            metadata,
            permissions,
            conflict_status: ConflictStatus::NoConflicts,
        }
    }

    /// Creates a new GenericCloudObject with the given model, owner, and initial folder id.
    /// This is for the local creation flow, as opposed to creating from a server update.
    pub fn new_local(
        model: M,
        owner: Owner,
        initial_folder_id: Option<SyncId>,
        client_id: ClientId,
    ) -> Self {
        Self {
            id: SyncId::ClientId(client_id),
            model: model.into(),
            metadata: CloudObjectMetadata {
                pending_changes_statuses: CloudObjectStatuses {
                    content_sync_status: CloudObjectSyncStatus::InFlight(NumInFlightRequests(1)),
                    has_pending_metadata_change: false,
                    has_pending_permissions_change: false,
                    pending_untrash: false,
                    pending_delete: false,
                },
                folder_id: initial_folder_id,
                revision: Default::default(),
                metadata_last_updated_ts: Default::default(),
                current_editor_uid: Default::default(),
                trashed_ts: Default::default(),
                is_welcome_object: false,
                creator_uid: None,
                last_editor_uid: None,
                last_task_run_ts: None,
            },
            permissions: CloudObjectPermissions {
                owner,
                anyone_with_link: None,
                guests: Default::default(),
                permissions_last_updated_ts: None,
            },
            conflict_status: ConflictStatus::NoConflicts,
        }
    }

    /// Creates a new [`GenericCloudObject`] from a [`GenericServerObject`].
    pub fn new_from_server(server_object: GenericServerObject<K, M>) -> Self {
        Self {
            id: server_object.id,
            model: server_object.model.into(),
            metadata: CloudObjectMetadata::new_from_server(server_object.metadata),
            permissions: CloudObjectPermissions::new_from_server(server_object.permissions),
            conflict_status: ConflictStatus::NoConflicts,
        }
    }

    /// Marks this object as being in conflict with the provided object.
    pub fn set_conflicting_object(&mut self, object: Arc<GenericServerObject<K, M>>) {
        self.conflict_status = ConflictStatus::ConflictingChanges { object };
    }

    pub fn update_from_server_object(&mut self, server_object: GenericServerObject<K, M>) {
        if self.metadata.has_pending_content_changes() || self.conflict_status.has_conflicts() {
            self.conflict_status = ConflictStatus::ConflictingChanges {
                object: Arc::new(server_object),
            };
        } else {
            self.metadata
                .update_revision_from_server(&server_object.metadata);
            self.model = server_object.model.into();
            self.conflict_status = ConflictStatus::NoConflicts;
        }
    }

    /// Returns portable upsert parameters for this object.
    pub fn upsert_params(&self, object_type: ObjectType) -> CloudObjectUpsertParams<M>
    where
        M: Clone,
    {
        CloudObjectUpsertParams {
            id: self.id,
            object_type,
            metadata: self.metadata.clone(),
            permissions: self.permissions.clone(),
            model: self.model().clone(),
        }
    }

    /// Converts this object into portable upsert parameters.
    pub fn into_upsert_params(self, object_type: ObjectType) -> CloudObjectUpsertParams<M>
    where
        M: Clone,
    {
        let Self {
            id,
            metadata,
            permissions,
            model,
            conflict_status: _,
        } = self;
        let model = Arc::try_unwrap(model).unwrap_or_else(|model| (*model).clone());
        CloudObjectUpsertParams {
            id,
            object_type,
            metadata,
            permissions,
            model,
        }
    }
}

impl<K, M> From<CloudObjectUpsertParams<M>> for GenericCloudObject<K, M> {
    fn from(params: CloudObjectUpsertParams<M>) -> Self {
        Self::new(params.id, params.model, params.metadata, params.permissions)
    }
}

// ===== Creation / Update types (from creation.rs / update.rs) =====
// Only the data shapes are restored; network send_* methods live on the CloudModelType trait
// where they are already stubbed to return Err.

/// Helper struct that contains all the info needed to create an object on the server.
#[derive(Clone, Debug)]
pub struct CreateObjectRequest {
    pub serialized_model: Option<SerializedModel>,
    pub title: Option<String>,
    pub owner: Owner,
    pub client_id: ClientId,
    pub initial_folder_id: Option<FolderId>,
    pub entrypoint: CloudObjectEventEntrypoint,
}

/// The data returned by the server when an object is created, generic to any object type.
#[derive(Debug, Clone)]
pub struct CreatedCloudObject {
    pub client_id: ClientId,
    pub revision_and_editor: RevisionAndLastEditor,
    pub metadata_ts: ServerTimestamp,
    pub server_id_and_type: ServerIdAndType,
    pub creator_uid: Option<String>,
    pub permissions: ServerPermissions,
}

/// Result of attempting to create a cloud object.
#[derive(Debug, Clone)]
pub enum CreateCloudObjectResult {
    /// The object creation was successful.
    Success {
        created_cloud_object: CreatedCloudObject,
    },
    /// The object creation was denied due to an expected user error.
    UserFacingError(String),
    /// The object creation was rejected because the generic string object had already been
    /// created by another client.
    GenericStringObjectUniqueKeyConflict,
}

/// Result of attempting to update a cloud object.
#[derive(Debug)]
pub enum UpdateCloudObjectResult<T> {
    /// The update was successful and the object now has the specified revision.
    Success {
        revision_and_editor: RevisionAndLastEditor,
    },
    /// The update was rejected because the update was not sent from the current revision in
    /// storage. The object and revision in storage are returned.
    Rejected { object: T },
}
