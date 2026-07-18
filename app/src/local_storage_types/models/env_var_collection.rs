use crate::local_storage_types::{GenericStringObjectFormat, JsonObjectType};

pub const ENV_VAR_COLLECTION_FORMAT: GenericStringObjectFormat =
    GenericStringObjectFormat::Json(JsonObjectType::EnvVarCollection);
