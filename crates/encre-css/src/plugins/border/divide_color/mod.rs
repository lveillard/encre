#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
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

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn handle(&self, context: &mut ContextHandle) {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |context: &mut ContextHandle| match context.modifier {
                    Modifier::Builtin { value, .. } => {
                        let color =
                            color::get(context.config, value).unwrap();

                        context.buffer.line(format_args!("border-color: {color});"));
                    }
                    Modifier::Arbitrary { value, .. } => {
                        context.buffer.line(format_args!("border-color: {value});"));
                    }
                },
                " > :not([hidden]) ~ :not([hidden])",
            );
        });
    }
}
