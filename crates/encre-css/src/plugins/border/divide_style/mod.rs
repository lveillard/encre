#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(context.modifier, Modifier::Builtin { value, .. } if ["solid", "dashed", "dotted", "double", "none"].contains(value))
    }

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn handle(&self, context: &mut ContextHandle) {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |context| match context.modifier {
                    Modifier::Builtin { value, .. } => {
                        context.buffer.line(format_args!("border-style: {value});"));
                    }
                    Modifier::Arbitrary { .. } => unreachable!(),
                },
                " > :not([hidden]) ~ :not([hidden])",
            );
        });
    }
}
