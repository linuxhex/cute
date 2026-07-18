pub use crate::local_storage_types::models::{AIFact, AIMemory, CloudAIFact, CloudAIFactModel};
use cute_core::ui::appearance::Appearance;

use crate::local_storage_types::model::generic_string_model::StringModel;
use crate::local_storage_types::model::json_model::JsonModel;
use crate::local_storage_types::{
    GenericStringObjectFormat, GenericStringObjectUniqueKey, JsonObjectType, Revision,
};
use crate::local_storage_types::items::ai_fact::CuteDriveAIFact;
use crate::local_storage_types::CuteDriveItem;
use crate::local_storage_types::CloudObjectTypeAndId;
use crate::server::ids::SyncId;

pub mod manager;
pub mod view;
pub use manager::AIFactManager;
pub use view::{AIFactView, AIFactViewEvent};

impl StringModel for AIFact {
    type CloudObjectType = CloudAIFact;

    fn model_type_name(&self) -> &'static str {
        "Rule"
    }

    fn should_enforce_revisions() -> bool {
        true
    }

    fn model_format() -> GenericStringObjectFormat {
        GenericStringObjectFormat::Json(JsonObjectType::AIFact)
    }

    fn should_show_activity_toasts() -> bool {
        true
    }

    fn warn_if_unsaved_at_quit() -> bool {
        true
    }

    fn display_name(&self) -> String {
        match self {
            AIFact::Memory(memory) => memory.content.clone(),
        }
    }

    fn _update_object_queue_item(
        &self,
        _revision_ts: Option<Revision>,
        _object: &Self::CloudObjectType,
    ) {
        // No-op for local version
    }

    fn uniqueness_key(&self) -> Option<GenericStringObjectUniqueKey> {
        None
    }

    fn renders_in_cute_drive(&self) -> bool {
        false
    }

    fn to_cute_drive_item(
        &self,
        id: SyncId,
        _appearance: &Appearance,
        ai_fact: &CloudAIFact,
    ) -> Option<Box<dyn CuteDriveItem>> {
        Some(Box::new(CuteDriveAIFact::new(
            CloudObjectTypeAndId::GenericStringObject {
                object_type: GenericStringObjectFormat::Json(JsonObjectType::AIFact),
                id,
            },
            ai_fact.model().string_model.display_name(),
        )))
    }
}

impl JsonModel for AIFact {
    fn json_object_type() -> JsonObjectType {
        JsonObjectType::AIFact
    }
}
