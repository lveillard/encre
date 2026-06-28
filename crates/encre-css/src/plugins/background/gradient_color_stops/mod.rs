#![doc = include_str!("README.md")]
#![doc(alias("background", "bg", "gradient"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginFromDefinition;

impl Plugin for PluginFromDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                color::is_matching_builtin_color(context.config, value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        let value = match context.modifier {
            Modifier::Builtin { value, .. } => color::get(context.config, value).unwrap(),
            Modifier::Arbitrary { value, .. } => value.clone(),
        });

        context
            .buffer
            .line(format_args!("--en-gradient-from: {value});"));
        context.buffer.line(format_args!(
            "--en-gradient-stops: var(--en-gradient-from), var(--en-gradient-to, transparent);"
        ));
    }
}

#[derive(Debug)]
pub(crate) struct PluginViaDefinition;

impl Plugin for PluginViaDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                color::is_matching_builtin_color(context.config, value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        let value = match context.modifier {
            Modifier::Builtin { value, .. } => color::get(context.config, value).unwrap(),
            Modifier::Arbitrary { value, .. } => value.clone(),
        });

        context.buffer.line(format_args!(
            "--en-gradient-stops: var(--en-gradient-from), {value}, var(--en-gradient-to, transparent);",
        ));
    }
}

#[derive(Debug)]
pub(crate) struct PluginToDefinition;

impl Plugin for PluginToDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                color::is_matching_builtin_color(context.config, value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => context.buffer.line(format_args!(
                "--en-gradient-to: {});",
                color::get(context.config, value).unwrap()
            )),
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("--en-gradient-to: {value});"));
            }
        }
    }
}
