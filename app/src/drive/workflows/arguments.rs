use std::ops::Range;

#[derive(Debug, Clone, Default)]
pub struct ArgumentsState {
    pub arguments: Vec<crate::workflows::workflow::Argument>,
    pub invalid_arguments_char_ranges: Vec<Range<usize>>,
    pub valid_arguments_char_ranges_and_arg_index: Vec<(Range<usize>, usize)>,
}

impl ArgumentsState {
    pub fn for_command_workflow(_prev_state: &ArgumentsState, _content: String) -> Self {
        // Stub implementation - extract arguments from content
        Self::default()
    }

    pub fn for_saved_prompt(_prev_state: &ArgumentsState, _content: String) -> Self {
        Self::default()
    }
}
