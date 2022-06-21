//! # cliparser
//!
//! Loads rust compiler information.
//!

#[cfg(test)]
#[path = "./parser_test.rs"]
mod parser_test;

use crate::types::{CliParsed, CliSpec, Command, ParserError};
use std::ffi::OsStr;
use std::path::Path;

/// Parsers the given command line based on the given spec and returns the result.<br>
/// In case of error (such as invalid input), an error will be returned.<br>
/// In case the command line does not match the spec, Ok(None) will be returned.
pub(crate) fn parse(command_line: &Vec<&str>, spec: &CliSpec) -> Result<CliParsed, ParserError> {
    if let Err(error) = validate_input(command_line, spec) {
        return Err(error);
    }

    // fetch the command
    let (valid, args_start_index) = parse_command(&command_line, spec);
    if !valid {
        return Err(ParserError::InvalidCommandLine(
            "Command does not match spec".to_string(),
        ));
    }

    let cli_parsed = CliParsed::new();
    if args_start_index >= command_line.len() {
        return Ok(cli_parsed);
    }

    // TODO IMPL THIS
    Err(ParserError::InvalidCommandLine(
        "not implemented.....".to_string(),
    ))
}

fn parse_command(command_line: &Vec<&str>, spec: &CliSpec) -> (bool, usize) {
    if spec.command.is_empty() {
        return (false, 0);
    }

    let root_command_with_path = command_line[0];
    match Path::new(root_command_with_path).file_name() {
        Some(root_command_file_name) => {
            for command in &spec.command {
                match command {
                    Command::Command(command_string) => {
                        if OsStr::new(command_string) == root_command_file_name {
                            return (true, 1);
                        }
                        ()
                    }
                    Command::SubCommand(command_strings) => {
                        if command_strings.len() <= command_line.len()
                            && OsStr::new(&command_strings[0]) == root_command_file_name
                        {
                            let mut found = true;
                            for index in 1..command_strings.len() {
                                if command_strings[index] != command_line[index] {
                                    found = false;
                                    break;
                                }
                            }

                            if found {
                                return (true, command_strings.len());
                            }
                        }

                        ()
                    }
                };
            }

            (false, 0)
        }
        None => (false, 0),
    }
}

fn validate_input(command_line: &Vec<&str>, spec: &CliSpec) -> Result<(), ParserError> {
    if command_line.is_empty() {
        return Err(ParserError::InvalidCommandLine(
            "Empty command line provided".to_string(),
        ));
    }

    if spec.command.is_empty() && spec.arguments.is_empty() {
        return Err(ParserError::InvalidCliSpec(
            "Empty cli spec provided".to_string(),
        ));
    }

    for argument in &spec.arguments {
        if argument.key.is_empty() {
            return Err(ParserError::InvalidCliSpec(
                "Argument key not defined".to_string(),
            ));
        }
    }

    for command in &spec.command {
        match command {
            Command::Command(command_string) => {
                if command_string.is_empty() {
                    return Err(ParserError::InvalidCliSpec(
                        "Command string is empty".to_string(),
                    ));
                }
                ()
            }
            Command::SubCommand(command_strings) => {
                if command_strings.len() < 2 {
                    return Err(ParserError::InvalidCliSpec(
                        "Sub command strings count must be at least 2".to_string(),
                    ));
                }

                for command_string in command_strings {
                    if command_string.is_empty() {
                        return Err(ParserError::InvalidCliSpec(
                            "Sub command string is empty".to_string(),
                        ));
                    }
                }
            }
        };
    }

    Ok(())
}
