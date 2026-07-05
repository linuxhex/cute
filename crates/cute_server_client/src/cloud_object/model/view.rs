// View module stub.
//
// The real implementation houses the client-side view model types used to render
// cloud objects (notebooks, workflows, env var collections, etc.) in the UI. This
// stub exists so that downstream crates can import
// `cute_server_client::cloud_object::model::view` while the full implementation is
// being ported.

/// The view model backing a cloud object in the UI.
#[derive(Debug, Clone)]
pub struct CloudViewModel;

/// An editor instance for a cloud object's view.
#[derive(Debug, Clone)]
pub struct Editor;

/// The state of an editor for a cloud object's view.
#[derive(Debug, Clone, Default)]
pub struct EditorState;
