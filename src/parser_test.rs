use super::*;
use crate::types::PositionalArgument;
use std::collections::{HashMap, HashSet};

#[test]
fn parse_invalid_input() {
    let cli_spec = CliSpec::new();
    let result = parse(&vec![], &cli_spec);

    assert!(result.is_err());
}

#[test]
fn parse_default_value() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "testarg".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: Some("test_default".to_string()),
        help: None,
    });

    let result = parse(&vec![], &cli_spec);

    assert!(result.is_ok());
    let cli_parsed = result.unwrap();

    let mut argument_values = HashMap::new();
    argument_values.insert("testarg".to_string(), vec!["test_default".to_string()]);
    assert_eq!(cli_parsed.arguments, HashSet::new());
    assert_eq!(cli_parsed.argument_values, argument_values);
}

#[test]
fn parse_combination() {
    let mut cli_spec = CliSpec::new();
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: None,
    });
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
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: None,
    });
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
fn parse_any_empty_vec() {
    let result = parse_any(&vec![], vec![]);

    assert!(result.is_err());
}

#[test]
fn parse_any_invalid_input() {
    let cli_spec = CliSpec::new();
    let result = parse_any(&vec![], vec![&cli_spec]);

    assert!(result.is_err());
}

#[test]
fn parse_any_no_match() {
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
    cli_spec2
        .command
        .push(Command::Command("test2".to_string()));
    cli_spec2.arguments.push(Argument {
        name: "testarg2".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
        help: None,
    });

    let result = parse_any(&vec!["test"], vec![&cli_spec1, &cli_spec2]);

    assert!(result.is_err());
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

#[test]
fn parse_no_match_command() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::Command("test".to_string()));
    let result = parse(&vec!["other"], &cli_spec);

    assert!(result.is_err());
}

#[test]
fn parse_only_command() {
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
fn insert_default_values_empty() {
    let cli_spec = CliSpec::new();
    let mut cli_parsed = CliParsed::new();

    insert_default_values(&cli_spec, &mut cli_parsed);

    assert!(cli_parsed.arguments.is_empty());
    assert!(cli_parsed.argument_values.is_empty());
}

#[test]
fn insert_default_values_all_types() {
    let mut cli_spec = CliSpec::new();
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "flag".to_string(),
        key: vec!["--flag".to_string(), "-f".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: Some("flag_default".to_string()),
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "single".to_string(),
        key: vec!["--s1".to_string(), "-s".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: Some("single_default".to_string()),
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "mo".to_string(),
        key: vec!["--mo1".to_string(), "-mo2".to_string()],
        argument_occurrence: ArgumentOccurrence::Multiple,
        value_type: ArgumentValueType::Single,
        default_value: Some("mo_default".to_string()),
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "mv".to_string(),
        key: vec!["--mv1".to_string(), "-mv2".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Multiple,
        default_value: Some("mv_default".to_string()),
        help: None,
    });

    let mut cli_parsed = CliParsed::new();

    insert_default_values(&cli_spec, &mut cli_parsed);

    let mut argument_values = HashMap::new();
    argument_values.insert("single".to_string(), vec!["single_default".to_string()]);
    argument_values.insert("mo".to_string(), vec!["mo_default".to_string()]);
    argument_values.insert("mv".to_string(), vec!["mv_default".to_string()]);
    assert!(cli_parsed.arguments.is_empty());
    assert_eq!(cli_parsed.argument_values, argument_values);
}

#[test]
fn parse_command_empty_command() {
    let cli_spec = CliSpec::new();
    let (valid, index) = parse_command(&vec!["test"], &cli_spec);

    assert!(valid);
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
        help: None,
    });
    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec![], &cli_spec, &mut cli_parsed);

    assert!(result.is_ok());
    assert!(cli_parsed.arguments.is_empty());
    assert!(cli_parsed.argument_values.is_empty());
}

#[test]
fn parse_arguments_non_empty_arguments_line_but_no_args_in_spec() {
    let cli_spec = CliSpec::new();
    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec!["test"], &cli_spec, &mut cli_parsed);

    assert!(result.is_err());
}

#[test]
fn parse_arguments_positional_only() {
    let mut cli_spec = CliSpec::new();
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: None,
    });
    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec!["1", "2", "3"], &cli_spec, &mut cli_parsed);

    assert!(result.is_ok());

    let mut argument_names = HashSet::new();
    argument_names.insert("args".to_string());
    let mut argument_values = HashMap::new();
    argument_values.insert(
        "args".to_string(),
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, argument_values);
}

#[test]
fn parse_arguments_positional_only_with_separator() {
    let mut cli_spec = CliSpec::new();
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: None,
    });
    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec!["--", "1", "2", "3"], &cli_spec, &mut cli_parsed);

    assert!(result.is_ok());

    let mut argument_names = HashSet::new();
    argument_names.insert("args".to_string());
    let mut argument_values = HashMap::new();
    argument_values.insert(
        "args".to_string(),
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, argument_values);
}

#[test]
fn parse_arguments_non_value_param_only() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "testarg".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
        help: None,
    });

    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec!["--test"], &cli_spec, &mut cli_parsed);

    assert!(result.is_ok());

    let mut argument_names = HashSet::new();
    argument_names.insert("testarg".to_string());
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, HashMap::new());
}

#[test]
fn parse_arguments_non_value_param_only_with_unsupported_value() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "testarg".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
        help: None,
    });

    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec!["--test", "value"], &cli_spec, &mut cli_parsed);

    assert!(result.is_err());
}

#[test]
fn parse_arguments_single_value_param_only() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "testarg".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });

    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec!["--test", "value"], &cli_spec, &mut cli_parsed);

    assert!(result.is_ok());

    let mut argument_names = HashSet::new();
    argument_names.insert("testarg".to_string());
    let mut argument_values = HashMap::new();
    argument_values.insert("testarg".to_string(), vec!["value".to_string()]);
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, argument_values);
}

#[test]
fn parse_arguments_single_value_param_with_equals_only() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "testarg".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });

    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec!["--test=value"], &cli_spec, &mut cli_parsed);

    assert!(result.is_ok());

    let mut argument_names = HashSet::new();
    argument_names.insert("testarg".to_string());
    let mut argument_values = HashMap::new();
    argument_values.insert("testarg".to_string(), vec!["value".to_string()]);
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, argument_values);
}

#[test]
fn parse_arguments_single_value_param_only_with_multiple_values() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "testarg".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });

    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec!["--test", "1", "2"], &cli_spec, &mut cli_parsed);

    assert!(result.is_err());
}

#[test]
fn parse_arguments_multi_value_param_only() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "testarg".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Multiple,
        default_value: None,
        help: None,
    });

    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec!["--test", "1", "2", "3"], &cli_spec, &mut cli_parsed);

    assert!(result.is_ok());

    let mut argument_names = HashSet::new();
    argument_names.insert("testarg".to_string());
    let mut argument_values = HashMap::new();
    argument_values.insert(
        "testarg".to_string(),
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, argument_values);
}

#[test]
fn parse_arguments_multi_value_param_only_no_values() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "testarg".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Multiple,
        default_value: None,
        help: None,
    });

    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(&vec!["--test"], &cli_spec, &mut cli_parsed);

    assert!(result.is_ok());

    let mut argument_names = HashSet::new();
    argument_names.insert("testarg".to_string());
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, HashMap::new());
}

#[test]
fn parse_arguments_single_occurrence_multiple_times_on_cli() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "testarg".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });

    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(
        &vec!["--test", "1", "--test", "2"],
        &cli_spec,
        &mut cli_parsed,
    );

    assert!(result.is_err());
}

#[test]
fn parse_arguments_multi_occurence_param_only() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "testarg".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Multiple,
        value_type: ArgumentValueType::Multiple,
        default_value: None,
        help: None,
    });

    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(
        &vec!["--test", "1", "2", "--test", "3"],
        &cli_spec,
        &mut cli_parsed,
    );

    assert!(result.is_ok());

    let mut argument_names = HashSet::new();
    argument_names.insert("testarg".to_string());
    let mut argument_values = HashMap::new();
    argument_values.insert(
        "testarg".to_string(),
        vec!["1".to_string(), "2".to_string(), "3".to_string()],
    );
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, argument_values);
}

#[test]
fn parse_arguments_single_value_param_inside_positional() {
    let mut cli_spec = CliSpec::new();
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "testarg".to_string(),
        key: vec!["--test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });

    let mut cli_parsed = CliParsed::new();
    let mut result = parse_arguments(
        &vec!["something", "--test", "value"],
        &cli_spec,
        &mut cli_parsed,
    );

    assert!(result.is_ok());

    let mut argument_names = HashSet::new();
    argument_names.insert("args".to_string());
    let mut argument_values = HashMap::new();
    argument_values.insert(
        "args".to_string(),
        vec![
            "something".to_string(),
            "--test".to_string(),
            "value".to_string(),
        ],
    );
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, argument_values);

    cli_parsed = CliParsed::new();
    result = parse_arguments(&vec!["--", "--test", "value"], &cli_spec, &mut cli_parsed);

    assert!(result.is_ok());

    argument_names = HashSet::new();
    argument_names.insert("args".to_string());
    argument_values = HashMap::new();
    argument_values.insert(
        "args".to_string(),
        vec!["--test".to_string(), "value".to_string()],
    );
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, argument_values);
}

#[test]
fn parse_arguments_combination() {
    let mut cli_spec = CliSpec::new();
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: None,
    });
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

    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(
        &vec![
            "-mv2", "4", "5", "6", "--mo1", "1", "-mo2", "2", "-f", "-s", "3", "arg1", "arg2",
            "-mo2", "arg5",
        ],
        &cli_spec,
        &mut cli_parsed,
    );

    assert!(result.is_ok());

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
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, argument_values);
}

#[test]
fn parse_arguments_multiple_keys() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "testarg".to_string(),
        key: vec!["--test1".to_string(), "--test2".to_string()],
        argument_occurrence: ArgumentOccurrence::Multiple,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });

    let mut cli_parsed = CliParsed::new();
    let result = parse_arguments(
        &vec!["--test1", "1", "--test2", "2"],
        &cli_spec,
        &mut cli_parsed,
    );

    assert!(result.is_ok());

    let mut argument_names = HashSet::new();
    argument_names.insert("testarg".to_string());
    let mut argument_values = HashMap::new();
    argument_values.insert(
        "testarg".to_string(),
        vec!["1".to_string(), "2".to_string()],
    );
    assert_eq!(cli_parsed.arguments, argument_names);
    assert_eq!(cli_parsed.argument_values, argument_values);
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
fn validate_input_empty_command_line_with_command() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::Command("cargo".to_string()));
    let result = validate_input(&vec![], &cli_spec);

    assert!(result.is_err());
}

#[test]
fn validate_input_empty_command_line_no_command() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "test".to_string(),
        key: vec!["test".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
        help: None,
    });
    let result = validate_input(&vec![], &cli_spec);

    assert!(result.is_ok());
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
        help: None,
    });
    let result = validate_input(&vec!["test"], &cli_spec);

    assert!(result.is_ok());
}
