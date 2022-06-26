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
fn cliparsed_new() {
    let cli_parsed = CliParsed::new();

    assert!(cli_parsed.arguments.is_empty());
    assert!(cli_parsed.argument_values.is_empty());
}
