use crate::anyhow;
use crate::cloud_stub_types::model::actions::{ObjectAction, ObjectActionHistory, ObjectActionSubtype, ObjectActionType};
use crate::server::ids::{HashedSqliteId, ObjectUid, ServerId, SyncId};

/// Converts the graphql action history type into an ObjectActionHistory, requires converting
/// the individual actions, action types, and action subtypes.
#[allow(dead_code)]
pub fn object_action_history_from_gql(
    history: cute_graphql::object_actions::ObjectActionHistory,
) -> Result<ObjectActionHistory, anyhow::Error> {
    let uid: ObjectUid = history.uid.into_inner();
    let sync_id = SyncId::ServerId(ServerId::from_string_lossy(&uid));
    let hashed_sqlite_id = sync_id.sqlite_uid_hash(history.object_type.try_into()?);

    let actions = history
        .actions
        .map(|actions| {
            actions
                .iter()
                .filter_map(|action| {
                    try_into_object_action(action, uid.clone(), hashed_sqlite_id.clone()).ok()
                })
                .collect::<Vec<ObjectAction>>()
        })
        .unwrap_or_default();

    Ok(ObjectActionHistory {
        uid,
        hashed_sqlite_id,
        latest_processed_at_timestamp: history
            .latest_processed_at_timestamp
            .ok_or(anyhow!(
                "Parsing error: latest processed at timestamp did not exist."
            ))?
            .utc(),
        actions,
    })
}

/// Converts the graphql action type ("EXECUTED", etc) to ObjectActionType.
#[allow(dead_code)]
fn try_into_object_action_type(
    action_type: cute_graphql::object_actions::ActionType,
) -> Result<ObjectActionType, anyhow::Error> {
    match action_type {
        cute_graphql::object_actions::ActionType::Executed => Ok(ObjectActionType::Execute),
    }
}

/// Converts the graphql action entry (SingleAction, BundledActions) into its ObjectAction corollary.
#[allow(dead_code)]
fn try_into_object_action(
    record: &cute_graphql::object_actions::ActionRecord,
    uid: ObjectUid,
    hashed_sqlite_id: HashedSqliteId,
) -> Result<ObjectAction, anyhow::Error> {
    match record {
        cute_graphql::object_actions::ActionRecord::SingleAction(s) => Ok(ObjectAction {
            action_type: try_into_object_action_type(s.action_type)?,
            action_subtype: ObjectActionSubtype::SingleAction {
                timestamp: s.timestamp.utc(),
                processed_at_timestamp: Some(s.processed_at_timestamp.utc()),
                data: None, // The server doesn't send data for actions, although it could in the future.
                pending: false, // Actions received from the server always have pending=false.
            },
            uid,
            hashed_sqlite_id,
        }),
        cute_graphql::object_actions::ActionRecord::BundledActions(b) => Ok(ObjectAction {
            action_type: try_into_object_action_type(b.action_type)?,
            action_subtype: ObjectActionSubtype::BundledActions {
                count: b.count,
                oldest_timestamp: b.oldest_timestamp.utc(),
                latest_timestamp: b.latest_timestamp.utc(),
                latest_processed_at_timestamp: b.latest_processed_at_timestamp.utc(),
            },
            uid,
            hashed_sqlite_id,
        }),
        cute_graphql::object_actions::ActionRecord::Unknown => {
            Err(anyhow!("Unknown object action subtype"))
        }
    }
}
