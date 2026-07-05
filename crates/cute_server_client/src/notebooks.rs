pub mod manager {
    #[derive(Debug, Clone)]
    pub struct NotebookManager;
    #[derive(Debug, Clone)]
    pub enum NotebookSource {
        Cloud,
        Local,
    }
    impl NotebookManager {
        pub fn new() -> Self { NotebookManager }
    }
}

pub mod notebook {
    #[derive(Debug, Clone)]
    pub struct NotebookView;
}

pub mod editor {
    pub mod view {
        #[derive(Debug, Clone)]
        pub struct RichTextEditorView;
    }
}

pub mod link {
    #[derive(Debug, Clone)]
    pub struct LinkEvent;
    #[derive(Debug, Clone)]
    pub struct NotebookLinks;
}

pub mod file {
    pub fn is_markdown_file(_path: &std::path::Path) -> bool { false }
}

#[derive(Debug, Clone)]
pub struct CloudNotebook;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NotebookId(pub String);

pub struct CloudNotebookModel;