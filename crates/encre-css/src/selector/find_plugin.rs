use std::ops::Range;

use crate::{
    Config,
    error::{ParseError, ParseErrorKind},
    plugins::{Plugin, PluginArbitraryMatcher, PluginKind},
    selector::{
        Modifier, Selector, Variant,
        parser::{ARBITRARY_END, ARBITRARY_START, LAYER_BUILTIN, LAYER_CUSTOM, to_css_value},
        trie::{Trie, TrieData},
    },
    utils::{color, spacing, value_matchers::*},
};

// TODO: avoid blowing up the stack
fn is_arbitrary_matching(matcher: &PluginArbitraryMatcher, value: &str) -> bool {
    match matcher {
        PluginArbitraryMatcher::All => true,
        PluginArbitraryMatcher::Url => is_matching_url(value),
        PluginArbitraryMatcher::Var => is_matching_var(value),
        PluginArbitraryMatcher::Shadow => is_matching_shadow(value),
        PluginArbitraryMatcher::AbsoluteSize => is_matching_absolute_size(value),
        PluginArbitraryMatcher::RelativeSize => is_matching_relative_size(value),
        PluginArbitraryMatcher::LineWidth => is_matching_line_width(value),
        PluginArbitraryMatcher::LineStyle => is_matching_line_style(value),
        PluginArbitraryMatcher::ComputationalCssFunction => {
            is_matching_computational_css_function(value)
        }
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
        PluginArbitraryMatcher::Or(matcher1, matcher2) => {
            is_arbitrary_matching(matcher1, value) || is_arbitrary_matching(matcher2, value)
        }
        PluginArbitraryMatcher::OrMultiple(matchers) => matchers
            .iter()
            .map(|m| is_arbitrary_matching(m, value))
            .any(|x| x),
        PluginArbitraryMatcher::CommaSeparated(matcher1) => value
            .split(',')
            .all(|v| is_arbitrary_matching(matcher1, v.trim())),
        PluginArbitraryMatcher::SpaceSeparated(matcher1) => value
            .split(' ')
            .all(|v| is_arbitrary_matching(matcher1, v.trim())),
    }
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
        has_prefix,
        is_custom,
        order,
        plugin,
    } in &plugins
    {
        let layer = forced_layer.unwrap_or(parent_forced_layer.unwrap_or_else(|| {
            if *is_custom {
                LAYER_CUSTOM
            } else {
                LAYER_BUILTIN
            }
        }));

        // Find the modifier
        let mut modifier = if *has_prefix {
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

                if let PluginKind::Arbitrary { .. } = plugin.kind {
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
                        plugin: plugin,
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

fn can_handle(plugin: &Plugin, config: &Config, modifier: &Modifier) -> bool {
    match (&plugin.kind, modifier) {
        (&PluginKind::ListCases { ref cases }, &Modifier::Builtin { value, .. }) => {
            cases.contains_key(value)
        }
        (&PluginKind::ListValues { ref values, .. }, &Modifier::Builtin { value, .. }) => {
            let (value, template_value) = if plugin.extra_slash.is_some()
                && let Some(index) = value.find('/')
            {
                let (before, after) = value.split_at(index);
                (before, Some(&after[1..]))
            } else {
                (value, plugin.extra_slash.as_ref().map(|e| e.1))
            };
            plugin
                .extra_slash
                .as_ref()
                .is_none_or(|extra_slash| extra_slash.0.contains_key(template_value.unwrap()))
                && values.contains_key(value)
        }
        (PluginKind::Spacing { .. }, Modifier::Builtin { value, .. }) => {
            spacing::is_matching_builtin_spacing(value)
                || (plugin.has_auto && *value == "auto")
                || (plugin.has_full && *value == "full")
        }
        (PluginKind::Color { .. }, Modifier::Builtin { value, .. }) => {
            color::is_matching_builtin_color(config, value)
        }
        (
            PluginKind::Number { .. },
            Modifier::Builtin {
                value, is_negative, ..
            },
        ) => {
            let (value, template_value) = if plugin.extra_slash.is_some()
                && let Some(index) = value.find('/')
            {
                let (before, after) = value.split_at(index);
                (before, Some(&after[1..]))
            } else {
                (*value, plugin.extra_slash.as_ref().map(|e| e.1))
            };
            plugin
                .extra_slash
                .as_ref()
                .is_none_or(|extra_slash| extra_slash.0.contains_key(template_value.unwrap()))
                && ((plugin.has_empty && value.is_empty())
                    || (value.parse::<usize>().is_ok() && (plugin.has_negative || !*is_negative)))
        }
        (PluginKind::Arbitrary { .. }, Modifier::Arbitrary { hint, value }) => {
            hint.is_some_and(|h| {
                plugin
                    .arbitrary_hints
                    .is_some_and(|hints| hints.contains(&h))
            }) || (hint.is_none()
                && plugin
                    .arbitrary_matcher
                    .is_none_or(|matcher| is_arbitrary_matching(&matcher, value)))
        }
        (PluginKind::Functional { class, .. }, Modifier::Builtin { value, .. }) => value == class,
        _ => false,
    }
}
