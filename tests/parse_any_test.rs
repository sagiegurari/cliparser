use cliparser::parse_any;
use cliparser::types::{Argument, ArgumentOccurrence, ArgumentValueType, CliSpec, Command};
use std::collections::HashSet;

#[test]
fn parse_any_test() {
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
