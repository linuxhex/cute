pub mod util;

use anyhow::{bail, Result};
use warp_graphql::generic_string_object::GenericStringObjectFormat;
use warp_graphql::mutations::update_generic_string_object::{
    GenericStringObjectUpdate, UpdateGenericStringObjectResult,
};
use warp_graphql::object::ObjectUpdateSuccess;

use crate::cloud_object::model::generic_string_model::GenericStringObjectId;
use crate::cloud_object::{
    GenericServerObject, RevisionAndLastEditor, ServerFolder, ServerObject, UpdateCloudObjectResult,
};
use crate::env_vars::CloudEnvVarCollectionModel;
use crate::server::graphql::get_user_facing_error_message;
use crate::server::ids::ServerId;

pub fn update_generic_string_object_result_to_update_result(
    value: UpdateGenericStringObjectResult,
) -> Result<UpdateCloudObjectResult<Box<dyn ServerObject>>> {
    match value {
        UpdateGenericStringObjectResult::UpdateGenericStringObjectOutput(output) => {
            match output.update {
                GenericStringObjectUpdate::ObjectUpdateSuccess(success) => {
                    Ok(UpdateCloudObjectResult::Success {
                        revision_and_editor: RevisionAndLastEditor {
                            revision: success.revision_ts.into(),
                            last_editor_uid: Some(success.last_editor_uid.into_inner()),
                        },
                    })
                }
                GenericStringObjectUpdate::GenericStringObjectUpdateRejected(rejected) => {
                    let boxed: Box<dyn ServerObject> = match rejected
                        .conflicting_generic_string_object
                        .format
                    {
                        GenericStringObjectFormat::JsonEnvVarCollection => {
                            let gso = GenericServerObject::<
                                GenericStringObjectId,
                                CloudEnvVarCollectionModel,
                            >::try_from_graphql_fields(
                                ServerId::from_string_lossy(
                                    rejected
                                        .conflicting_generic_string_object
                                        .metadata
                                        .uid
                                        .inner(),
                                ),
                                Some(rejected.conflicting_generic_string_object.serialized_model),
                                rejected
                                    .conflicting_generic_string_object
                                    .metadata
                                    .try_into()?,
                                rejected
                                    .conflicting_generic_string_object
                                    .permissions
                                    .try_into()?,
                            )?;
                            let boxed: Box<dyn ServerObject> = Box::new(gso);
                            boxed
                        }
                        // AI-related formats removed
                        _ => {
                            bail!("Unsupported generic string object format: {:?}", rejected.conflicting_generic_string_object.format)
                        }
                    };
                    Ok(UpdateCloudObjectResult::Rejected { object: boxed })
                }
                GenericStringObjectUpdate::Unknown => {
                    bail!("update generic string object response has unknown variant")
                }
            }
        }
        UpdateGenericStringObjectResult::UserFacingError(e) => {
            bail!(get_user_facing_error_message(e))
        }
        UpdateGenericStringObjectResult::Unknown => {
            bail!("update generic string object response has unknown variant")
        }
    }
}

pub fn object_update_success_to_update_result(
    value: ObjectUpdateSuccess,
) -> Result<UpdateCloudObjectResult<ServerFolder>> {
    Ok(UpdateCloudObjectResult::Success {
        revision_and_editor: RevisionAndLastEditor {
            revision: value.revision_ts.into(),
            last_editor_uid: Some(value.last_editor_uid.into_inner()),
        },
    })
}
