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
fn help_empty() {
    let cli_spec = CliSpec::new();

    let text = help(&cli_spec);

    assert_eq!(text, "\n");
}

#[test]
fn help_all_types() {
    let mut cli_spec = CliSpec::new();
    cli_spec.meta_info = Some(CliSpecMetaInfo {
        author: Some("Sagie Gur-Ari".to_string()),
        version: Some("1.2.3-beta".to_string()),
        description: Some("Amazing example".to_string()),
        project: Some("example".to_string()),
        help_post_text: Some(
            "See more info at: https://github.com/sagiegurari/cargo-make".to_string(),
        ),
    });
    cli_spec.command.push(Command::Command("test1".to_string()));
    cli_spec.command.push(Command::Command("test2".to_string()));
    cli_spec.command.push(Command::SubCommand(vec![
        "cargo".to_string(),
        "test".to_string(),
    ]));
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
        help: Some(ArgumentHelp::Text("A flag".to_string())),
    });
    cli_spec.arguments.push(Argument {
        name: "single".to_string(),
        key: vec!["--s1".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: Some(ArgumentHelp::Text("A single value arg".to_string())),
    });
    cli_spec.arguments.push(Argument {
        name: "mo".to_string(),
        key: vec!["--mo1".to_string(), "-mo2".to_string()],
        argument_occurrence: ArgumentOccurrence::Multiple,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: Some(ArgumentHelp::Text("A multi occurrence arg".to_string())),
    });
    cli_spec.arguments.push(Argument {
        name: "mv".to_string(),
        key: vec!["--mv1".to_string(), "-mv2".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Multiple,
        default_value: None,
        help: Some(ArgumentHelp::Text("A multi value arg".to_string())),
    });
    cli_spec.arguments.push(Argument {
        name: "default".to_string(),
        key: vec!["--d1".to_string(), "-d".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: Some("some default".to_string()),
        help: Some(ArgumentHelp::Text("A default value arg".to_string())),
    });
    cli_spec.arguments.push(Argument {
        name: "nohelp".to_string(),
        key: vec!["--nh".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "nohelp_with_default".to_string(),
        key: vec!["--nh2".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: Some("TEST".to_string()),
        help: None,
    });
    cli_spec.positional_argument = Some(PositionalArgument {
        name: "args".to_string(),
        help: Some(ArgumentHelp::TextAndParam(
            "helps a lot".to_string(),
            "ARGS".to_string(),
        )),
    });

    let text = help(&cli_spec);

    assert_eq!(
        text,
        r#"example 1.2.3-beta
Sagie Gur-Ari
Amazing example

USAGE:
    [test1 | test2 | cargo test] [OPTIONS] [--] [<ARGS>...]

ARGS:
    <ARGS>    helps a lot

OPTIONS:
    --flag, -f                     A flag
    --s1 <single>                  A single value arg
    --mo1, -mo2 <mo>               A multi occurrence arg
    --mv1, -mv2 <mv>               A multi value arg
    --d1, -d <default>             A default value arg [default: some default]
    --nh <nohelp>
    --nh2 <nohelp_with_default>    [default: TEST]

See more info at: https://github.com/sagiegurari/cargo-make
"#
    );
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
fn append_options_block_empty() {
    let cli_spec = CliSpec::new();

    let mut text = String::new();
    append_options_block(&cli_spec, &mut text);

    assert_eq!(text, "");
}

#[test]
fn append_options_block_all_types() {
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
        help: Some(ArgumentHelp::Text("A flag".to_string())),
    });
    cli_spec.arguments.push(Argument {
        name: "single".to_string(),
        key: vec!["--s1".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: Some(ArgumentHelp::Text("A single value arg".to_string())),
    });
    cli_spec.arguments.push(Argument {
        name: "mo".to_string(),
        key: vec!["--mo1".to_string(), "-mo2".to_string()],
        argument_occurrence: ArgumentOccurrence::Multiple,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: Some(ArgumentHelp::Text("A multi occurrence arg".to_string())),
    });
    cli_spec.arguments.push(Argument {
        name: "mv".to_string(),
        key: vec!["--mv1".to_string(), "-mv2".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Multiple,
        default_value: None,
        help: Some(ArgumentHelp::Text("A multi value arg".to_string())),
    });
    cli_spec.arguments.push(Argument {
        name: "default".to_string(),
        key: vec!["--d1".to_string(), "-d".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: Some("some default".to_string()),
        help: Some(ArgumentHelp::Text("A default value arg".to_string())),
    });
    cli_spec.arguments.push(Argument {
        name: "nohelp".to_string(),
        key: vec!["--nh".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: None,
        help: None,
    });
    cli_spec.arguments.push(Argument {
        name: "nohelp_with_default".to_string(),
        key: vec!["--nh2".to_string()],
        argument_occurrence: ArgumentOccurrence::Single,
        value_type: ArgumentValueType::Single,
        default_value: Some("TEST".to_string()),
        help: None,
    });

    let mut text = String::new();
    append_options_block(&cli_spec, &mut text);

    assert_eq!(
        text,
        r#"OPTIONS:
    --flag, -f                     A flag
    --s1 <single>                  A single value arg
    --mo1, -mo2 <mo>               A multi occurrence arg
    --mv1, -mv2 <mv>               A multi value arg
    --d1, -d <default>             A default value arg [default: some default]
    --nh <nohelp>
    --nh2 <nohelp_with_default>    [default: TEST]"#
    );
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
