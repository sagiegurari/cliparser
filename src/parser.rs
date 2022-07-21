//! # parser
//!
//! Parsers arguments line based on given spec and returned parsed data.
//!

#[cfg(test)]
#[path = "./parser_test.rs"]
mod parser_test;

use crate::types::{
    Argument, ArgumentOccurrence, ArgumentValueType, CliParsed, CliSpec, Command, ParserError,
};
use std::env;
use std::ffi::OsStr;
use std::path::Path;

/// Parsers the given command line based on the given spec and returns the result.<br>
/// In case of invalid input or the provided spec does not match the command line, an error will be returned.
pub(crate) fn parse(command_line: &Vec<&str>, spec: &CliSpec) -> Result<CliParsed, ParserError> {
    if let Err(error) = validate_input(command_line, spec) {
        return Err(error);
    }

    // fetch the command
    let (valid, args_start_index) = parse_command(&command_line, spec);
    if !valid {
        return Err(ParserError::InvalidCommandLine(
            format!(
                "Command does not match spec, command line: {:?}",
                command_line
            )
            .to_string(),
        ));
    }

    let mut cli_parsed = CliParsed::new();
    if args_start_index > command_line.len() {
        return Ok(cli_parsed);
    }

    let arguments_line = &command_line[args_start_index..];
    match parse_arguments(arguments_line, spec, &mut cli_parsed) {
        Ok(_) => {
            insert_default_values(spec, &mut cli_parsed);
            Ok(cli_parsed)
        }
        Err(error) => Err(error),
    }
}

/// Parsers the current process command line based on the given spec and returns the result.<br>
/// In case of invalid input or the provided spec does not match the command line, an error will be returned.
pub(crate) fn parse_process(spec: &CliSpec) -> Result<CliParsed, ParserError> {
    let command_line_strings = get_process_command_line();
    let command_line = command_line_strings
        .iter()
        .map(|value| value.as_str())
        .collect();
    parse(&command_line, spec)
}

/// Parsers the given command line based on the given specs and returns the result.<br>
/// In case of invalid input or none of the provided specs do not match the command line, an error will be returned.
pub(crate) fn parse_any(
    command_line: &Vec<&str>,
    specs: Vec<&CliSpec>,
) -> Result<(usize, CliParsed), ParserError> {
    let mut index = 0;
    for spec in specs {
        let result = parse(command_line, spec);
        if let Ok(cli_parsed) = result {
            return Ok((index, cli_parsed));
        }

        index = index + 1;
    }

    Err(ParserError::InvalidCommandLine(
        "Command does not match any spec".to_string(),
    ))
}

/// Parsers the current process command line based on the given specs and returns the result.<br>
/// In case of invalid input or none of the provided specs do not match the command line, an error will be returned.
pub(crate) fn parse_process_any(specs: Vec<&CliSpec>) -> Result<(usize, CliParsed), ParserError> {
    let command_line_strings = get_process_command_line();
    let command_line = command_line_strings
        .iter()
        .map(|value| value.as_str())
        .collect();
    parse_any(&command_line, specs)
}

#[cfg(not(test))]
fn get_process_command_line() -> Vec<String> {
    env::args().collect()
}

#[cfg(test)]
fn get_process_command_line() -> Vec<String> {
    env::var("TEST_CLI_PARSER_COMMAND_LINE")
        .unwrap_or("".to_string())
        .split(" ")
        .map(|value| value.to_string())
        .collect()
}

fn insert_default_values(spec: &CliSpec, cli_parsed: &mut CliParsed) {
    for argument_spec in &spec.arguments {
        if !cli_parsed.arguments.contains(&argument_spec.name) {
            if argument_spec.value_type != ArgumentValueType::None {
                if let Some(ref default_value) = argument_spec.default_value {
                    cli_parsed
                        .argument_values
                        .insert(argument_spec.name.clone(), vec![default_value.to_string()]);
                }
            }
        }
    }
}

fn parse_command(command_line: &Vec<&str>, spec: &CliSpec) -> (bool, usize) {
    if spec.command.is_empty() {
        return (true, 0);
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
    cli_parsed: &mut CliParsed,
) -> Result<(), ParserError> {
    if arguments_line.is_empty() {
        return Ok(());
    }

    if spec.arguments.is_empty() && spec.positional_argument.is_none() {
        // we have arguments on the command line but we do not support arguments at all
        return Err(ParserError::InvalidCommandLine(
            "Positional arguments found but not allowed per spec".to_string(),
        ));
    }

    let positional_argument_name = match &spec.positional_argument {
        Some(positional_argument_spec) => Some(positional_argument_spec.name.clone()),
        None => None,
    };

    let mut argument_spec_in_scope: Option<Argument> = None;
    let mut started_positional = false;
    for argument_raw in arguments_line {
        if started_positional {
            insert_argument_value(
                cli_parsed,
                &positional_argument_name.clone().unwrap(),
                &argument_raw,
            );
        } else {
            // we found an argument key of single value type in previous index, now we get its value
            if let Some(ref argument_spec) = argument_spec_in_scope {
                if argument_spec.value_type == ArgumentValueType::Single {
                    insert_argument_value(cli_parsed, &argument_spec.name, &argument_raw);

                    argument_spec_in_scope = None;
                    continue;
                }
            }

            // The -- is a special case that defines start of positional arguments
            if *argument_raw == "--" {
                started_positional = true;
                continue;
            }

            // search for argument in arguments spec
            let mut argument_spec_found = None;
            let argument_key_value_parts = argument_raw.split_once("=");
            let mut argument_with_value = false;
            for argument_spec in &spec.arguments {
                for key in &argument_spec.key {
                    if key == argument_raw {
                        argument_spec_found = Some(argument_spec);
                        break;
                    } else {
                        if argument_spec.value_type == ArgumentValueType::Single {
                            match argument_key_value_parts {
                                Some(ref parts) => {
                                    let (key_prefix, _) = parts;
                                    if key == key_prefix {
                                        argument_spec_found = Some(argument_spec);
                                        argument_with_value = true;
                                        break;
                                    }
                                }
                                None => (),
                            }
                        }
                    }
                }
            }

            match argument_spec_in_scope {
                Some(ref current_argument_spec) => match argument_spec_found {
                    Some(found_argument_spec) => {
                        if cli_parsed.arguments.contains(&found_argument_spec.name)
                            && found_argument_spec.argument_occurrence
                                != ArgumentOccurrence::Multiple
                        {
                            return Err(ParserError::InvalidCommandLine(
                                format!("Duplicate argument {} found", &found_argument_spec.name)
                                    .to_string(),
                            ));
                        }

                        cli_parsed
                            .arguments
                            .insert(found_argument_spec.name.clone());

                        match found_argument_spec.value_type {
                            ArgumentValueType::Single | ArgumentValueType::Multiple => {
                                argument_spec_in_scope = Some(found_argument_spec.clone());
                            }
                            ArgumentValueType::None => (),
                        }

                        if argument_with_value {
                            let (_, value_part) = argument_key_value_parts.unwrap();
                            insert_argument_value(
                                cli_parsed,
                                &found_argument_spec.name,
                                value_part,
                            );
                            argument_spec_in_scope = None;
                        }
                    }
                    None => {
                        // current value is not a new argument key and we are currently in multi value key so
                        // lets add it as an additional value
                        if current_argument_spec.value_type == ArgumentValueType::Multiple {
                            insert_argument_value(
                                cli_parsed,
                                &current_argument_spec.name,
                                &argument_raw,
                            );
                        } else {
                            return Err(ParserError::InternalError(
                                "Non multiple argument found in scope".to_string(),
                            ));
                        }
                    }
                },
                None => match argument_spec_found {
                    Some(found_argument_spec) => {
                        if cli_parsed.arguments.contains(&found_argument_spec.name)
                            && found_argument_spec.argument_occurrence
                                != ArgumentOccurrence::Multiple
                        {
                            return Err(ParserError::InvalidCommandLine(
                                format!("Duplicate argument {} found", &found_argument_spec.name)
                                    .to_string(),
                            ));
                        }

                        cli_parsed
                            .arguments
                            .insert(found_argument_spec.name.clone());

                        match found_argument_spec.value_type {
                            ArgumentValueType::Single | ArgumentValueType::Multiple => {
                                argument_spec_in_scope = Some(found_argument_spec.clone());
                            }
                            ArgumentValueType::None => (),
                        }

                        if argument_with_value {
                            let (_, value_part) = argument_key_value_parts.unwrap();
                            insert_argument_value(
                                cli_parsed,
                                &found_argument_spec.name,
                                value_part,
                            );
                            argument_spec_in_scope = None;
                        }
                    }
                    None => match positional_argument_name {
                        // current value is not a new argument key and we are not in a scope of some argument
                        // so it must be a positional argument
                        Some(ref name) => {
                            started_positional = true;
                            insert_argument_value(cli_parsed, name, &argument_raw)
                        }
                        None => {
                            return Err(ParserError::InvalidCommandLine(
                                "Positional arguments found but not allowed per spec".to_string(),
                            ));
                        }
                    },
                },
            }
        }
    }

    Ok(())
}

fn validate_input(command_line: &Vec<&str>, spec: &CliSpec) -> Result<(), ParserError> {
    if command_line.is_empty() && !spec.command.is_empty() {
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

fn insert_argument_value(cli_parsed: &mut CliParsed, name: &str, value: &str) {
    cli_parsed.arguments.insert(name.to_string());

    match cli_parsed.argument_values.remove(name) {
        Some(mut values) => {
            values.push(value.to_string());
            cli_parsed.argument_values.insert(name.to_string(), values);
        }
        None => {
            cli_parsed
                .argument_values
                .insert(name.to_string(), vec![value.to_string()]);
        }
    }
}
