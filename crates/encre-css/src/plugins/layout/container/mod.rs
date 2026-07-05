#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::{
    config::BUILTIN_SCREENS,
    generator::{ContextCanHandle, ContextHandle, generate_at_rules, generate_class},
    prelude::build_plugin::*,
};

use std::{borrow::Cow, cmp::Ordering};

fn handle(context: &mut ContextHandle) {
    if let Modifier::Builtin { .. } = context.modifier {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |context| {
                    context.buffer.line("width: 100%;");
                },
                "",
            );
        });

        context.buffer.raw("\n\n");

        let mut first_child = true;

        generate_at_rules(context, |context| {
            let mut screens = context
                .config
                .theme
                .screens
                .iter()
                .map(|(a, b)| (a.clone(), b.clone()))
                .chain(
                    BUILTIN_SCREENS
                        .iter()
                        .map(|(a, b)| (Cow::from(*a), Cow::from(*b))),
                )
                .collect::<Vec<(Cow<str>, Cow<str>)>>();

            // Deduplicate screens
            screens.sort_by(|a, b| a.0.cmp(&b.0));
            screens.dedup_by(|a, b| a.0.eq(&b.0));

            // Emulate Tailwind sorting (based on the JS `parseInt` function)
            screens.sort_by(|a, b| {
                let a = if let Some(first_char_a) = a.1.chars().position(char::is_alphabetic) {
                    a.1[..first_char_a].parse::<usize>().ok()
                } else {
                    a.1.parse::<usize>().ok()
                };

                let b = if let Some(first_char_b) = b.1.chars().position(char::is_alphabetic) {
                    b.1[..first_char_b].parse::<usize>().ok()
                } else {
                    b.1.parse::<usize>().ok()
                };

                if let Some(a) = a {
                    if let Some(b) = b {
                        a.cmp(&b)
                    } else {
                        Ordering::Less
                    }
                } else {
                    Ordering::Greater
                }
            });

            for (_, screen) in &screens {
                if first_child {
                    first_child = false;
                } else if context.buffer.is_unindented() {
                    context.buffer.raw("\n\n");
                } else {
                    context.buffer.raw("\n");
                }

                context
                    .buffer
                    .line(format_args!("@media (width >= {screen}) {{"));
                context.buffer.indent();

                generate_class(
                    context,
                    |context| {
                        context.buffer.line(format_args!("max-width: {screen};"));
                    },
                    "",
                );

                context.buffer.unindent();
                if context.buffer.is_unindented() {
                    context.buffer.raw("}");
                } else {
                    context.buffer.line("}");
                }
            }

            // After rule
            while !context.buffer.is_unindented() {
                context.buffer.unindent();

                if context.buffer.is_unindented() {
                    context.buffer.raw("}");
                } else {
                    context.buffer.line("}");
                }
            }
        });
    }
}

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Functional { class: "container", handle });
