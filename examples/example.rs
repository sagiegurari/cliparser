use cliparser::types::{
    Argument, ArgumentHelp, ArgumentOccurrence, ArgumentValueType, CliSpec, CliSpecMetaInfo,
    PositionalArgument,
};
use cliparser::{help, parse, version};
use std::collections::{HashMap, HashSet};

fn main() {
    let mut cli_spec = CliSpec::new();

    // Add meta info to support help and version text generation
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
        // Define the prefix of the arguments.
        // It can be a command/s (path prefix ignored) and/or a sub command/s
        // If not defined, the parsing will start checking the arguments only.
        // In this example, the spec defines two ways to invoke a process, either
        // as 'makers' or as 'cargo make' and afterwards the arguments.
        .add_command("makers")
        .add_subcommand(vec!["cargo", "make"])
        // Positional arguments come after all the known argument keys.
        // If the positional is None, positional arguments are not allowed.
        // Add -- to the command line forces positional arguments and stops key
        // based argument parsing.
        .set_positional_argument(Some(PositionalArgument {
            name: "args".to_string(),
            help: Some(ArgumentHelp::TextAndParam(
                "The command line arguments".to_string(),
                "ARGS".to_string(),
            )),
        }))
        // Add a 'flag' only argument which is an argument that does not accept any value.
        // You can define multiple variations of the parameter name.
        // For example, in this case putting --flag or -f in the command line would be parsed
        // as the 'flag' parameter.
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
        // Add an argument that accepts a single value, for example -s value
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
        // Add an argument that accepts multiple values
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
        // Add an argument that can appear multiple times.
        // Even if the value type if Single, multiple occurrences will
        // enable the argument to collect multiple values (one for each occurrence).
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
        // We can define a 'default' value.
        // In case the argument is not in the command line, we will get the default value.
        // However, the argument names list in the parsed struct will not include this
        // argument as it was not found. Only the argument values will contain it.
        // This is a good way to understand that we have a value but it was not entered by the caller.
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

    // Parsers the given command line based on the given spec and returns the result.
    // In case of invalid input or the provided spec does not match the command line, an error will be returned.
    // In order to parse the process command line instead of providing it, use the parse_process.
    // Also, if you want to provide multiple specs and have each one checked, until the first one
    // which fits is found, use the parse_any and parse_process_any functions.
    let result = parse(
        &vec![
            "cargo", "make", "-mv2", "4", "5", "6", "--mo1", "1", "-mo2", "2", "-f", "-s", "3",
            "arg1", "arg2", "-mo2", "arg5",
        ],
        &cli_spec,
    );

    // The CliParsed struct includes multiple members that define what was found
    // arguments - A collection of all arguments found (list of names not keys).
    // Arguments that were not found by defaulted to a given value will not be listed here.
    // argument_values - A map of all values for arguments found.
    // The map will exclude arguments that do not accept value but include arguments not provided
    // on the command line but were defaulted to a given value.
    // The map keys are the argument names (not keys) and the value is the list of all values
    // found for all occurrences.
    assert!(result.is_ok());
    let cli_parsed = result.unwrap();
    println!("Cli Parsed:\n{:?}", &cli_parsed);

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

    // generate help text
    let help_text = help(&cli_spec);
    println!("{}", help_text);

    // generate version text
    let version_text = version(&cli_spec);
    println!("{}", version_text);
}
