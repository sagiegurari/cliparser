//! # types
//!
//! Defines the various types.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
/// Holds the error information
pub enum ParserError {
    /// Error Info Type
    InvalidCommandLine(String),
    /// Error Info Type
    InvalidCliSpec(String),
    /// Error Info Type
    CommandDoesNotMatchSpec(String),
    /// Error Info Type
    InternalError(String),
}

impl Display for ParserError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::InvalidCommandLine(ref message) => write!(formatter, "{}", message),
            Self::InvalidCliSpec(ref message) => write!(formatter, "{}", message),
            Self::CommandDoesNotMatchSpec(ref message) => write!(formatter, "{}", message),
            Self::InternalError(ref message) => write!(formatter, "{}", message),
        }
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidCommandLine(_) => None,
            Self::InvalidCliSpec(_) => None,
            Self::CommandDoesNotMatchSpec(_) => None,
            Self::InternalError(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// The command (not params) string/s
pub enum Command {
    /// Single command (not sub command) such as: "ls".
    /// Any path prefix will be ignored while parsing.
    Command(String),
    /// Sub command value such as: vec!["cargo".to_string(), "myplugin".to_string()].
    /// Any path prefix will be ignored while parsing for the first element only.
    SubCommand(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Copy)]
/// The argument occurrence type (see values for more info)
pub enum ArgumentOccurrence {
    /// The argument can appear only once
    Single,
    /// The argument can appear multiple times. The value of each occurrence will be
    /// picked up so even args with single value constraint can have multiple values if
    /// they support multiple occurrences.
    Multiple,
}

#[derive(Debug, Clone, PartialEq, Copy)]
/// The argument value type (see values for more info)
pub enum ArgumentValueType {
    /// The argument does not accept any value
    None,
    /// Only single value is allowed
    Single,
    /// Allows multiple values (minimum one)
    Multiple,
}

#[derive(Debug, Clone, PartialEq)]
/// The argument help text
pub enum ArgumentHelp {
    /// Text value
    Text(String),
    /// Text and variable name
    TextAndParam(String, String),
}

#[derive(Debug, Clone, PartialEq)]
/// Holds the command line argument spec
pub struct Argument {
    /// Unique name for the argument later used to pull the parsed information
    pub name: String,
    /// All possible argument keys in the command line (for example: vec!["--env".to_string(), "-e".to_string()])
    pub key: Vec<String>,
    /// The argument occurrence (see enum)
    pub argument_occurrence: ArgumentOccurrence,
    /// The possible value type for this specific argument
    pub value_type: ArgumentValueType,
    /// Default value if not found
    pub default_value: Option<String>,
    /// Help text
    pub help: Option<ArgumentHelp>,
}

#[derive(Debug, Clone, PartialEq)]
/// Holds the positional argument spec
pub struct PositionalArgument {
    /// Unique name for the argument later used to pull the parsed information
    pub name: String,
    /// Help text
    pub help: Option<ArgumentHelp>,
}

#[derive(Debug, Clone, PartialEq, Default)]
/// Holds the command line spec meta information used to generate version and help messages
pub struct CliSpecMetaInfo {
    /// Author name
    pub author: Option<String>,
    /// Version string
    pub version: Option<String>,
    /// Description string
    pub description: Option<String>,
    /// Project name
    pub project: Option<String>,
}

impl CliSpecMetaInfo {
    /// Returns new instance
    pub fn new() -> CliSpecMetaInfo {
        Default::default()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
/// Holds the command line spec (command/parameters/...)
pub struct CliSpec {
    /// A list of all possible commands and sub commands.
    pub command: Vec<Command>,
    /// A list of all possible command line arguments.
    pub arguments: Vec<Argument>,
    /// The name of the argument that will hold all arguments after the last known key based
    /// argument. If not defined, such positional arguments are not allowed.
    pub positional_argument: Option<PositionalArgument>,
    /// Meta information used for generating version and help messages
    pub meta_info: Option<CliSpecMetaInfo>,
}

impl CliSpec {
    /// Returns new instance
    pub fn new() -> CliSpec {
        Default::default()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
/// Holds the command line parse result
pub struct CliParsed {
    /// A collection of all arguments found (list of names not keys).
    /// Arguments that were not found by defaulted to a given value will not be listed here.
    pub arguments: HashSet<String>,
    /// A map of all values for arguments found.
    /// The map will exclude arguments that do not accept value but include arguments not provided
    /// on the command line but were defaulted to a given value.
    /// The map keys are the argument names (not keys) and the value is the list of all values
    /// found for all occurrences.
    pub argument_values: HashMap<String, Vec<String>>,
}

impl CliParsed {
    /// Returns new instance
    pub fn new() -> CliParsed {
        Default::default()
    }
}
