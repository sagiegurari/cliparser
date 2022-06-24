use super::*;
use crate::types::{Argument, ArgumentOccurrence, ArgumentValueType, CliSpec, Command};
use doc_comment as _;
use std::collections::{HashMap, HashSet};
use std::env;

#[test]
fn parse_combination() {
    let mut cli_spec = CliSpec::new();
    cli_spec.positional_argument_name = Some("args".to_string());
    cli_spec.arguments.push(Argument {
        name: "flag".to_string(),
        key: vec!["--flag".to_string(), "-f".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "single".to_string(),
        key: vec!["--s1".to_string(), "-s".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "mo".to_string(),
        key: vec!["--mo1".to_string(), "-mo2".to_string()],
        argument_occurrence: ArgumentOccurrence::Multiple,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "mv".to_string(),
        key: vec!["--mv1".to_string(), "-mv2".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Multiple,
        default_value: None,
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "default".to_string(),
        key: vec!["--d1".to_string(), "-d".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: Some("some default".to_string()),
        help: None,
    });

    let result = parse(
        &vec![
            "-mv2", "4", "5", "6", "--mo1", "1", "-mo2", "2", "-f", "-s", "3", "arg1", "arg2",
            "-mo2", "arg5",
        ],
        &cli_spec,
    );

    assert!(result.is_ok());
    let cli_parsed = result.unwrap();

    let mut argument_names = HashSet::new();
    argument_names.insert("flag".to_string());
    argument_names.insert("single".to_string());
    argument_names.insert("mo".to_string());
    argument_names.insert("mv".to_string());
    argument_names.insert("args".to_string());
    let mut argument_values = HashMap::new();
    argument_values.insert("single".to_string(), vec!["3".to_string()]);
    argument_values.insert("mo".to_string(), vec!["1".to_string(), "2".to_string()]);
    argument_values.insert(
        "mv".to_string(),
        vec!["4".to_string(), "5".to_string(), "6".to_string()],
    );
    argument_values.insert(
        "args".to_string(),
        vec![
            "arg1".to_string(),
            "arg2".to_string(),
            "-mo2".to_string(),
            "arg5".to_string(),
        ],
    );
    argument_values.insert("default".to_string(), vec!["some default".to_string()]);
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, argument_values);
}

#[test]
#[ignore]
fn parse_process_combination() {
    let mut cli_spec = CliSpec::new();
    cli_spec.positional_argument_name = Some("args".to_string());
    cli_spec.arguments.push(Argument {
        name: "flag".to_string(),
        key: vec!["--flag".to_string(), "-f".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "single".to_string(),
        key: vec!["--s1".to_string(), "-s".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "mo".to_string(),
        key: vec!["--mo1".to_string(), "-mo2".to_string()],
        argument_occurrence: ArgumentOccurrence::Multiple,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "mv".to_string(),
        key: vec!["--mv1".to_string(), "-mv2".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Multiple,
        default_value: None,
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "default".to_string(),
        key: vec!["--d1".to_string(), "-d".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: Some("some default".to_string()),
        help: None,
    });

    env::set_var(
        "TEST_CLI_PARSER_COMMAND_LINE",
        vec![
            "-mv2", "4", "5", "6", "--mo1", "1", "-mo2", "2", "-f", "-s", "3", "arg1", "arg2",
            "-mo2", "arg5",
        ]
        .join(" "),
    );
    let result = parse_process(&cli_spec);

    assert!(result.is_ok());
    let cli_parsed = result.unwrap();

    let mut argument_names = HashSet::new();
    argument_names.insert("flag".to_string());
    argument_names.insert("single".to_string());
    argument_names.insert("mo".to_string());
    argument_names.insert("mv".to_string());
    argument_names.insert("args".to_string());
    let mut argument_values = HashMap::new();
    argument_values.insert("single".to_string(), vec!["3".to_string()]);
    argument_values.insert("mo".to_string(), vec!["1".to_string(), "2".to_string()]);
    argument_values.insert(
        "mv".to_string(),
        vec!["4".to_string(), "5".to_string(), "6".to_string()],
    );
    argument_values.insert(
        "args".to_string(),
        vec![
            "arg1".to_string(),
            "arg2".to_string(),
            "-mo2".to_string(),
            "arg5".to_string(),
        ],
    );
    argument_values.insert("default".to_string(), vec!["some default".to_string()]);
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, argument_values);
}

#[test]
fn parse_any_match() {
    let mut cli_spec1 = CliSpec::new();
    cli_spec1
        .command
        .push(Command::Command("test1".to_string()));
    cli_spec1.arguments.push(Argument {
        name: "testarg1".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: Some("test_default".to_string()),
        help: None,
    });

    let mut cli_spec2 = CliSpec::new();
    cli_spec2.command.push(Command::Command("test".to_string()));
    cli_spec2.arguments.push(Argument {
        name: "testarg2".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
        help: None,
    });

    let empty_cli_spec = CliSpec::new();

    let result = parse_any(
        &vec!["test", "--test"],
        vec![&cli_spec1, &empty_cli_spec, &cli_spec2],
    );

    assert!(result.is_ok());
    let (index, cli_parsed) = result.unwrap();
    assert_eq!(index, 2);

    let mut argument_names = HashSet::new();
    argument_names.insert("testarg2".to_string());
    assert_eq!(cli_parsed.arguments, argument_names);
    assert!(cli_parsed.argument_values.is_empty());
}

#[test]
#[ignore]
fn parse_process_any_match() {
    let mut cli_spec1 = CliSpec::new();
    cli_spec1
        .command
        .push(Command::Command("test1".to_string()));
    cli_spec1.arguments.push(Argument {
        name: "testarg1".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: Some("test_default".to_string()),
        help: None,
    });

    let mut cli_spec2 = CliSpec::new();
    cli_spec2.command.push(Command::Command("test".to_string()));
    cli_spec2.arguments.push(Argument {
        name: "testarg2".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
        help: None,
    });

    let empty_cli_spec = CliSpec::new();

    env::set_var(
        "TEST_CLI_PARSER_COMMAND_LINE",
        vec!["test", "--test"].join(" "),
    );
    let result = parse_process_any(vec![&cli_spec1, &empty_cli_spec, &cli_spec2]);

    assert!(result.is_ok());
    let (index, cli_parsed) = result.unwrap();
    assert_eq!(index, 2);

    let mut argument_names = HashSet::new();
    argument_names.insert("testarg2".to_string());
    assert_eq!(cli_parsed.arguments, argument_names);
    assert!(cli_parsed.argument_values.is_empty());
}
