//! # help
//!
//! Generates help/version text based on given spec.
//!

#[cfg(test)]
#[path = "./help_test.rs"]
mod help_test;

use crate::types::{
    Argument, ArgumentHelp, ArgumentValueType, CliSpec, Command, PositionalArgument,
};

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

    if append_usage_line(spec, &mut buffer) {
        buffer.push_str("\n\n");
    }
    if append_args_line(spec, &mut buffer) {
        buffer.push_str("\n\n");
    }
    if append_options_block(spec, &mut buffer) {
        buffer.push_str("\n\n");
    }

    match spec.meta_info {
        Some(ref meta_info) => match meta_info.help_post_text {
            Some(ref text) => buffer.push_str(text),
            None => (),
        },
        None => (),
    }

    buffer = buffer.trim().to_string();
    buffer.push_str("\n");

    buffer
}

fn append_usage_line(spec: &CliSpec, buffer: &mut String) -> bool {
    if spec.command.is_empty() && spec.arguments.is_empty() && spec.positional_argument.is_none() {
        return false;
    }

    buffer.push_str("USAGE:\n    ");
    let mut sub_buffer = String::new();
    let mut multiple = false;
    let mut added_content = false;
    for command in &spec.command {
        if !sub_buffer.is_empty() {
            sub_buffer.push_str(" | ");
            multiple = true;
            added_content = true;
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

    if !spec.arguments.is_empty() {
        if added_content {
            buffer.push_str(" ");
        }
        added_content = true;
        buffer.push_str("[OPTIONS]");
    }

    if let Some(ref positional_argument_spec) = spec.positional_argument {
        let name = get_positional_argument_value_name(positional_argument_spec);

        if added_content {
            buffer.push_str(" ");
        }
        buffer.push_str("[--] [<");
        buffer.push_str(&name);
        buffer.push_str(">...]");
    }

    true
}

fn append_args_line(spec: &CliSpec, buffer: &mut String) -> bool {
    if let Some(ref positional_argument_spec) = spec.positional_argument {
        let name = get_positional_argument_value_name(positional_argument_spec);

        buffer.push_str("ARGS:\n    <");
        buffer.push_str(&name);
        buffer.push_str(">");

        if let Some(ref help) = positional_argument_spec.help {
            buffer.push_str("    ");

            match help {
                ArgumentHelp::Text(text) => buffer.push_str(text),
                ArgumentHelp::TextAndParam(text, _) => buffer.push_str(text),
            }
        }

        true
    } else {
        false
    }
}

fn append_options_block(spec: &CliSpec, buffer: &mut String) -> bool {
    if !spec.arguments.is_empty() {
        let mut names = vec![];
        let mut max_width = 0;

        for argument in &spec.arguments {
            let mut sub_buffer = String::new();
            sub_buffer.push_str("    ");
            let mut added = false;
            for key in &argument.key {
                if added {
                    sub_buffer.push_str(", ");
                }
                added = true;
                sub_buffer.push_str(&key);
            }

            let value_name = get_argument_value_name(argument);
            if let Some(name) = value_name {
                sub_buffer.push_str(" <");
                sub_buffer.push_str(&name);
                sub_buffer.push_str(">");
            }

            let name_len = sub_buffer.len();
            names.push(sub_buffer);
            if max_width < name_len {
                max_width = name_len;
            }
        }

        buffer.push_str("OPTIONS:\n");

        let mut index = 0;
        let help_offset = max_width + 4;
        for argument in &spec.arguments {
            if index > 0 {
                buffer.push_str("\n");
            }

            let help_text = match argument.help {
                Some(ref help) => {
                    let mut text = match help {
                        ArgumentHelp::Text(ref text) => text.to_string(),
                        ArgumentHelp::TextAndParam(ref text, _) => text.to_string(),
                    };
                    if !text.is_empty() {
                        text.push_str(" ");
                    }

                    text
                }
                None => "".to_string(),
            };

            let default_text = match argument.value_type {
                ArgumentValueType::None => "".to_string(),
                _ => match argument.default_value {
                    Some(ref value) => format!("[default: {}]", value),
                    None => "".to_string(),
                },
            };

            let line = format!(
                "{:<help_offset$}{}{}",
                &names[index],
                &help_text,
                &default_text,
                help_offset = help_offset
            );
            buffer.push_str(&line.trim_end());
            index = index + 1;
        }

        true
    } else {
        false
    }
}

fn get_positional_argument_value_name(positional_argument_spec: &PositionalArgument) -> String {
    match positional_argument_spec.help {
        Some(ref help) => match help {
            ArgumentHelp::TextAndParam(_, value) => value.to_string(),
            ArgumentHelp::Text(_) => positional_argument_spec.name.clone(),
        },
        None => positional_argument_spec.name.clone(),
    }
}

fn get_argument_value_name(argument_spec: &Argument) -> Option<String> {
    if argument_spec.value_type == ArgumentValueType::None {
        None
    } else {
        let name = match argument_spec.help {
            Some(ref help) => match help {
                ArgumentHelp::TextAndParam(_, value) => value.to_string(),
                ArgumentHelp::Text(_) => argument_spec.name.clone(),
            },
            None => argument_spec.name.clone(),
        };

        Some(name)
    }
}
