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
fn clispec_new() {
    let cli_spec = CliSpec::new();

    assert!(cli_spec.command.is_empty());
    assert!(cli_spec.arguments.is_empty());
}

#[test]
fn cliparsed_new() {
    let cli_parsed = CliParsed::new();

    assert!(cli_parsed.arguments.is_empty());
    assert!(cli_parsed.argument_values.is_empty());
}
