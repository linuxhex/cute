//! Stub implementation for arg_selector module

#[derive(Debug, Clone)]
pub struct ArgSelector {
    pub id: String,
}

impl ArgSelector {
    pub fn new() -> Self {
        Self { id: String::new() }
    }
}

impl Default for ArgSelector {
    fn default() -> Self {
        Self::new()
    }
}
