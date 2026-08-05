// Stub for warp-command-signatures.
// Provides minimal types needed for compilation.

use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    CmdExe,
    Posix,
    Powershell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconType {
    Default,
    File,
    Folder,
}

#[derive(Debug, Clone, Default)]
pub struct Argument {
    pub display_name: Option<String>,
    pub description: Option<String>,
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

    pub fn is_variadic(&self) -> bool {
        self.is_variadic
    }

    pub fn is_required(&self) -> bool {
        matches!(self.optional, IsArgumentOptional::Required)
    }

    pub fn name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    pub fn filter_template_by_name(
        &self,
        _filters: Option<&[TemplateFilter]>,
        _name: &str,
    ) -> Option<&TemplateFilter> {
        None
    }

    pub fn generator_by_name(
        &self,
        _generators: Option<&[Generator]>,
        _name: &GeneratorName,
    ) -> Option<&Generator> {
        None
    }
}

#[derive(Debug, Clone)]
pub enum ArgumentType {
    String,
    Path,
    Suggestion(Suggestion),
    Template(Template),
    Generator(GeneratorName),
    Alias(String),
}

impl Default for ArgumentType {
    fn default() -> Self {
        ArgumentType::String
    }
}

#[derive(Debug, Clone, Default)]
pub struct Alias;

impl Alias {
    pub fn command(&self, _tokens: &[&str]) -> String {
        String::new()
    }

    pub fn on_complete(&self, _output: &str, _tokens: &[&str], _token_idx: usize) -> Option<String> {
        None
    }
}

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone)]
pub struct ShellCommandBuilder(String);

impl ShellCommandBuilder {
    pub fn build(&self, _shell: Shell) -> Cow<'_, str> {
        Cow::Borrowed("")
    }
}

impl Default for ShellCommandBuilder {
    fn default() -> Self {
        ShellCommandBuilder(String::new())
    }
}

pub type CommandFromTokensFn = fn(&[&str], bool, &[String]) -> ShellCommandBuilder;

#[derive(Debug, Clone)]
pub enum GeneratorProcess {
    ShellCommand(ShellCommandBuilder),
    CommandFromTokens(CommandFromTokensFn),
}

impl Default for GeneratorProcess {
    fn default() -> Self {
        GeneratorProcess::ShellCommand(ShellCommandBuilder::default())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Generator {
    pub script: Option<GeneratorScript>,
    pub template: Option<Template>,
    pub process: GeneratorProcess,
}

impl Generator {
    pub fn on_complete(&self, _output: &str) -> GeneratorResults {
        GeneratorResults::default()
    }
}

#[derive(Debug, Clone, Default)]
pub struct GeneratorScript;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum GeneratorProcessKind {
    #[default]
    ShellCommand,
    CommandFromTokens,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct GeneratorName;

#[derive(Debug, Clone, Default)]
pub struct GeneratorResults {
    pub suggestions: Vec<Suggestion>,
    pub is_ordered: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Flag {
    pub name: &'static str,
    pub description: Option<String>,
    pub priority: Priority,
    pub style: FlagStyle,
}

#[derive(Debug, Clone, Default)]
pub struct Signature {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<Vec<Argument>>,
    pub options: Option<Vec<Opt>>,
    pub subcommands: Option<Vec<Signature>>,
    pub priority: Priority,
    pub alias_generator: Option<Generator>,
    pub parser_directives: ParserDirectives,
}

impl Signature {
    pub fn arguments(&self) -> &[Argument] {
        self.arguments.as_deref().unwrap_or(&[])
    }

    pub fn options(&self) -> &[Opt] {
        self.options.as_deref().unwrap_or(&[])
    }

    pub fn subcommands(&self) -> &[Signature] {
        self.subcommands.as_deref().unwrap_or(&[])
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn alias(&self, _aliases: Option<&[String]>) -> Option<Alias> {
        None
    }

    pub fn short_hand_flags(&self) -> std::vec::IntoIter<Flag> {
        vec![].into_iter()
    }

    pub fn long_hand_flags(&self) -> std::vec::IntoIter<Flag> {
        vec![].into_iter()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Template {
    pub type_name: TemplateType,
    pub filter_name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct TemplateFilter;

impl TemplateFilter {
    pub fn filter(&self, _suggestion: Suggestion, _file_type: PathSuggestionType) -> Option<Suggestion> {
        None
    }
}

#[derive(Debug, Clone, Default)]
pub enum TemplateType {
    #[default]
    Default,
    FilesAndFolders,
    Files {
        must_exist: bool,
    },
    Folders {
        must_exist: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum IsArgumentOptional {
    Optional(Option<String>),
    Required,
}

impl Default for IsArgumentOptional {
    fn default() -> Self {
        IsArgumentOptional::Optional(None)
    }
}

impl IsArgumentOptional {
    pub fn is_required(&self) -> bool {
        matches!(self, IsArgumentOptional::Required)
    }
}

#[derive(Debug, Clone, Default)]
pub struct Opt {
    pub arguments: Option<Vec<Argument>>,
    pub description: Option<String>,
    pub exact_string: Vec<String>,
    pub priority: Priority,
    pub required: bool,
}

impl Opt {
    pub fn arguments(&self) -> &[Argument] {
        self.arguments.as_deref().unwrap_or(&[])
    }

    pub fn is_switch(&self) -> bool {
        false
    }

    pub fn names(&self) -> std::vec::IntoIter<&str> {
        self.exact_string.iter().map(|s| s.as_str()).collect::<Vec<_>>().into_iter()
    }

    pub fn has_name(&self, _name: &str) -> bool {
        false
    }
}

#[derive(Debug, Clone, Default)]
pub struct ParserDirectives {
    pub always_case_insensitive: bool,
    pub flags_are_posix_noncompliant: bool,
    pub flags_match_unique_prefix: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Order(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Importance {
    More(Order),
    Less(Order),
}

impl Default for Importance {
    fn default() -> Self {
        Importance::More(Order(0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Global(Importance),
    Local(Importance),
    Default,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Default
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum FlagStyle {
    #[default]
    Default,
    SingleDash,
    DoubleDash,
}

#[derive(Debug, Clone, Default)]
pub enum PathSuggestionType {
    #[default]
    Default,
    File,
    Folder,
}

#[derive(Debug, Clone, Default)]
pub struct Suggestion {
    pub display: String,
    pub display_name: Option<String>,
    pub exact_string: String,
    pub description: Option<String>,
    pub icon: Option<IconType>,
    pub is_hidden: bool,
    pub priority: Priority,
}

#[derive(Debug, Clone, Default)]
pub struct OrderData;

pub fn dynamic_command_signature_data() -> HashMap<String, DynamicCompletionData> {
    HashMap::new()
}

pub fn signature_by_name(_name: &str) -> Option<Signature> {
    None
}

pub fn order_by_importance(_signatures: &mut [Signature]) {}
