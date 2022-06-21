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

    let mut cli_parsed = CliParsed::new();
    if args_start_index >= command_line.len() {
        return Ok(cli_parsed);
    }

    let arguments_line = &command_line[args_start_index..];
    match parse_arguments(arguments_line, spec, &mut cli_parsed) {
        Ok(_) => Ok(cli_parsed),
        Err(error) => Err(error),
    }
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

fn parse_arguments(
    arguments_line: &[&str],
    spec: &CliSpec,
    _cli_parsed: &mut CliParsed,
) -> Result<(), ParserError> {
    if arguments_line.is_empty() {
        return Ok(());
    }

    if spec.arguments.is_empty() && spec.positional_argument_name.is_none() {
        // we have arguments on the command line but we do not support arguments at all
        return Err(ParserError::InvalidCommandLine(
            "Positional arguments found but not allowed per spec".to_string(),
        ));
    }

    let mut argument_spec_in_scope = None;
    let started_positional = false;
    let mut values = vec![];
    for argument_raw in arguments_line {
        if started_positional {
            values.push(argument_raw.to_string());
        } else {
            match argument_spec_in_scope {
                Some(_argument_spec) => (),
                None => {
                    // search for argument in arguments spec
                    for argument_spec in &spec.arguments {
                        for key in &argument_spec.key {
                            if key == argument_raw {
                                argument_spec_in_scope = Some(argument_spec);
                                break;
                            }
                        }
                    }

                    match argument_spec_in_scope {
                        Some(_argument_spec) => (),
                        None => (),
                    }
                    // TODO IMPL

                    ()
                }
            }
        }
    }

    Ok(())
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
