use crate::cloud_stub_types::{GenericStringObjectFormat, JsonObjectType};

pub const ENV_VAR_COLLECTION_FORMAT: GenericStringObjectFormat =
    GenericStringObjectFormat::Json(JsonObjectType::EnvVarCollection);
