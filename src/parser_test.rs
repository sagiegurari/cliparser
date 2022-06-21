use super::*;
use crate::types::{Argument, ArgumentOccurrence, ArgumentValueType};

#[test]
fn parse_invalid_input() {
    let cli_spec = CliSpec::new();
    let result = parse(&vec![], &cli_spec);

    assert!(result.is_err());
}

#[test]
fn parse_command_no_match() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::Command("test".to_string()));
    let result = parse(&vec!["other"], &cli_spec);

    assert!(result.is_err());
}

#[test]
fn parse_command_only_command() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "test".to_string(),
    ]));
    let result = parse(&vec!["cargo", "test"], &cli_spec);

    assert!(result.is_ok());

    let cli_parsed = result.unwrap();
    assert!(cli_parsed.arguments.is_empty());
    assert!(cli_parsed.argument_values.is_empty());
}

#[test]
fn parse_command_empty_command() {
    let cli_spec = CliSpec::new();
    let (valid, index) = parse_command(&vec!["test"], &cli_spec);

    assert!(!valid);
    assert_eq!(index, 0);
}

#[test]
fn parse_command_match_command() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::Command("test".to_string()));
    let (valid, index) = parse_command(&vec!["test"], &cli_spec);

    assert!(valid);
    assert_eq!(index, 1);
}

#[test]
fn parse_command_match_command_ignore_path() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::Command("test".to_string()));
    let (valid, index) = parse_command(&vec!["./bin/dir/test"], &cli_spec);

    assert!(valid);
    assert_eq!(index, 1);
}

#[test]
fn parse_command_no_match_command() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::Command("test".to_string()));
    let (valid, index) = parse_command(&vec!["test2"], &cli_spec);

    assert!(!valid);
    assert_eq!(index, 0);
}

#[test]
fn parse_command_match_subcommand() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "test1".to_string(),
        "test2".to_string(),
        "test3".to_string(),
    ]));
    let (valid, index) = parse_command(&vec!["cargo", "test1", "test2", "test3"], &cli_spec);

    assert!(valid);
    assert_eq!(index, 4);
}

#[test]
fn parse_command_match_subcommand_ignore_path() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "test".to_string(),
    ]));
    let (valid, index) = parse_command(&vec!["./bin/dir/cargo", "test"], &cli_spec);

    assert!(valid);
    assert_eq!(index, 2);
}

#[test]
fn parse_command_no_match_subcommand_root() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "test".to_string(),
    ]));
    let (valid, index) = parse_command(&vec!["cargo2", "test"], &cli_spec);

    assert!(!valid);
    assert_eq!(index, 0);
}

#[test]
fn parse_command_no_match_subcommand_non_root() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "test".to_string(),
    ]));
    let (valid, index) = parse_command(&vec!["cargo", "./test"], &cli_spec);

    assert!(!valid);
    assert_eq!(index, 0);
}

#[test]
fn parse_command_no_match_subcommand_command_line_too_short() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "test1".to_string(),
        "test2".to_string(),
    ]));
    let (valid, index) = parse_command(&vec!["cargo", "test1"], &cli_spec);

    assert!(!valid);
    assert_eq!(index, 0);
}

#[test]
fn parse_arguments_empty_arguments_line() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "test".to_string(),
        key: vec!["test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
    });
    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec![], &cli_spec, &mut cli_parsed);

    assert!(result.is_ok());
    assert!(cli_parsed.arguments.is_empty());
    assert!(cli_parsed.argument_values.is_empty());
}

#[test]
fn parse_arguments_non_empty_arguments_line_but_not_args_in_spec() {
    let cli_spec = CliSpec::new();
    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec!["test"], &cli_spec, &mut cli_parsed);

    assert!(result.is_err());
}

#[test]
fn validate_input_all_empty() {
    let cli_spec = CliSpec::new();
    let result = validate_input(&vec![], &cli_spec);

    assert!(result.is_err());
}

#[test]
fn validate_input_empty_spec() {
    let cli_spec = CliSpec::new();
    let result = validate_input(&vec!["cargo"], &cli_spec);

    assert!(result.is_err());
}

#[test]
fn validate_input_empty_command_line() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::Command("cargo".to_string()));
    let result = validate_input(&vec![], &cli_spec);

    assert!(result.is_err());
}

#[test]
fn validate_input_empty_spec_command() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::Command("".to_string()));
    let result = validate_input(&vec!["test"], &cli_spec);

    assert!(result.is_err());
}

#[test]
fn validate_input_empty_spec_sub_command() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::SubCommand(vec![]));
    let result = validate_input(&vec!["test"], &cli_spec);

    assert!(result.is_err());
}

#[test]
fn validate_input_spec_sub_command_size_one() {
    let mut cli_spec = CliSpec::new();
    cli_spec
        .command
        .push(Command::SubCommand(vec!["test".to_string()]));
    let result = validate_input(&vec!["test"], &cli_spec);

    assert!(result.is_err());
}

#[test]
fn validate_input_spec_sub_command_with_empty_string() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::SubCommand(vec![
        "test".to_string(),
        "".to_string(),
    ]));
    let result = validate_input(&vec!["test"], &cli_spec);

    assert!(result.is_err());
}

#[test]
fn validate_input_valid_spec_with_command() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::Command("cargo".to_string()));
    let result = validate_input(&vec!["test"], &cli_spec);

    assert!(result.is_ok());
}

#[test]
fn validate_input_valid_spec_with_subcommand() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "test".to_string(),
    ]));
    let result = validate_input(&vec!["test"], &cli_spec);

    assert!(result.is_ok());
}

#[test]
fn validate_input_valid_spec_with_arguments() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "test".to_string(),
        key: vec!["test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
    });
    let result = validate_input(&vec!["test"], &cli_spec);

    assert!(result.is_ok());
}
