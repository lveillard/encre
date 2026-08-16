use std::ops::Range;

use crate::{
    Config, error::{ParseError, ParseErrorKind}, generator::ContextCanHandle, plugins::{
        Arbitrary, Color, CustomPlugin, DynamicPluginArbitraryMatcher, Functional, ListProperties, ListValues, Number, Plugin, PluginArbitraryMatcher, PluginArbitraryMatcherSeparation, Spacing, StaticPluginArbitraryMatcher,
    }, selector::{
        Modifier, Selector, Variant,
        parser::{ARBITRARY_END, ARBITRARY_START, LAYER_BUILTIN, LAYER_CUSTOM, to_css_value},
        trie::{Trie, TrieData},
    }, utils::{color, spacing, value_matchers::*},
};

fn is_arbitrary_matching(
    (matchers, modifier): &(
        &[StaticPluginArbitraryMatcher],
        PluginArbitraryMatcherSeparation,
    ),
    value: &str,
) -> bool {
    let values: Vec<&str> = match modifier {
        PluginArbitraryMatcherSeparation::None => std::iter::once(value).collect(),
        PluginArbitraryMatcherSeparation::Comma => value.split(',').collect(),
        PluginArbitraryMatcherSeparation::Space => value.split(' ').collect(),
        PluginArbitraryMatcherSeparation::Both => {
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
        Vec<DynamicPluginArbitraryMatcher>,
        PluginArbitraryMatcherSeparation,
    ),
    value: &str,
) -> bool {
    let values: Vec<&str> = match modifier {
        PluginArbitraryMatcherSeparation::None => std::iter::once(value).collect(),
        PluginArbitraryMatcherSeparation::Comma => value.split(',').collect(),
        PluginArbitraryMatcherSeparation::Space => value.split(' ').collect(),
        PluginArbitraryMatcherSeparation::Both => {
            value.split(',').flat_map(|s| s.split(' ')).collect()
        }
    };

    values.iter().all(|value| {
        matchers.iter().any(|matcher| {
            (match matcher {
                DynamicPluginArbitraryMatcher::Shadow => is_matching_shadow(value),
                DynamicPluginArbitraryMatcher::AbsoluteSize => is_matching_absolute_size(value),
                DynamicPluginArbitraryMatcher::RelativeSize => is_matching_relative_size(value),
                DynamicPluginArbitraryMatcher::LineWidth => is_matching_line_width(value),
                DynamicPluginArbitraryMatcher::LineStyle => is_matching_line_style(value),
                DynamicPluginArbitraryMatcher::Color => is_matching_color(value),
                DynamicPluginArbitraryMatcher::Length => is_matching_length(value),
                DynamicPluginArbitraryMatcher::Number => is_matching_number(value),
                DynamicPluginArbitraryMatcher::Percentage => is_matching_percentage(value),
                DynamicPluginArbitraryMatcher::Time => is_matching_time(value),
                DynamicPluginArbitraryMatcher::Gradient => is_matching_gradient(value),
                DynamicPluginArbitraryMatcher::Position => is_matching_position(value),
                DynamicPluginArbitraryMatcher::Angle => is_matching_angle(value),
                DynamicPluginArbitraryMatcher::Image => is_matching_image(value),
                DynamicPluginArbitraryMatcher::FontFamilyName => {
                    is_matching_font_family_name(value)
                }
                DynamicPluginArbitraryMatcher::Custom(v) => value == v,
                DynamicPluginArbitraryMatcher::CustomMultiple(values) => {
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

                if let CustomPlugin::Static(Plugin::Arbitrary(_))
                | CustomPlugin::Dynamic(Plugin::Arbitrary(_)) = plugin
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
            CustomPlugin::Static(Plugin::ListProperties(ListProperties { props, .. })),
            Modifier::Builtin { value, .. },
        ) => props.contains_key(value),
        (
            CustomPlugin::Dynamic(Plugin::ListProperties(ListProperties { props, .. })),
            Modifier::Builtin { value, .. },
        ) => props.contains_key(*value),

        (
            CustomPlugin::Static(Plugin::ListValues(ListValues {
                values,
                extra_slash,
                ..
            })),
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
            CustomPlugin::Dynamic(Plugin::ListValues(ListValues {
                values,
                extra_slash,
                ..
            })),
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
            CustomPlugin::Static(Plugin::Spacing(Spacing {
                has_auto, has_full, ..
            }))
            | CustomPlugin::Dynamic(Plugin::Spacing(Spacing {
                has_auto, has_full, ..
            })),
            Modifier::Builtin { value, .. },
        ) => {
            spacing::is_matching_builtin_spacing(value)
                || (has_auto.unwrap_or(false) && *value == "auto")
                || (has_full.unwrap_or(false) && *value == "full")
        }

        (
            CustomPlugin::Static(Plugin::Color(Color { .. }))
            | CustomPlugin::Dynamic(Plugin::Color(Color { .. })),
            Modifier::Builtin { value, .. },
        ) => color::is_matching_builtin_color(config, value),

        (
            CustomPlugin::Static(Plugin::Number(Number {
                has_auto,
                has_empty,
                has_negative,
                extra_slash,
                ..
            })),
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
                && ((has_empty.unwrap_or(false) && value.is_empty())
                    || (has_auto.unwrap_or(false) && value == "auto")
                    || (value.parse::<usize>().is_ok() && (has_negative.unwrap_or(false) || !*is_negative)))
        }
        (
            CustomPlugin::Dynamic(Plugin::Number(Number {
                has_auto,
                has_empty,
                has_negative,
                extra_slash,
                ..
            })),
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
                && ((has_empty.unwrap_or(false) && value.is_empty())
                    || (has_auto.unwrap_or(false) && value == "auto")
                    || (value.parse::<usize>().is_ok() && (has_negative.unwrap_or(false) || !*is_negative)))
        }

        (
            CustomPlugin::Static(Plugin::Arbitrary(Arbitrary {
                hints, matchers, ..
            })),
            Modifier::Arbitrary { hint, value },
        ) => {
            hint.is_some_and(|h| hints.is_some_and(|hints| hints.contains(&h)))
                || (hint.is_none()
                    && matchers.is_none_or(|matchers| is_arbitrary_matching(&matchers, value)))
        }
        (
            CustomPlugin::Dynamic(Plugin::Arbitrary(Arbitrary {
                hints, matchers, ..
            })),
            Modifier::Arbitrary { hint, value },
        ) => {
            hint.is_some_and(|h| hints.as_ref().is_some_and(|hints| hints.contains(&h)))
                || (hint.is_none()
                    && matchers
                        .as_ref()
                        .is_none_or(|matchers| is_parsed_arbitrary_matching(matchers, value)))
        }

        (CustomPlugin::Static(Plugin::Functional(Functional { can_handle, .. })), Modifier::Builtin { .. }) => {
            can_handle(&ContextCanHandle { config, modifier })
        }
        _ => false,
    }
}
