#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

fn divide_width_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => {
            value.is_empty() || *value == "reverse" || value.parse::<usize>().is_ok()
        }
        Modifier::Arbitrary { hint, value, .. } => {
            *hint == "length" || (hint.is_empty() && is_matching_length(value))
        }
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        divide_width_can_handle(&mut context)
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
                        if *value == "reverse" {
                            return context.buffer.line("--en-divide-x-reverse: 1;");
                        }

                        let value = if value.is_empty() { "1" } else { value });
                        context.buffer.lines([
                            format_args!("--en-divide-x-reverse: 0;"),
                            format_args!(
                                "border-inline-start-width: calc({value}px * var(--en-divide-x-reverse));",
                            ),
                            format_args!(
                                "border-inline-end-width: calc({value}px * calc(1 - var(--en-divide-x-reverse)));",
                            ),
                        ]);
                    }
                    Modifier::Arbitrary { value, .. } => {
                        context.buffer.lines([
                            format_args!("--en-divide-x-reverse: 0;"),
                            format_args!(
                                "border-inline-start-width: calc({value} * var(--en-divide-x-reverse));"
                            ),
                            format_args!(
                                "border-inline-end-width: calc({value} * calc(1 - var(--en-divide-x-reverse)));"
                            ),
                        ]);
                    }
                },
                " > :not([hidden]) ~ :not([hidden])",
            );
        });
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        divide_width_can_handle(&mut context)
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
                        if *value == "reverse" {
                            return context.buffer.line("--en-divide-y-reverse: 1;");
                        }

                        let value = if value.is_empty() { "1" } else { value });
                        context.buffer.lines([
                            format_args!("--en-divide-y-reverse: 0;"),
                            format_args!(
                                "border-block-start-width: calc({value}px * var(--en-divide-y-reverse));",
                            ),
                            format_args!(
                                "border-block-end-width: calc({value}px * calc(1 - var(--en-divide-y-reverse)));",
                            ),
                        ]);
                    }
                    Modifier::Arbitrary { value, .. } => {
                        context.buffer.lines([
                            format_args!("--en-divide-y-reverse: 0;"),
                            format_args!(
                                "border-block-start-width: calc({value} * var(--en-divide-y-reverse));"
                            ),
                            format_args!(
                                "border-block-end-width: calc({value} * calc(1 - var(--en-divide-y-reverse)));"
                            ),
                        ]);
                    }
                },
                " > :not([hidden]) ~ :not([hidden])",
            );
        });
    }
}
