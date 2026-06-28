#![doc = include_str!("README.md")]
#![doc(alias("background", "bg", "gradient"))]
use std::borrow::Cow;

use crate::prelude::build_plugin::*;

const INTERPOLATION_MODES: &[&str] = &[
    "srgb",
    "hsl",
    "oklab",
    "oklch",
    "longer",
    "shorter",
    "increasing",
    "decreasing",
];

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "none",
                "gradient-to-t",
                "gradient-to-tr",
                "gradient-to-r",
                "gradient-to-br",
                "gradient-to-b",
                "gradient-to-bl",
                "gradient-to-l",
                "gradient-to-tl",
            ]
            .contains(value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "image" || *hint == "url" || (hint.is_empty() && is_matching_image(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "none" => context.buffer.line("background-image: none;"),
                "gradient-to-t" => context
                    .buffer
                    .line("background-image: linear-gradient(to top in oklab, var(--en-gradient-stops));"),
                "gradient-to-tr" => context.buffer.line(
                    "background-image: linear-gradient(to top right in oklab, var(--en-gradient-stops));",
                ),
                "gradient-to-r" => context
                    .buffer
                    .line("background-image: linear-gradient(to right in oklab, var(--en-gradient-stops));"),
                "gradient-to-br" => context.buffer.line(
                    "background-image: linear-gradient(to bottom right in oklab, var(--en-gradient-stops));",
                ),
                "gradient-to-b" => context.buffer.line(
                    "background-image: linear-gradient(to bottom in oklab, var(--en-gradient-stops));",
                ),
                "gradient-to-bl" => context.buffer.line(
                    "background-image: linear-gradient(to bottom left in oklab, var(--en-gradient-stops));",
                ),
                "gradient-to-l" => context
                    .buffer
                    .line("background-image: linear-gradient(to left in oklab, var(--en-gradient-stops));"),
                "gradient-to-tl" => context.buffer.line(
                    "background-image: linear-gradient(to top left in oklab, var(--en-gradient-stops));",
                ),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("background-image: {value});"));
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct PluginLinearDefinition;

impl Plugin for PluginLinearDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                let (gradient_type, interpolation_mode) = if let Some(index) = value.find('/') {
                    let (before, after) = value.split_at(index);
                    (before, &after[1..])
                } else {
                    (*value, "oklab")
                });

                ([
                    "to-t", "to-tr", "to-r", "to-br", "to-b", "to-bl", "to-l", "to-tl",
                ]
                .contains(&gradient_type)
                    || gradient_type.parse::<usize>().is_ok())
                    && INTERPOLATION_MODES.contains(&interpolation_mode)
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { is_negative, value } => {
                let (gradient_type, interpolation_mode) = if let Some(index) = value.find('/') {
                    let (before, after) = value.split_at(index);
                    (before, &after[1..])
                } else {
                    // The interpolation mode defaults to `oklab`
                    (*value, "oklab")
                });

                let interpolation_mode = match interpolation_mode {
                    "longer" | "shorter" | "increasing" | "decreasing" => {
                        Cow::Owned(format!("oklch {interpolation_mode} hue"))
                    }
                    _ => Cow::Borrowed(interpolation_mode),
                });

                match gradient_type {
                    "none" => context.buffer.line("background-image: none;"),
                    "to-t" => context
                        .buffer
                        .line(format_args!("background-image: linear-gradient(to top in {interpolation_mode}, var(--en-gradient-stops));")),
                    "to-tr" => context.buffer.line(
                        format_args!("background-image: linear-gradient(to top right in {interpolation_mode}, var(--en-gradient-stops));"),
                    ),
                    "to-r" => context
                        .buffer
                        .line(format_args!("background-image: linear-gradient(to right in {interpolation_mode}, var(--en-gradient-stops));")),
                    "to-br" => context.buffer.line(
                        format_args!("background-image: linear-gradient(to bottom right in {interpolation_mode}, var(--en-gradient-stops));"),
                    ),
                    "to-b" => context.buffer.line(
                        format_args!("background-image: linear-gradient(to bottom in {interpolation_mode}, var(--en-gradient-stops));"),
                    ),
                    "to-bl" => context.buffer.line(
                        format_args!("background-image: linear-gradient(to bottom left in {interpolation_mode}, var(--en-gradient-stops));"),
                    ),
                    "to-l" => context
                        .buffer
                        .line(format_args!("background-image: linear-gradient(to left in {interpolation_mode}, var(--en-gradient-stops));")),
                    "to-tl" => context.buffer.line(
                        format_args!("background-image: linear-gradient(to top left in {interpolation_mode}, var(--en-gradient-stops));"),
                    ),
                    _ => {
                        let angle = value.parse::<usize>().unwrap();
                        context.buffer.line(
                            format_args!("background-image: linear-gradient({}{angle}deg in {interpolation_mode}, var(--en-gradient-stops));", if *is_negative { "-" } else { "" }),
                        );
                    },
                }
            }
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("background-image: linear-gradient({value});"));
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct PluginRadialDefinition;

impl Plugin for PluginRadialDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                let (gradient_type, interpolation_mode) = if let Some(index) = value.find('/') {
                    let (before, after) = value.split_at(index);
                    (before, &after[1..])
                } else {
                    (*value, "oklab")
                });

                gradient_type.is_empty() && INTERPOLATION_MODES.contains(&interpolation_mode)
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                let (_, interpolation_mode) = if let Some(index) = value.find('/') {
                    let (before, after) = value.split_at(index);
                    (before, &after[1..])
                } else {
                    // The interpolation mode defaults to `oklab`
                    (*value, "oklab")
                });

                let interpolation_mode = match interpolation_mode {
                    "longer" | "shorter" | "increasing" | "decreasing" => {
                        Cow::Owned(format!("oklch {interpolation_mode} hue"))
                    }
                    _ => Cow::Borrowed(interpolation_mode),
                });

                context
                    .buffer
                    .line(format_args!("background-image: radial-gradient(in {interpolation_mode}, var(--en-gradient-stops));"));
            }
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("background-image: radial-gradient({value});"));
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct PluginConicDefinition;

impl Plugin for PluginConicDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                let (gradient_type, interpolation_mode) = if let Some(index) = value.find('/') {
                    let (before, after) = value.split_at(index);
                    (before, &after[1..])
                } else {
                    (*value, "oklab")
                });

                (gradient_type.is_empty() || gradient_type.parse::<usize>().is_ok())
                    && INTERPOLATION_MODES.contains(&interpolation_mode)
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { is_negative, value } => {
                let (gradient_type, interpolation_mode) = if let Some(index) = value.find('/') {
                    let (before, after) = value.split_at(index);
                    (before, &after[1..])
                } else {
                    // The interpolation mode defaults to `oklab`
                    (*value, "oklab")
                });

                let interpolation_mode = match interpolation_mode {
                    "longer" | "shorter" | "increasing" | "decreasing" => {
                        Cow::Owned(format!("oklch {interpolation_mode} hue"))
                    }
                    _ => Cow::Borrowed(interpolation_mode),
                });

                if gradient_type.is_empty() {
                    context
                        .buffer
                        .line(format_args!("background-image: conic-gradient(in {interpolation_mode}, var(--en-gradient-stops));"));
                } else {
                    let angle = value.parse::<usize>().unwrap();
                    context
                        .buffer
                        .line(format_args!("background-image: conic-gradient(from {}{angle}deg in {interpolation_mode}, var(--en-gradient-stops));", if *is_negative { "-" } else { "" }));
                }
            }
            Modifier::Arbitrary { value, .. } => {
                context
                    .buffer
                    .line(format_args!("background-image: conic-gradient({value});"));
            }
        }
    }
}
