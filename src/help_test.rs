use super::*;
use crate::types::CliSpecMetaInfo;

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

    assert_eq!(
        text,
        r#"USAGE:
    "#
    );
}
