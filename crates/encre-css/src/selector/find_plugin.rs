use std::ops::Range;

use crate::{
    Config,
    error::{ParseError, ParseErrorKind},
    generator::ContextCanHandle,
    plugins::{
        CustomPlugin, Plugin, PluginArbitraryMatcher, PluginArbitraryMatcherModifier, PluginKind,
        parsed::{ParsedPlugin, ParsedPluginArbitraryMatcher, ParsedPluginKind},
    },
    selector::{
        Modifier, Selector, Variant,
        parser::{ARBITRARY_END, ARBITRARY_START, LAYER_BUILTIN, LAYER_CUSTOM, to_css_value},
        trie::{Trie, TrieData},
    },
    utils::{color, spacing, value_matchers::*},
};

fn is_arbitrary_matching(
    (matchers, modifier): &(&[PluginArbitraryMatcher], PluginArbitraryMatcherModifier),
    value: &str,
) -> bool {
    let values: Vec<&str> = match modifier {
        PluginArbitraryMatcherModifier::None => std::iter::once(value).collect(),
        PluginArbitraryMatcherModifier::CommaSeparated => value.split(',').collect(),
        PluginArbitraryMatcherModifier::SpaceSeparated => value.split(' ').collect(),
        PluginArbitraryMatcherModifier::Both => {
            value.split(',').flat_map(|s| s.split(' ')).collect()
        }
    };

    values.iter().all(|value| {
        matchers.iter().any(|matcher| {
            let value = value.trim();
            if value.is_empty() {
                return false;
            }
            (match matcher {
                PluginArbitraryMatcher::Shadow => is_matching_shadow(value),
                PluginArbitraryMatcher::AbsoluteSize => is_matching_absolute_size(value),
                PluginArbitraryMatcher::RelativeSize => is_matching_relative_size(value),
                PluginArbitraryMatcher::LineWidth => is_matching_line_width(value),
                PluginArbitraryMatcher::LineStyle => is_matching_line_style(value),
                PluginArbitraryMatcher::Color => is_matching_color(value),
                PluginArbitraryMatcher::Length => is_matching_length(value),
                PluginArbitraryMatcher::Number => is_matching_number(value),
                PluginArbitraryMatcher::Percentage => is_matching_percentage(value),
                PluginArbitraryMatcher::Time => is_matching_time(value),
                PluginArbitraryMatcher::Gradient => is_matching_gradient(value),
                PluginArbitraryMatcher::Position => is_matching_position(value),
                PluginArbitraryMatcher::Angle => is_matching_angle(value),
                PluginArbitraryMatcher::Image => is_matching_image(value),
                PluginArbitraryMatcher::FontFamilyName => is_matching_font_family_name(value),
                PluginArbitraryMatcher::Custom(v) => value == *v,
                PluginArbitraryMatcher::CustomMultiple(values) => values.contains(&value),
            }) || is_matching_var(value)
        })
    })
}

fn is_parsed_arbitrary_matching(
    (matchers, modifier): &(
        Vec<ParsedPluginArbitraryMatcher>,
        PluginArbitraryMatcherModifier,
    ),
    value: &str,
) -> bool {
    let values: Vec<&str> = match modifier {
        PluginArbitraryMatcherModifier::None => std::iter::once(value).collect(),
        PluginArbitraryMatcherModifier::CommaSeparated => value.split(',').collect(),
        PluginArbitraryMatcherModifier::SpaceSeparated => value.split(' ').collect(),
        PluginArbitraryMatcherModifier::Both => {
            value.split(',').flat_map(|s| s.split(' ')).collect()
        }
    };

    values.iter().all(|value| {
        matchers.iter().any(|matcher| {
            (match matcher {
                ParsedPluginArbitraryMatcher::Shadow => is_matching_shadow(value),
                ParsedPluginArbitraryMatcher::AbsoluteSize => is_matching_absolute_size(value),
                ParsedPluginArbitraryMatcher::RelativeSize => is_matching_relative_size(value),
                ParsedPluginArbitraryMatcher::LineWidth => is_matching_line_width(value),
                ParsedPluginArbitraryMatcher::LineStyle => is_matching_line_style(value),
                ParsedPluginArbitraryMatcher::Color => is_matching_color(value),
                ParsedPluginArbitraryMatcher::Length => is_matching_length(value),
                ParsedPluginArbitraryMatcher::Number => is_matching_number(value),
                ParsedPluginArbitraryMatcher::Percentage => is_matching_percentage(value),
                ParsedPluginArbitraryMatcher::Time => is_matching_time(value),
                ParsedPluginArbitraryMatcher::Gradient => is_matching_gradient(value),
                ParsedPluginArbitraryMatcher::Position => is_matching_position(value),
                ParsedPluginArbitraryMatcher::Angle => is_matching_angle(value),
                ParsedPluginArbitraryMatcher::Image => is_matching_image(value),
                ParsedPluginArbitraryMatcher::FontFamilyName => is_matching_font_family_name(value),
                ParsedPluginArbitraryMatcher::Custom(v) => value == v,
                ParsedPluginArbitraryMatcher::CustomMultiple(values) => {
                    values.iter().any(|v| v == value)
                }
            }) || is_matching_var(value)
        })
    })
}

pub(super) fn find_plugin_to_handle_class<'a>(
    config: &Config,
    parent_forced_layer: Option<i8>,
    forced_layer: Option<i8>,
    variants: Vec<Vec<Variant<'a>>>,
    val: &'a str,
    full_class: Option<&'a str>,
    class: &'a str,
    is_negative: bool,
    is_important: bool,
    span: Range<usize>,
    trie: &Trie,
) -> Vec<Result<Selector<'a>, ParseError<'a>>> {
    let (common_prefix, plugins) = trie.common_prefix_search(class);
    let plugins = plugins.collect::<Vec<&TrieData>>();

    for TrieData {
        has_namespace,
        is_custom,
        order,
        plugin,
    } in &plugins
    {
        let layer = forced_layer.unwrap_or(parent_forced_layer.unwrap_or({
            if *is_custom {
                LAYER_CUSTOM
            } else {
                LAYER_BUILTIN
            }
        }));

        // Find the modifier
        let mut modifier = if *has_namespace {
            class
                .strip_prefix(&*common_prefix)
                .expect("common_prefix_search returns a common prefix has tuple first value")
        } else {
            class
        };

        let parsed_modifier = {
            if modifier.is_empty() {
                Modifier::Builtin {
                    is_negative: false,
                    value: "",
                }
            } else {
                if modifier.starts_with('-') {
                    modifier = &modifier[1..];
                }

                if let CustomPlugin::Static(Plugin {
                    kind: PluginKind::Arbitrary { .. },
                    ..
                })
                | CustomPlugin::Parsed(ParsedPlugin {
                    kind: ParsedPluginKind::Arbitrary { .. },
                    ..
                }) = plugin
                {
                    if let Some(value) = modifier.strip_prefix(ARBITRARY_START)
                        && let Some(value) = value.strip_suffix(ARBITRARY_END)
                    {
                        let (hint, value) = if let Some((maybe_hint, rest)) = value.split_once(':')
                            && let Ok(hint) = maybe_hint.parse()
                        {
                            (Some(hint), to_css_value(rest))
                        } else {
                            (None, to_css_value(value))
                        };

                        Modifier::Arbitrary { hint, value }
                    } else {
                        continue;
                    }
                } else {
                    Modifier::Builtin {
                        is_negative,
                        value: modifier,
                    }
                }
            }
        };

        if can_handle(plugin, config, &parsed_modifier) {
            return variants
                .into_iter()
                .map(|variants| {
                    Ok(Selector {
                        layer,
                        order: *order,
                        full: if let Some(full_class) = full_class {
                            full_class
                        } else {
                            val
                        },
                        modifier: parsed_modifier.clone(),
                        variants,
                        is_important,
                        plugin: plugin.clone(),
                    })
                })
                .collect();
        }
    }

    vec![Err(ParseError::new(
        span,
        ParseErrorKind::UnknownPlugin(val),
    ))]
}

fn can_handle(plugin: &CustomPlugin, config: &Config, modifier: &Modifier) -> bool {
    match (&plugin, modifier) {
        (
            CustomPlugin::Static(Plugin {
                kind: PluginKind::ListCases { cases },
                ..
            }),
            Modifier::Builtin { value, .. },
        ) => cases.contains_key(value),
        (
            CustomPlugin::Parsed(ParsedPlugin {
                kind: ParsedPluginKind::ListCases { cases },
                ..
            }),
            Modifier::Builtin { value, .. },
        ) => cases.contains_key(*value),

        (
            CustomPlugin::Static(Plugin {
                kind: PluginKind::ListValues { values, .. },
                extra_slash,
                ..
            }),
            Modifier::Builtin { value, .. },
        ) => {
            let (value, template_value) = if extra_slash.is_some()
                && let Some(index) = value.find('/')
            {
                let (before, after) = value.split_at(index);
                (before, Some(&after[1..]))
            } else {
                (*value, extra_slash.as_ref().map(|e| e.1))
            };
            extra_slash
                .as_ref()
                .is_none_or(|extra_slash| extra_slash.0.contains_key(template_value.unwrap()))
                && values.contains_key(value)
        }

        (
            CustomPlugin::Parsed(ParsedPlugin {
                kind: ParsedPluginKind::ListValues { values, .. },
                extra_slash,
                ..
            }),
            Modifier::Builtin { value, .. },
        ) => {
            let (value, template_value) = if extra_slash.is_some()
                && let Some(index) = value.find('/')
            {
                let (before, after) = value.split_at(index);
                (before, Some(&after[1..]))
            } else {
                (*value, extra_slash.as_ref().map(|e| e.1.as_str()))
            };
            extra_slash
                .as_ref()
                .is_none_or(|extra_slash| extra_slash.0.contains_key(template_value.unwrap()))
                && values.contains_key(value)
        }

        (
            CustomPlugin::Static(Plugin {
                kind: PluginKind::Spacing { .. },
                has_auto,
                has_full,
                ..
            })
            | CustomPlugin::Parsed(ParsedPlugin {
                kind: ParsedPluginKind::Spacing { .. },
                has_auto,
                has_full,
                ..
            }),
            Modifier::Builtin { value, .. },
        ) => {
            spacing::is_matching_builtin_spacing(value)
                || (*has_auto && *value == "auto")
                || (*has_full && *value == "full")
        }

        (
            CustomPlugin::Static(Plugin {
                kind: PluginKind::Color { .. },
                ..
            })
            | CustomPlugin::Parsed(ParsedPlugin {
                kind: ParsedPluginKind::Color { .. },
                ..
            }),
            Modifier::Builtin { value, .. },
        ) => color::is_matching_builtin_color(config, value),

        (
            CustomPlugin::Static(Plugin {
                kind: PluginKind::Number { .. },
                has_auto,
                has_empty,
                has_negative,
                extra_slash,
                ..
            }),
            Modifier::Builtin {
                value, is_negative, ..
            },
        ) => {
            let (value, template_value) = if extra_slash.is_some()
                && let Some(index) = value.find('/')
            {
                let (before, after) = value.split_at(index);
                (before, Some(&after[1..]))
            } else {
                (*value, extra_slash.as_ref().map(|e| e.1))
            };
            extra_slash
                .as_ref()
                .is_none_or(|extra_slash| extra_slash.0.contains_key(template_value.unwrap()))
                && ((*has_empty && value.is_empty())
                    || (*has_auto && value == "auto")
                    || (value.parse::<usize>().is_ok() && (*has_negative || !*is_negative)))
        }
        (
            CustomPlugin::Parsed(ParsedPlugin {
                kind: ParsedPluginKind::Number { .. },
                has_auto,
                has_empty,
                has_negative,
                extra_slash,
                ..
            }),
            Modifier::Builtin {
                value, is_negative, ..
            },
        ) => {
            let (value, template_value) = if extra_slash.is_some()
                && let Some(index) = value.find('/')
            {
                let (before, after) = value.split_at(index);
                (before, Some(&after[1..]))
            } else {
                (*value, extra_slash.as_ref().map(|e| e.1.as_str()))
            };
            extra_slash
                .as_ref()
                .is_none_or(|extra_slash| extra_slash.0.contains_key(template_value.unwrap()))
                && ((*has_empty && value.is_empty())
                    || (*has_auto && value == "auto")
                    || (value.parse::<usize>().is_ok() && (*has_negative || !*is_negative)))
        }

        (
            CustomPlugin::Static(Plugin {
                kind: PluginKind::Arbitrary { .. },
                arbitrary_hints,
                arbitrary_matchers,
                ..
            }),
            Modifier::Arbitrary { hint, value },
        ) => {
            hint.is_some_and(|h| arbitrary_hints.is_some_and(|hints| hints.contains(&h)))
                || (hint.is_none()
                    && arbitrary_matchers
                        .is_none_or(|matchers| is_arbitrary_matching(&matchers, value)))
        }
        (
            CustomPlugin::Parsed(ParsedPlugin {
                kind: ParsedPluginKind::Arbitrary { .. },
                arbitrary_hints,
                arbitrary_matchers,
                ..
            }),
            Modifier::Arbitrary { hint, value },
        ) => {
            hint.is_some_and(|h| {
                arbitrary_hints
                    .as_ref()
                    .is_some_and(|hints| hints.contains(&h))
            }) || (hint.is_none()
                && arbitrary_matchers
                    .as_ref()
                    .is_none_or(|matchers| is_parsed_arbitrary_matching(&matchers, value)))
        }

        (
            CustomPlugin::Static(Plugin {
                kind: PluginKind::Functional { can_handle, .. },
                ..
            }),
            Modifier::Builtin { .. },
        ) => can_handle(&ContextCanHandle { config, modifier }),
        _ => false,
    }
}
