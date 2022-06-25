use super::*;
use crate::types::{ArgumentOccurrence, CliSpecMetaInfo};

#[test]
fn version_empty() {
    let cli_spec = CliSpec::new();

    let text = version(&cli_spec);

    assert_eq!(text, "");
}

#[test]
fn version_only_project() {
    let mut meta_info = CliSpecMetaInfo::new();
    meta_info.project = Some("test-project".to_string());

    let mut cli_spec = CliSpec::new();
    cli_spec.meta_info = Some(meta_info);

    let text = version(&cli_spec);

    assert_eq!(text, "test-project");
}

#[test]
fn version_only_version() {
    let mut meta_info = CliSpecMetaInfo::new();
    meta_info.version = Some("3.2.1-beta".to_string());

    let mut cli_spec = CliSpec::new();
    cli_spec.meta_info = Some(meta_info);

    let text = version(&cli_spec);

    assert_eq!(text, "3.2.1-beta");
}

#[test]
fn version_all() {
    let mut meta_info = CliSpecMetaInfo::new();
    meta_info.project = Some("test-project".to_string());
    meta_info.version = Some("3.2.1-beta".to_string());

    let mut cli_spec = CliSpec::new();
    cli_spec.meta_info = Some(meta_info);

    let text = version(&cli_spec);

    assert_eq!(text, "test-project 3.2.1-beta");
}

#[test]
fn append_usage_line_empty() {
    let cli_spec = CliSpec::new();

    let mut text = String::new();
    append_usage_line(&cli_spec, &mut text);

    assert_eq!(text, "");
}

#[test]
fn append_usage_line_single_command() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::Command("test".to_string()));

    let mut text = String::new();
    append_usage_line(&cli_spec, &mut text);

    assert_eq!(
        text,
        r#"USAGE:
    test"#
    );
}

#[test]
fn append_usage_line_single_sub_command() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "test".to_string(),
    ]));

    let mut text = String::new();
    append_usage_line(&cli_spec, &mut text);

    assert_eq!(
        text,
        r#"USAGE:
    cargo test"#
    );
}

#[test]
fn append_usage_line_multiple_commands() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::Command("test1".to_string()));
    cli_spec.command.push(Command::Command("test2".to_string()));
    cli_spec.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "test".to_string(),
    ]));

    let mut text = String::new();
    append_usage_line(&cli_spec, &mut text);

    assert_eq!(
        text,
        r#"USAGE:
    [test1 | test2 | cargo test]"#
    );
}

#[test]
fn append_usage_line_only_arguments() {
    let mut cli_spec = CliSpec::new();
    cli_spec.arguments.push(Argument {
        name: "flag".to_string(),
        key: vec!["--flag".to_string(), "-f".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
        help: None,
    });

    let mut text = String::new();
    append_usage_line(&cli_spec, &mut text);

    assert_eq!(
        text,
        r#"USAGE:
    [OPTIONS]"#
    );
}

#[test]
fn append_usage_line_only_positional_arguments_no_value_name() {
    let mut cli_spec = CliSpec::new();
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: None,
    });

    let mut text = String::new();
    append_usage_line(&cli_spec, &mut text);

    assert_eq!(
        text,
        r#"USAGE:
    [--] [<args>...]"#
    );
}

#[test]
fn append_usage_line_all() {
    let mut cli_spec = CliSpec::new();
    cli_spec.command.push(Command::Command("test1".to_string()));
    cli_spec.command.push(Command::Command("test2".to_string()));
    cli_spec.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "test".to_string(),
    ]));
    cli_spec.arguments.push(Argument {
        name: "flag".to_string(),
        key: vec!["--flag".to_string(), "-f".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
        help: None,
    });
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: Some(ArgumentHelp::TextAndParam(
            "help".to_string(),
            "ARGS".to_string(),
        )),
    });

    let mut text = String::new();
    append_usage_line(&cli_spec, &mut text);

    assert_eq!(
        text,
        r#"USAGE:
    [test1 | test2 | cargo test] [OPTIONS] [--] [<ARGS>...]"#
    );
}

#[test]
fn append_args_line_empty() {
    let cli_spec = CliSpec::new();

    let mut text = String::new();
    append_args_line(&cli_spec, &mut text);

    assert_eq!(text, "");
}

#[test]
fn append_args_line_args_without_help() {
    let mut cli_spec = CliSpec::new();
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: None,
    });

    let mut text = String::new();
    append_args_line(&cli_spec, &mut text);

    assert_eq!(
        text,
        r#"ARGS:
    <args>"#
    );
}

#[test]
fn append_args_line_args_with_help_text() {
    let mut cli_spec = CliSpec::new();
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: Some(ArgumentHelp::Text("helps a lot".to_string())),
    });

    let mut text = String::new();
    append_args_line(&cli_spec, &mut text);

    assert_eq!(
        text,
        r#"ARGS:
    <args>    helps a lot"#
    );
}

#[test]
fn append_args_line_args_with_help_text_and_value_name() {
    let mut cli_spec = CliSpec::new();
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: Some(ArgumentHelp::TextAndParam(
            "helps a lot".to_string(),
            "ARGS".to_string(),
        )),
    });

    let mut text = String::new();
    append_args_line(&cli_spec, &mut text);

    assert_eq!(
        text,
        r#"ARGS:
    <ARGS>    helps a lot"#
    );
}

#[test]
fn append_options_line_empty() {
    let cli_spec = CliSpec::new();

    let mut text = String::new();
    append_options_line(&cli_spec, &mut text);

    assert_eq!(text, "");
}

#[test]
fn get_positional_argument_value_name_no_help() {
    let text = get_positional_argument_value_name(&PositionalArgument {
        name: "test".to_string(),
        help: None,
    });

    assert_eq!(text, "test");
}

#[test]
fn get_positional_argument_value_name_help_text() {
    let text = get_positional_argument_value_name(&PositionalArgument {
        name: "test".to_string(),
        help: Some(ArgumentHelp::Text("help".to_string())),
    });

    assert_eq!(text, "test");
}

#[test]
fn get_positional_argument_value_name_help_text_and_name() {
    let text = get_positional_argument_value_name(&PositionalArgument {
        name: "test".to_string(),
        help: Some(ArgumentHelp::TextAndParam(
            "help".to_string(),
            "ARGS".to_string(),
        )),
    });

    assert_eq!(text, "ARGS");
}

#[test]
fn get_argument_value_name_no_value() {
    let text = get_argument_value_name(&Argument {
        name: "flag".to_string(),
        key: vec!["--flag".to_string(), "-f".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::None,
        default_value: None,
        help: None,
    });

    assert!(text.is_none());
}

#[test]
fn get_argument_value_name_no_help() {
    let text = get_argument_value_name(&Argument {
        name: "flag".to_string(),
        key: vec!["--flag".to_string(), "-f".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });

    assert_eq!(text.unwrap(), "flag");
}

#[test]
fn get_argument_value_name_help_text() {
    let text = get_argument_value_name(&Argument {
        name: "flag".to_string(),
        key: vec!["--flag".to_string(), "-f".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: Some(ArgumentHelp::Text("help".to_string())),
    });

    assert_eq!(text.unwrap(), "flag");
}

#[test]
fn get_argument_value_name_help_text_and_name() {
    let text = get_argument_value_name(&Argument {
        name: "flag".to_string(),
        key: vec!["--flag".to_string(), "-f".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: Some(ArgumentHelp::TextAndParam(
            "help".to_string(),
            "VALUE".to_string(),
        )),
    });

    assert_eq!(text.unwrap(), "VALUE");
}
