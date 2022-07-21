use super::*;

#[test]
fn display_invalid_command_line() {
    let error = ParserError::InvalidCommandLine("test".to_string());
    assert!(error.to_string().contains("test"));
}

#[test]
fn display_invalid_cli_spec() {
    let error = ParserError::InvalidCliSpec("test".to_string());
    assert!(error.to_string().contains("test"));
}

#[test]
fn display_command_does_not_match_spec() {
    let error = ParserError::CommandDoesNotMatchSpec("test".to_string());
    assert!(error.to_string().contains("test"));
}

#[test]
fn display_internal_error() {
    let error = ParserError::InternalError("test".to_string());
    assert!(error.to_string().contains("test"));
}

#[test]
fn clispecmetainfo_new() {
    let meta_info = CliSpecMetaInfo::new();

    assert!(meta_info.author.is_none());
    assert!(meta_info.version.is_none());
    assert!(meta_info.description.is_none());
    assert!(meta_info.project.is_none());
    assert!(meta_info.help_post_text.is_none());
}

#[test]
fn clispec_new() {
    let cli_spec = CliSpec::new();

    assert!(cli_spec.command.is_empty());
    assert!(cli_spec.arguments.is_empty());
    assert!(cli_spec.positional_argument.is_none());
    assert!(cli_spec.meta_info.is_none());
}

#[test]
fn clispec_builder_function() {
    let mut cli_spec1 = CliSpec::new();
    cli_spec1 = cli_spec1
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
        });

    let mut cli_spec2 = CliSpec::new();
    cli_spec2.meta_info = Some(CliSpecMetaInfo {
        author: Some("Sagie Gur-Ari".to_string()),
        version: Some("1.2.3-beta".to_string()),
        description: Some("Amazing example".to_string()),
        project: Some("example".to_string()),
        help_post_text: Some(
            "See more info at: https://github.com/sagiegurari/cargo-make".to_string(),
        ),
    });
    cli_spec2
        .command
        .push(Command::Command("makers".to_string()));
    cli_spec2.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "make".to_string(),
    ]));
    cli_spec2.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: Some(ArgumentHelp::TextAndParam(
            "The command line arguments".to_string(),
            "ARGS".to_string(),
        )),
    });
    cli_spec2.arguments.push(Argument {
        name: "flag".to_string(),
        key: vec!["--flag".to_string(), "-f".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
        help: Some(ArgumentHelp::Text(
            "A flag without value example".to_string(),
        )),
    });

    assert_eq!(cli_spec1, cli_spec2);
}

#[test]
fn cliparsed_new() {
    let cli_parsed = CliParsed::new();

    assert!(cli_parsed.arguments.is_empty());
    assert!(cli_parsed.argument_values.is_empty());
}

#[test]
fn cliparsed_get_first_none() {
    let cli_parsed = CliParsed::new();

    assert!(cli_parsed.get_first_value("test").is_none());
}

#[test]
fn cliparsed_get_first_empty() {
    let mut cli_parsed = CliParsed::new();
    cli_parsed
        .argument_values
        .insert("test".to_string(), vec![]);

    assert!(cli_parsed.get_first_value("test").is_none());
}

#[test]
fn cliparsed_get_first_multiple() {
    let mut cli_parsed = CliParsed::new();
    cli_parsed
        .argument_values
        .insert("test".to_string(), vec!["1".to_string(), "2".to_string()]);

    assert_eq!(cli_parsed.get_first_value("test").unwrap(), "1");
}
