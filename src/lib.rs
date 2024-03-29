#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    ambiguous_glob_imports,
    ambiguous_glob_reexports,
    ambiguous_wide_pointer_comparisons,
    anonymous_parameters,
    arithmetic_overflow,
    array_into_iter,
    asm_sub_register,
    async_fn_in_trait,
    bad_asm_style,
    bindings_with_variant_name,
    break_with_label_and_loop,
    byte_slice_in_packed_struct_with_derive,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    coherence_leak_check,
    conflicting_repr_hints,
    confusable_idents,
    const_evaluatable_unchecked,
    const_item_mutation,
    const_patterns_without_partial_eq,
    dead_code,
    deprecated,
    deprecated_cfg_attr_crate_type_name,
    deprecated_in_future,
    deprecated_where_clause_location,
    deref_into_dyn_supertrait,
    deref_nullptr,
    drop_bounds,
    dropping_copy_types,
    dropping_references,
    duplicate_macro_attributes,
    dyn_drop,
    elided_lifetimes_in_associated_constant,
    ellipsis_inclusive_range_patterns,
    enum_intrinsics_non_enums,
    explicit_outlives_requirements,
    exported_private_dependencies,
    ffi_unwind_calls,
    for_loops_over_fallibles,
    forbidden_lint_groups,
    forgetting_copy_types,
    forgetting_references,
    function_item_references,
    hidden_glob_reexports,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    incomplete_include,
    indirect_structural_match,
    ineffective_unstable_trait_impl,
    inline_no_sanitize,
    internal_features,
    invalid_atomic_ordering,
    invalid_doc_attributes,
    invalid_from_utf8,
    invalid_from_utf8_unchecked,
    invalid_macro_export_arguments,
    invalid_nan_comparisons,
    invalid_reference_casting,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    large_assignments,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    let_underscore_drop,
    let_underscore_lock,
    long_running_const_eval,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    map_unit_fn,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mixed_script_confusables,
    mutable_transmutes,
    named_arguments_used_positionally,
    named_asm_labels,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_fmt_panics,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    noop_method_call,
    opaque_hidden_inferred_bound,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_range_endpoints,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_bounds,
    private_interfaces,
    proc_macro_back_compat,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolons,
    refining_impl_trait,
    repr_transparent_external_private_fields,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    semicolon_in_expressions_from_macros,
    soft_unstable,
    special_module_name,
    stable_features,
    static_mut_ref,
    suspicious_auto_trait_impls,
    suspicious_double_ref_op,
    temporary_cstring_as_ptr,
    text_direction_codepoint_in_comment,
    text_direction_codepoint_in_literal,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_panic,
    unconditional_recursion,
    undefined_naked_function_abi,
    undropped_manually_drops,
    unexpected_cfgs,
    ungated_async_fn_track_caller,
    uninhabited_static,
    unit_bindings,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unstable_name_collisions,
    unstable_syntax_pre_expansion,
    unsupported_calling_conventions,
    unused_allocation,
    unused_assignments,
    unused_assignments,
    unused_associated_type_bounds,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_crate_dependencies,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macro_rules,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    useless_deprecated,
    useless_ptr_null_checks,
    where_clauses_object_safety,
    while_true,
    writes_through_immutable_pointer
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]

//! # cliparser
//!
//! Simple command line parser.
//!
//! This library provide a very simple API to parse command line arguments.<br>
//! It will not cover all possible use cases in order to ensure a simple thin API.
//!
//! # Examples
//!
//! ```
//! use cliparser::types::{
//!     Argument, ArgumentHelp, ArgumentOccurrence, ArgumentValueType, CliSpec, CliSpecMetaInfo,
//!     Command, PositionalArgument,
//! };
//! use cliparser::{help, parse, version};
//! use std::collections::{HashMap, HashSet};
//!
//! fn main() {
//!     let mut cli_spec = CliSpec::new();
//!
//!     // Add meta info to support help and version text generation
//!     cli_spec = cli_spec
//!         .set_meta_info(Some(CliSpecMetaInfo {
//!             author: Some("Sagie Gur-Ari".to_string()),
//!             version: Some("1.2.3-beta".to_string()),
//!             description: Some("Amazing example".to_string()),
//!             project: Some("example".to_string()),
//!             help_post_text: Some(
//!                 "See more info at: https://github.com/sagiegurari/cargo-make".to_string(),
//!             ),
//!         }))
//!         // Define the prefix of the arguments.
//!         // It can be a command/s (path prefix ignored) and/or a sub command/s
//!         // If not defined, the parsing will start checking the arguments only.
//!         // In this example, the spec defines two ways to invoke a process, either
//!         // as 'makers' or as 'cargo make' and afterwards the arguments.
//!         .add_command("makers")
//!         .add_subcommand(vec!["cargo", "make"])
//!         // Positional arguments come after all the known argument keys.
//!         // If the positional is None, positional arguments are not allowed.
//!         // Add -- to the command line forces positional arguments and stops key
//!         // based argument parsing.
//!         .set_positional_argument(Some(PositionalArgument {
//!             name: "args".to_string(),
//!             help: Some(ArgumentHelp::TextAndParam(
//!                 "The command line arguments".to_string(),
//!                 "ARGS".to_string(),
//!             )),
//!         }))
//!         // Add a 'flag' only argument which is an argument that does not accept any value.
//!         // You can define multiple variations of the parameter name.
//!         // For example, in this case putting --flag or -f in the command line would be parsed
//!         // as the 'flag' parameter.
//!         .add_argument(Argument {
//!             name: "flag".to_string(),
//!             key: vec!["--flag".to_string(), "-f".to_string()],
//!             argument_occurrence: ArgumentOccurrence::Single,
//!             value_type: ArgumentValueType::None,
//!             default_value: None,
//!             help: Some(ArgumentHelp::Text(
//!                 "A flag without value example".to_string(),
//!             )),
//!         })
//!         // Add an argument that accepts a single value, for example -s value
//!         .add_argument(Argument {
//!             name: "single".to_string(),
//!             key: vec!["--s1".to_string(), "-s".to_string()],
//!             argument_occurrence: ArgumentOccurrence::Single,
//!             value_type: ArgumentValueType::Single,
//!             default_value: None,
//!             help: Some(ArgumentHelp::Text(
//!                 "A parameter with single value example".to_string(),
//!             )),
//!         })
//!         // Add an argument that accepts multiple values
//!         .add_argument(Argument {
//!             name: "mo".to_string(),
//!             key: vec!["--mo1".to_string(), "-mo2".to_string()],
//!             argument_occurrence: ArgumentOccurrence::Multiple,
//!             value_type: ArgumentValueType::Single,
//!             default_value: None,
//!             help: Some(ArgumentHelp::Text(
//!                 "A parameter with multiple values example".to_string(),
//!             )),
//!         })
//!         // Add an argument that can appear multiple times.
//!         // Even if the value type if Single, multiple occurrences will
//!         // enable the argument to collect multiple values (one for each occurrence).
//!         .add_argument(Argument {
//!             name: "mv".to_string(),
//!             key: vec!["--mv1".to_string(), "-mv2".to_string()],
//!             argument_occurrence: ArgumentOccurrence::Single,
//!             value_type: ArgumentValueType::Multiple,
//!             default_value: None,
//!             help: Some(ArgumentHelp::Text(
//!                 "A parameter with single value but can appear multiple times example".to_string(),
//!             )),
//!         })
//!         // We can define a 'default' value.
//!         // In case the argument is not in the command line, we will get the default value.
//!         // However, the argument names list in the parsed struct will not include this
//!         // argument as it was not found. Only the argument values will contain it.
//!         // This is a good way to understand that we have a value but it was not entered by the caller.
//!         .add_argument(Argument {
//!             name: "default".to_string(),
//!             key: vec!["--d1".to_string(), "-d".to_string()],
//!             argument_occurrence: ArgumentOccurrence::Single,
//!             value_type: ArgumentValueType::Single,
//!             default_value: Some("some default".to_string()),
//!             help: Some(ArgumentHelp::Text(
//!                 "A parameter with default value example".to_string(),
//!             )),
//!         });
//!
//!     // Parsers the given command line based on the given spec and returns the result.
//!     // In case of invalid input or the provided spec does not match the command line, an error will be returned.
//!     // In order to parse the process command line instead of providing it, use the parse_process.
//!     // Also, if you want to provide multiple specs and have each one checked, until the first one
//!     // which fits is found, use the parse_any and parse_process_any functions.
//!     let result = parse(
//!         &vec![
//!             "cargo", "make", "-mv2", "4", "5", "6", "--mo1=1", "-mo2", "2", "-f", "-s", "3",
//!             "arg1", "arg2", "-mo2", "arg5",
//!         ],
//!         &cli_spec,
//!     );
//!
//!     // The CliParsed struct includes multiple members that define what was found
//!     // arguments - A collection of all arguments found (list of names not keys).
//!     // Arguments that were not found by defaulted to a given value will not be listed here.
//!     // argument_values - A map of all values for arguments found.
//!     // The map will exclude arguments that do not accept value but include arguments not provided
//!     // on the command line but were defaulted to a given value.
//!     // The map keys are the argument names (not keys) and the value is the list of all values
//!     // found for all occurrences.
//!     assert!(result.is_ok());
//!     let cli_parsed = result.unwrap();
//!     println!("Cli Parsed:\n{:?}", &cli_parsed);
//!
//!     let mut argument_names = HashSet::new();
//!     argument_names.insert("flag".to_string());
//!     argument_names.insert("single".to_string());
//!     argument_names.insert("mo".to_string());
//!     argument_names.insert("mv".to_string());
//!     argument_names.insert("args".to_string());
//!     let mut argument_values = HashMap::new();
//!     argument_values.insert("single".to_string(), vec!["3".to_string()]);
//!     argument_values.insert("mo".to_string(), vec!["1".to_string(), "2".to_string()]);
//!     argument_values.insert(
//!         "mv".to_string(),
//!         vec!["4".to_string(), "5".to_string(), "6".to_string()],
//!     );
//!     argument_values.insert(
//!         "args".to_string(),
//!         vec![
//!             "arg1".to_string(),
//!             "arg2".to_string(),
//!             "-mo2".to_string(),
//!             "arg5".to_string(),
//!         ],
//!     );
//!     argument_values.insert("default".to_string(), vec!["some default".to_string()]);
//!     assert_eq!(cli_parsed.arguments, argument_names);
//!     assert_eq!(cli_parsed.argument_values, argument_values);
//!
//!     // generate help text
//!     let help_text = help(&cli_spec);
//!     println!("{}", help_text);
//!
//!     // generate version text
//!     let version_text = version(&cli_spec);
//!     println!("{}", version_text);
//! }
//! ```
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! cliparser = "*"
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/cliparser/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/cliparser/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod help;
mod parser;
pub mod types;

use crate::types::{CliParsed, CliSpec, ParserError};

/// Parsers the given command line based on the given spec and returns the result.<br>
/// In case of error (such as invalid input), an error will be returned.<br>
/// In case the command line does not match the spec, Ok(None) will be returned.
///
/// # Example
///
/// ```
/// use cliparser::types::{
///     Argument, ArgumentHelp, ArgumentOccurrence, ArgumentValueType, CliSpec, CliSpecMetaInfo,
///     Command, PositionalArgument,
/// };
/// use cliparser::{help, parse, version};
/// use std::collections::{HashMap, HashSet};
///
/// fn main() {
///     let mut cli_spec = CliSpec::new();
///
///     // Add meta info to support help and version text generation
///     cli_spec = cli_spec
///         .set_meta_info(Some(CliSpecMetaInfo {
///             author: Some("Sagie Gur-Ari".to_string()),
///             version: Some("1.2.3-beta".to_string()),
///             description: Some("Amazing example".to_string()),
///             project: Some("example".to_string()),
///             help_post_text: Some(
///                 "See more info at: https://github.com/sagiegurari/cargo-make".to_string(),
///             ),
///         }))
///         // Define the prefix of the arguments.
///         // It can be a command/s (path prefix ignored) and/or a sub command/s
///         // If not defined, the parsing will start checking the arguments only.
///         // In this example, the spec defines two ways to invoke a process, either
///         // as 'makers' or as 'cargo make' and afterwards the arguments.
///         .add_command("makers")
///         .add_subcommand(vec!["cargo", "make"])
///         // Positional arguments come after all the known argument keys.
///         // If the positional is None, positional arguments are not allowed.
///         // Add -- to the command line forces positional arguments and stops key
///         // based argument parsing.
///         .set_positional_argument(Some(PositionalArgument {
///             name: "args".to_string(),
///             help: Some(ArgumentHelp::TextAndParam(
///                 "The command line arguments".to_string(),
///                 "ARGS".to_string(),
///             )),
///         }))
///         // Add a 'flag' only argument which is an argument that does not accept any value.
///         // You can define multiple variations of the parameter name.
///         // For example, in this case putting --flag or -f in the command line would be parsed
///         // as the 'flag' parameter.
///         .add_argument(Argument {
///             name: "flag".to_string(),
///             key: vec!["--flag".to_string(), "-f".to_string()],
///             argument_occurrence: ArgumentOccurrence::Single,
///             value_type: ArgumentValueType::None,
///             default_value: None,
///             help: Some(ArgumentHelp::Text(
///                 "A flag without value example".to_string(),
///             )),
///         })
///         // Add an argument that accepts a single value, for example -s value
///         .add_argument(Argument {
///             name: "single".to_string(),
///             key: vec!["--s1".to_string(), "-s".to_string()],
///             argument_occurrence: ArgumentOccurrence::Single,
///             value_type: ArgumentValueType::Single,
///             default_value: None,
///             help: Some(ArgumentHelp::Text(
///                 "A parameter with single value example".to_string(),
///             )),
///         })
///         // Add an argument that accepts multiple values
///         .add_argument(Argument {
///             name: "mo".to_string(),
///             key: vec!["--mo1".to_string(), "-mo2".to_string()],
///             argument_occurrence: ArgumentOccurrence::Multiple,
///             value_type: ArgumentValueType::Single,
///             default_value: None,
///             help: Some(ArgumentHelp::Text(
///                 "A parameter with multiple values example".to_string(),
///             )),
///         })
///         // Add an argument that can appear multiple times.
///         // Even if the value type if Single, multiple occurrences will
///         // enable the argument to collect multiple values (one for each occurrence).
///         .add_argument(Argument {
///             name: "mv".to_string(),
///             key: vec!["--mv1".to_string(), "-mv2".to_string()],
///             argument_occurrence: ArgumentOccurrence::Single,
///             value_type: ArgumentValueType::Multiple,
///             default_value: None,
///             help: Some(ArgumentHelp::Text(
///                 "A parameter with single value but can appear multiple times example".to_string(),
///             )),
///         })
///         // We can define a 'default' value.
///         // In case the argument is not in the command line, we will get the default value.
///         // However, the argument names list in the parsed struct will not include this
///         // argument as it was not found. Only the argument values will contain it.
///         // This is a good way to understand that we have a value but it was not entered by the caller.
///         .add_argument(Argument {
///             name: "default".to_string(),
///             key: vec!["--d1".to_string(), "-d".to_string()],
///             argument_occurrence: ArgumentOccurrence::Single,
///             value_type: ArgumentValueType::Single,
///             default_value: Some("some default".to_string()),
///             help: Some(ArgumentHelp::Text(
///                 "A parameter with default value example".to_string(),
///             )),
///         });
///
///     // Parsers the given command line based on the given spec and returns the result.
///     // In case of invalid input or the provided spec does not match the command line, an error will be returned.
///     // In order to parse the process command line instead of providing it, use the parse_process.
///     // Also, if you want to provide multiple specs and have each one checked, until the first one
///     // which fits is found, use the parse_any and parse_process_any functions.
///     let result = parse(
///         &vec![
///             "cargo", "make", "-mv2", "4", "5", "6", "--mo1=1", "-mo2", "2", "-f", "-s", "3",
///             "arg1", "arg2", "-mo2", "arg5",
///         ],
///         &cli_spec,
///     );
///
///     // The CliParsed struct includes multiple members that define what was found
///     // arguments - A collection of all arguments found (list of names not keys).
///     // Arguments that were not found by defaulted to a given value will not be listed here.
///     // argument_values - A map of all values for arguments found.
///     // The map will exclude arguments that do not accept value but include arguments not provided
///     // on the command line but were defaulted to a given value.
///     // The map keys are the argument names (not keys) and the value is the list of all values
///     // found for all occurrences.
///     assert!(result.is_ok());
///     let cli_parsed = result.unwrap();
///     println!("Cli Parsed:\n{:?}", &cli_parsed);
///
///     let mut argument_names = HashSet::new();
///     argument_names.insert("flag".to_string());
///     argument_names.insert("single".to_string());
///     argument_names.insert("mo".to_string());
///     argument_names.insert("mv".to_string());
///     argument_names.insert("args".to_string());
///     let mut argument_values = HashMap::new();
///     argument_values.insert("single".to_string(), vec!["3".to_string()]);
///     argument_values.insert("mo".to_string(), vec!["1".to_string(), "2".to_string()]);
///     argument_values.insert(
///         "mv".to_string(),
///         vec!["4".to_string(), "5".to_string(), "6".to_string()],
///     );
///     argument_values.insert(
///         "args".to_string(),
///         vec![
///             "arg1".to_string(),
///             "arg2".to_string(),
///             "-mo2".to_string(),
///             "arg5".to_string(),
///         ],
///     );
///     argument_values.insert("default".to_string(), vec!["some default".to_string()]);
///     assert_eq!(cli_parsed.arguments, argument_names);
///     assert_eq!(cli_parsed.argument_values, argument_values);
///
///     // generate help text
///     let help_text = help(&cli_spec);
///     println!("{}", help_text);
///
///     // generate version text
///     let version_text = version(&cli_spec);
///     println!("{}", version_text);
/// }
/// ```
pub fn parse(command_line: &Vec<&str>, spec: &CliSpec) -> Result<CliParsed, ParserError> {
    parser::parse(command_line, spec)
}

/// Parsers the given command line based on the given spec and returns the result.<br>
/// In case of error (such as invalid input), an error will be returned.<br>
/// In case the command line does not match the spec, Ok(None) will be returned.
pub fn parse_process(spec: &CliSpec) -> Result<CliParsed, ParserError> {
    parser::parse_process(spec)
}

/// Parsers the given command line based on the given specs and returns the result.<br>
/// In case of invalid input or none of the provided specs do not match the command line, an error will be returned.
pub fn parse_any(
    command_line: &Vec<&str>,
    specs: Vec<&CliSpec>,
) -> Result<(usize, CliParsed), ParserError> {
    parser::parse_any(command_line, specs)
}

/// Parsers the current process command line based on the given specs and returns the result.<br>
/// In case of invalid input or none of the provided specs do not match the command line, an error will be returned.
pub fn parse_process_any(specs: Vec<&CliSpec>) -> Result<(usize, CliParsed), ParserError> {
    parser::parse_process_any(specs)
}

/// Generates and returns the spec help text
pub fn help(spec: &CliSpec) -> String {
    help::help(spec)
}

/// Generates and returns the spec version text
pub fn version(spec: &CliSpec) -> String {
    help::version(spec)
}
