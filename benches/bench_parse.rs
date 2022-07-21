#![feature(test)]
extern crate test;

use cliparser::parse;
use cliparser::types::{
    Argument, ArgumentHelp, ArgumentOccurrence, ArgumentValueType, CliSpec, CliSpecMetaInfo,
    PositionalArgument,
};
use test::Bencher;

#[bench]
fn parse_test(bencher: &mut Bencher) {
    let mut cli_spec = CliSpec::new();

    cli_spec = cli_spec
        .set_meta_info(Some(CliSpecMetaInfo {
            author: Some("Sagie Gur-Ari".to_string()),
            version: Some("1.2.3-beta".to_string()),
            description: Some("Amazing example".to_string()),
            project: Some("example".to_string()),
            help_post_text: Some(
                "See more info at: https://github.com/sagiegurari/cargo-make".to_string(),
            ),
        }))
        .add_command("makers")
        .add_subcommand(vec!["cargo", "make"])
        .set_positional_argument(Some(PositionalArgument {
            name: "args".to_string(),
            help: Some(ArgumentHelp::TextAndParam(
                "The command line arguments".to_string(),
                "ARGS".to_string(),
            )),
        }))
        .add_argument(Argument {
            name: "flag".to_string(),
            key: vec!["--flag".to_string(), "-f".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "A flag without value example".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "single".to_string(),
            key: vec!["--s1".to_string(), "-s".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "A parameter with single value example".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "mo".to_string(),
            key: vec!["--mo1".to_string(), "-mo2".to_string()],
            argument_occurrence: ArgumentOccurrence::Multiple,
            value_type: ArgumentValueType::Single,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "A parameter with multiple values example".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "mv".to_string(),
            key: vec!["--mv1".to_string(), "-mv2".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Multiple,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "A parameter with single value but can appear multiple times example".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "default".to_string(),
            key: vec!["--d1".to_string(), "-d".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: Some("some default".to_string()),
            help: Some(ArgumentHelp::Text(
                "A parameter with default value example".to_string(),
            )),
        });

    let command_line = vec![
        "cargo", "make", "-mv2", "4", "5", "6", "--mo1=1", "-mo2", "2", "-f", "-s", "3", "arg1",
        "arg2", "-mo2", "arg5",
    ];

    bencher.iter(|| {
        let result = parse(&command_line, &cli_spec);
        assert!(result.is_ok());
    });
}
