//! # help
//!
//! Generates help/version text based on given spec.
//!

#[cfg(test)]
#[path = "./help_test.rs"]
mod help_test;

use crate::types::{CliSpec, Command};

/// Generates and returns the spec version text
pub(crate) fn version(spec: &CliSpec) -> String {
    let mut buffer = String::new();

    match spec.meta_info {
        Some(ref meta_info) => {
            if let Some(ref project) = meta_info.project {
                buffer.push_str(&project);
            }

            if let Some(ref version) = meta_info.version {
                if !buffer.is_empty() {
                    buffer.push_str(" ");
                }
                buffer.push_str(&version);
            }
        }
        None => (),
    }

    buffer
}

/// Generates and returns the spec help text
pub(crate) fn help(spec: &CliSpec) -> String {
    let mut buffer = version(spec);

    match spec.meta_info {
        Some(ref meta_info) => {
            if let Some(ref author) = meta_info.author {
                if !buffer.is_empty() {
                    buffer.push_str("\n");
                }
                buffer.push_str(&author);
            }

            if let Some(ref description) = meta_info.description {
                if !buffer.is_empty() {
                    buffer.push_str("\n");
                }
                buffer.push_str(&description);
            }
        }
        None => (),
    }

    if !buffer.is_empty() {
        buffer.push_str("\n\n");
    }

    // usage string
    buffer.push_str("USAGE:\n    ");
    let mut sub_buffer = String::new();
    let mut multiple = false;
    for command in &spec.command {
        if !sub_buffer.is_empty() {
            sub_buffer.push_str(" | ");
            multiple = true;
        }

        match command {
            Command::Command(value) => sub_buffer.push_str(value),
            Command::SubCommand(values) => sub_buffer.push_str(values.join(" ").as_str()),
        }
    }
    if multiple {
        buffer.push_str("[");
    }
    buffer.push_str(&sub_buffer);
    if multiple {
        buffer.push_str("]");
    }

    buffer // todo fix this
}
