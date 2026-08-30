use std::ops::Range;

use crate::{
    Config,
    error::{ParseError, ParseErrorKind},
    generator::ContextCanHandle,
    plugins::{
        Arbitrary, Color, CustomPlugin, DynamicPropertyName, Functional, ListProperties,
        ListValues, Number, Plugin, ArbitraryDisambiguateSeparation, PropertyName, Spacing,
        StaticPropertyName,
    },
    selector::{
        CssType, Modifier, Selector, Variant,
        parser::{ARBITRARY_END, ARBITRARY_START, LAYER_BUILTIN, LAYER_CUSTOM, to_css_value},
        trie::{Trie, TrieData},
    },
    utils::{color, spacing, value_matchers::*},
};

fn is_template_matching_prop_type(
    template: &StaticPropertyName,
    prop: &StaticPropertyName,
) -> bool {
    match (template, prop) {
        (PropertyName::SingleProp(_), PropertyName::SingleProp(_)) => true,
        (PropertyName::MultipleProps(p1), PropertyName::MultipleProps(p2))
            if p1.len() == p2.len() =>
        {
            true
        }
        _ => false,
    }
}

fn dynamic_is_template_matching_prop_type(
    template: &DynamicPropertyName,
    prop: &DynamicPropertyName,
) -> bool {
    match (template, prop) {
        (PropertyName::SingleProp(_), PropertyName::SingleProp(_)) => true,
        (PropertyName::MultipleProps(p1), PropertyName::MultipleProps(p2))
            if p1.len() == p2.len() =>
        {
            true
        }
        _ => false,
    }
}

fn is_arbitrary_matching(
    matchers: &[CssType],
    matcher_separation: ArbitraryDisambiguateSeparation,
    value: &str,
) -> bool {
    let values: Vec<&str> = match matcher_separation {
        ArbitraryDisambiguateSeparation::None => std::iter::once(value).collect(),
        ArbitraryDisambiguateSeparation::Comma => value.split(',').collect(),
        ArbitraryDisambiguateSeparation::Space => value.split(' ').collect(),
        ArbitraryDisambiguateSeparation::Both => {
            value.split(',').flat_map(|s| s.split(' ')).collect()
        }
    };

    values.iter().all(|value| {
        matchers.iter().any(|matcher| {
            let value = value.trim();
            if value.is_empty() {
                return false;
            }
            match matcher {
                CssType::Shadow => is_matching_shadow(value),
                CssType::AbsoluteSize => is_matching_absolute_size(value),
                CssType::RelativeSize => is_matching_relative_size(value),
                CssType::Url => is_matching_url(value),
                CssType::LineWidth => is_matching_line_width(value),
                CssType::LineStyle => is_matching_line_style(value),
                CssType::Color => is_matching_color(value),
                CssType::Length => is_matching_length(value),
                CssType::Number => is_matching_number(value),
                CssType::Percentage => is_matching_percentage(value),
                CssType::Time => is_matching_time(value),
                CssType::Position => is_matching_position(value),
                CssType::Angle => is_matching_angle(value),
                CssType::Image => is_matching_image(value),
                CssType::FontFamilyName => is_matching_font_family_name(value),
            }
        })
    })
}

fn dynamic_is_arbitrary_matching(
    matchers: &[CssType],
    matcher_separation: ArbitraryDisambiguateSeparation,
    value: &str,
) -> bool {
    let values: Vec<&str> = match matcher_separation {
        ArbitraryDisambiguateSeparation::None => std::iter::once(value).collect(),
        ArbitraryDisambiguateSeparation::Comma => value.split(',').collect(),
        ArbitraryDisambiguateSeparation::Space => value.split(' ').collect(),
        ArbitraryDisambiguateSeparation::Both => {
            value.split(',').flat_map(|s| s.split(' ')).collect()
        }
    };

    values.iter().all(|value| {
        matchers.iter().any(|matcher| match matcher {
            CssType::Shadow => is_matching_shadow(value),
            CssType::AbsoluteSize => is_matching_absolute_size(value),
            CssType::RelativeSize => is_matching_relative_size(value),
            CssType::Url => is_matching_url(value),
            CssType::LineWidth => is_matching_line_width(value),
            CssType::LineStyle => is_matching_line_style(value),
            CssType::Color => is_matching_color(value),
            CssType::Length => is_matching_length(value),
            CssType::Number => is_matching_number(value),
            CssType::Percentage => is_matching_percentage(value),
            CssType::Time => is_matching_time(value),
            CssType::Position => is_matching_position(value),
            CssType::Angle => is_matching_angle(value),
            CssType::Image => is_matching_image(value),
            CssType::FontFamilyName => is_matching_font_family_name(value),
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

        let dynamic_modifier = {
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

        if can_handle(plugin, config, &dynamic_modifier) {
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
                        modifier: dynamic_modifier.clone(),
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
                (*value, extra_slash.as_ref().map(|e| e.default))
            };
            extra_slash
                .as_ref()
                .is_none_or(|extra_slash| extra_slash.values.contains_key(template_value.unwrap()))
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
                (*value, extra_slash.as_ref().map(|e| e.default.as_str()))
            };
            extra_slash
                .as_ref()
                .is_none_or(|extra_slash| extra_slash.values.contains_key(template_value.unwrap()))
                && values.contains_key(value)
        }

        (
            CustomPlugin::Static(Plugin::Spacing(Spacing {
                prop,
                has_auto,
                has_full,
                template,
                ..
            })),
            Modifier::Builtin { value, .. },
        ) => {
            (spacing::is_matching_builtin_spacing(value)
                || (has_auto.unwrap_or(false) && *value == "auto")
                || (has_full.unwrap_or(false) && *value == "full"))
                && template.is_none_or(|t| is_template_matching_prop_type(&t, prop))
        }
        (
            CustomPlugin::Dynamic(Plugin::Spacing(Spacing {
                prop,
                has_auto,
                has_full,
                template,
                ..
            })),
            Modifier::Builtin { value, .. },
        ) => {
            (spacing::is_matching_builtin_spacing(value)
                || (has_auto.unwrap_or(false) && *value == "auto")
                || (has_full.unwrap_or(false) && *value == "full"))
                && template
                    .as_ref()
                    .is_none_or(|t| dynamic_is_template_matching_prop_type(&t, prop))
        }

        (
            CustomPlugin::Static(Plugin::Color(Color { prop, template, .. })),
            Modifier::Builtin { value, .. },
        ) => {
            color::is_matching_builtin_color(config, value)
                && template.is_none_or(|t| is_template_matching_prop_type(&t, prop))
        }
        (
            CustomPlugin::Dynamic(Plugin::Color(Color { prop, template, .. })),
            Modifier::Builtin { value, .. },
        ) => {
            color::is_matching_builtin_color(config, value)
                && template
                    .as_ref()
                    .is_none_or(|t| dynamic_is_template_matching_prop_type(&t, prop))
        }

        (
            CustomPlugin::Static(Plugin::Number(Number {
                prop,
                has_auto,
                has_empty,
                has_negative,
                extra_slash,
                template,
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
                (*value, extra_slash.as_ref().map(|e| e.default))
            };
            extra_slash
                .as_ref()
                .is_none_or(|extra_slash| extra_slash.values.contains_key(template_value.unwrap()))
                && ((has_empty.unwrap_or(false) && value.is_empty())
                    || (has_auto.unwrap_or(false) && value == "auto")
                    || (value.parse::<usize>().is_ok()
                        && (has_negative.unwrap_or(false) || !*is_negative)))
                && template.is_none_or(|t| is_template_matching_prop_type(&t, prop))
        }
        (
            CustomPlugin::Dynamic(Plugin::Number(Number {
                prop,
                has_auto,
                has_empty,
                has_negative,
                extra_slash,
                template,
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
                (*value, extra_slash.as_ref().map(|e| e.default.as_str()))
            };
            extra_slash
                .as_ref()
                .is_none_or(|extra_slash| extra_slash.values.contains_key(template_value.unwrap()))
                && ((has_empty.unwrap_or(false) && value.is_empty())
                    || (has_auto.unwrap_or(false) && value == "auto")
                    || (value.parse::<usize>().is_ok()
                        && (has_negative.unwrap_or(false) || !*is_negative)))
                && template
                    .as_ref()
                    .is_none_or(|t| dynamic_is_template_matching_prop_type(&t, prop))
        }

        (
            CustomPlugin::Static(Plugin::Arbitrary(Arbitrary { disambiguate, .. })),
            Modifier::Arbitrary { hint, value },
        ) => disambiguate.as_ref().is_none_or(|disambiguate| {
            hint.is_some_and(|h| disambiguate.matched.contains(&h))
                || (hint.is_none()
                    && is_arbitrary_matching(
                        &disambiguate.matched,
                        disambiguate.separation,
                        value,
                    ))
        }),
        (
            CustomPlugin::Dynamic(Plugin::Arbitrary(Arbitrary { disambiguate, .. })),
            Modifier::Arbitrary { hint, value },
        ) => disambiguate.as_ref().is_none_or(|disambiguate| {
            hint.is_some_and(|h| disambiguate.matched.contains(&h))
                || (hint.is_none()
                    && dynamic_is_arbitrary_matching(
                        &disambiguate.matched,
                        disambiguate.separation,
                        value,
                    ))
        }),

        (
            CustomPlugin::Static(Plugin::Functional(Functional { can_handle, .. })),
            Modifier::Builtin { .. },
        ) => can_handle(&ContextCanHandle { config, modifier }),
        _ => false,
    }
}
