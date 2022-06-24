#![feature(test)]
extern crate test;

use cliparser::parse;
use cliparser::types::{Argument, ArgumentOccurrence, ArgumentValueType, CliSpec, Command};
use test::Bencher;

#[bench]
fn parse_test(bencher: &mut Bencher) {
    let mut cli_spec = CliSpec::new();

    cli_spec
        .command
        .push(Command::Command("makers".to_string()));
    cli_spec.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "make".to_string(),
    ]));

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

    let command_line = vec![
        "cargo", "make", "-mv2", "4", "5", "6", "--mo1", "1", "-mo2", "2", "-f", "-s", "3", "arg1",
        "arg2", "-mo2", "arg5",
    ];

    bencher.iter(|| {
        let result = parse(&command_line, &cli_spec);
        assert!(result.is_ok());
    });
}
