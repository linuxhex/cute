// Stub for warp-command-signatures.
// Provides minimal types needed for compilation.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IconType {
    Default,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub description: Option<String>,
    pub display_name: Option<String>,
    pub is_command: bool,
    pub is_variadic: bool,
    pub optional: IsArgumentOptional,
    pub argument_types: Vec<ArgumentType>,
    pub skip_generator_validation: bool,
}

impl Argument {
    pub fn is_command(&self) -> bool {
        self.is_command
    }
    pub fn filter_template_by_name(&self, _name: &str) -> Option<&Template> {
        None
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ArgumentType {
    #[default]
    String,
    Path,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DynamicCompletionData;

impl DynamicCompletionData {
    pub fn aliases(&self) -> &[String] {
        &[]
    }
    pub fn filters(&self) -> &[TemplateFilter] {
        &[]
    }
    pub fn generators(&self) -> &[Generator] {
        &[]
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Generator;

#[derive(Debug, Clone, Default)]
pub struct GeneratorProcess {
    pub kind: GeneratorProcessKind,
}

#[derive(Debug, Clone, Default)]
pub enum GeneratorProcessKind {
    #[default]
    ShellCommand,
    CommandFromTokens,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GeneratorName;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Vec<Argument>,
    pub options: Vec<Opt>,
    pub subcommands: Vec<Signature>,
    pub priority: Priority,
    pub alias_generator: Option<Generator>,
    pub parser_directives: Option<ParserDirectives>,
}

impl Signature {
    pub fn arguments(&self) -> &[Argument] {
        &self.arguments
    }
    pub fn alias(&self) -> Option<&str> {
        None
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Template;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateFilter;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum TemplateType {
    #[default]
    Default,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum IsArgumentOptional {
    #[default]
    Optional,
    Required,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Opt {
    pub arguments: Vec<Argument>,
    pub description: Option<String>,
    pub exact_string: Option<String>,
    pub priority: Priority,
    pub required: bool,
}

impl Opt {
    pub fn arguments(&self) -> &[Argument] {
        &self.arguments
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ParserDirectives;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Priority {
    #[default]
    Default,
    Global,
    Local,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum FlagStyle {
    #[default]
    Default,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum PathSuggestionType {
    #[default]
    Default,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Suggestion {
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Order;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum Importance {
    #[default]
    Less,
    More,
}

pub fn dynamic_command_signature_data() -> Vec<DynamicCompletionData> {
    vec![]
}

pub fn signature_by_name(_name: &str) -> Option<Signature> {
    None
}
