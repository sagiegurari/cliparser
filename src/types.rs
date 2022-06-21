//! # types
//!
//! Defines the various types.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

use std::collections::HashMap;
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
}

impl Display for ParserError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::InvalidCommandLine(ref message) => write!(formatter, "{}", message),
            Self::InvalidCliSpec(ref message) => write!(formatter, "{}", message),
            Self::CommandDoesNotMatchSpec(ref message) => write!(formatter, "{}", message),
        }
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidCommandLine(_) => None,
            Self::InvalidCliSpec(_) => None,
            Self::CommandDoesNotMatchSpec(_) => None,
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
    /// The argument can appear multiple times
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
}

#[derive(Debug, Clone, PartialEq, Default)]
/// Holds the command line spec (command/parameters/...)
pub struct CliSpec {
    /// A list of all possible commands and sub commands.
    pub command: Vec<Command>,
    /// A list of all possible command line arguments.
    pub arguments: Vec<Argument>,
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
    /// A list of all arguments found (list of names not keys).
    /// Arguments that were not found by defaulted to a given value will not be listed here.
    pub arguments: Vec<String>,
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
