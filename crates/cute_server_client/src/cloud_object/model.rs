pub mod view;

pub mod generic_string_model {
    pub trait StringModel {}
    #[derive(Debug, Clone)]
    pub struct GenericStringModel;
    #[derive(Debug, Clone)]
    pub struct GenericStringObjectId(pub String);
    pub trait Serializer {}
    #[derive(Debug, Clone)]
    pub enum GenericStringObjectFormat {
        Json(super::json_model::JsonObjectType),
        Text,
    }
    #[derive(Debug, Clone)]
    pub struct CloudStringObject;
}

pub mod persistence {
    pub trait CloudModel: Sized + 'static {
        type Id: std::fmt::Debug + Clone + PartialEq + Eq + std::hash::Hash + 'static;
        fn id(&self) -> &Self::Id { unimplemented!() }
        fn serialize_for_update(&self) -> Vec<u8> { Vec::new() }
        fn serialize_for_create(&self) -> Vec<u8> { Vec::new() }
    }
    pub struct CloudModelConfig;
    #[derive(Debug, Clone)]
    pub enum CloudModelEvent {
        ObjectUpdated,
        ObjectDeleted,
    }
}

pub mod json_model {
    #[derive(Debug, Clone)]
    pub struct JsonModel;
    #[derive(Debug, Clone)]
    pub enum JsonObjectType {
        EnvVarCollection,
        MCPServer,
        AIExecutionProfile,
        ScheduledAmbientAgent,
        UserProfile,
        Preference,
    }
}

pub mod actions {
    #[derive(Debug, Clone)]
    pub struct ObjectActions;
    #[derive(Debug, Clone)]
    pub struct ObjectOperation;
    #[derive(Debug, Clone)]
    pub struct OperationSuccessType;
    pub trait CloudObjectActions {}
    pub trait ObjectType: Clone + std::fmt::Debug + 'static {}
}
