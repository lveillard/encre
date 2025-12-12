use super::{Modifier, Selector, Variant};
use crate::{
    config::{Config, BUILTIN_PLUGINS, BUILTIN_VARIANTS},
    error::{ParseError, ParseErrorKind},
    generator::ContextCanHandle,
    plugins::css_property::CssPropertyPlugin,
    utils::split_ignore_arbitrary,
};

use std::{borrow::Cow, ops::Range};

pub(crate) const ARBITRARY_START: char = '[';
pub(crate) const ARBITRARY_END: char = ']';
pub(crate) const GROUP_START: char = '(';
pub(crate) const GROUP_END: char = ')';
pub(crate) const ESCAPE: char = '\\';

const VALID_PLUGIN_HINT: [&str; 13] = [
    "color",
    "length",
    "line-width",
    "image",
    "url",
    "position",
    "percentage",
    "number",
    "generic-name",
    "family-name",
    "absolute-size",
    "relative-size",
    "shadow",
];

const LAYER_CUSTOM: i8 = -1;
const LAYER_BUILTIN: i8 = 0;
const LAYER_ARBITRARY: i8 = i8::MAX;

/// Remove the first and last character of a string.
pub(crate) fn unwrap_string(val: &mut &str) {
    let len = val.len();
    *val = &val[1..len - 1];
}

/// Replace all underscores with spaces (not in `url()`).
pub(crate) fn underscores_to_spaces(val: Cow<str>) -> Cow<str> {
    // Don't replace `_` if it is a URL
    if val.contains("url(") {
        // For the `CursorPlugin`, `ContentPlugin` and `ImagePlugin` plugins, we need to keep underscores in URLs
        val.split("url(")
            .map(|p| {
                if p.is_empty() {
                    return String::new();
                }

                let result = if let Some(sub) = p.strip_prefix('\'') {
                    sub.find("')").map(|index| p.split_at(index + 2))
                } else if let Some(sub) = p.strip_prefix('"') {
                    sub.find("\")").map(|index| p.split_at(index + 2))
                } else {
                    p.find(')').map(|index| p.split_at(index + 1))
                };

                if let Some((before, after)) = result {
                    format!("url({}{}", before, after.replace('_', " "))
                } else {
                    format!("url({}", p.replace('_', " "))
                }
            })
            .collect()
    } else if val.contains('_') {
        // Replace `_` by ` ` (space)
        Cow::from(val.replace('_', " "))
    } else {
        val
    }
}

/// Replace:
/// - `&#34;` by `"`
/// - `&#39;` by `'`
/// - `&#40;` by `(`
/// - `&#41;` by `)`
/// - `&#91;` by `[`
/// - `&#92;` by `\`
/// - `&#93;` by `]`
/// - `&#95;` by `_`
/// - `&#96;` by `` ` ``
pub(crate) fn replace_escape_codes(val: Cow<str>) -> Cow<str> {
    Cow::from(
        val.replace("&#34;", "\"")
            .replace("&#39;", "'")
            .replace("&#40;", "(")
            .replace("&#41;", ")")
            .replace("&#91;", "[")
            .replace("&#92;", "\\")
            .replace("&#93;", "]")
            .replace("&#95;", "_")
            .replace("&#96;", "`"),
    )
}

/// Convert an arbitrary value into a CSS value.
///
///  -  `_` (underscores) are converted to ` ` (spaces) (not in `url`s or if using `&#95;`);
///  - Spaces are added around operators in the `calc` CSS function.
///  - Some escape codes are replaced by the characters, see [replace_escape_codes]
pub(crate) fn to_css_value(val: &str) -> Cow<'_, str> {
    let mut val = underscores_to_spaces(Cow::from(val));

    // Add spaces around operators in the `calc` CSS function
    if val.contains("calc") {
        val = Cow::from(
            val.split(' ')
                .map(|v| {
                    if v.starts_with("calc(") {
                        Cow::from(
                            v.replace('-', " - ")
                                .replace('+', " + ")
                                .replace('/', " / ")
                                .replace('*', " * "),
                        )
                    } else {
                        Cow::from(v)
                    }
                })
                .collect::<Vec<Cow<str>>>()
                .join(" "),
        );
    }

    replace_escape_codes(val)
}

pub(crate) fn parse<'a>(
    val: &'a str,
    span: Option<Range<usize>>,
    full_class: Option<&'a str>,
    config: &Config,
    config_derived_variants: &[(Cow<'static, str>, Variant<'static>)],
) -> Vec<Result<Selector<'a>, ParseError<'a>>> {
    // The shortest selector is `m1`
    if val.len() < 2 {
        return vec![Err(ParseError::new(
            span.unwrap_or(0..val.len()),
            ParseErrorKind::TooShort(val),
        ))];
    }

    parse_recursive(val, span, full_class, None, config, config_derived_variants)
}

#[allow(clippy::too_many_lines)]
fn push_variant<'a>(
    is_arbitrary: bool,
    full_variant: &'a str,
    variant_list: &mut Vec<Variant<'a>>,
    forced_variant: &mut Option<i8>,
    val: &'a str,
    span: &Range<usize>,
    config: &Config,
    config_derived_variants: &[(Cow<'static, str>, Variant<'static>)],
) -> Result<(), ParseError<'a>> {
    if is_arbitrary {
        variant_list.push(Variant {
            order: config.last_variant_order(),
            prefixed: false,
            template: replace_escape_codes(underscores_to_spaces(Cow::from(full_variant))),
        });
        return Ok(());
    } else if let Some(variant) = BUILTIN_VARIANTS.get(full_variant) {
        if matches!(
            variant,
            Variant {
                prefixed: false,
                ..
            }
        ) {
            variant_list.push(variant.clone());
            return Ok(());
        }
    } else if let Some(variant) = config
        .custom_variants
        .iter()
        .chain(config_derived_variants)
        .find(|(n, _)| n == full_variant)
    {
        variant_list.push(variant.1.clone());
        return Ok(());
    } else if let Some(group_variant) = full_variant.strip_prefix("group-") {
        let (group_variant, name) = if let Some((variant, name)) = group_variant.split_once('/') {
            (variant, Some(name.to_string()))
        } else {
            (group_variant, None)
        };

        if let Some(mut variant) = BUILTIN_VARIANTS.get(group_variant).cloned() {
            if !variant.template.starts_with('@') {
                variant.order += 1000;

                let suffix = if let Some(name) = name {
                    Cow::Owned(format!("\\/{name}"))
                } else {
                    Cow::Borrowed("")
                };

                variant.template = Cow::Owned(format!(
                    "{} &",
                    variant.template.replace('&', &format!(".group{suffix}"))
                ));
                variant_list.push(variant.clone());
                return Ok(());
            }
        } else if let Some(arbitrary_variant) = group_variant
            .strip_prefix(ARBITRARY_START)
            .and_then(|p| p.strip_suffix(ARBITRARY_END))
        {
            if !arbitrary_variant.starts_with('@') {
                let template =
                    replace_escape_codes(underscores_to_spaces(Cow::from(arbitrary_variant)));

                let suffix = if let Some(name) = name {
                    Cow::Owned(format!("\\/{name}"))
                } else {
                    Cow::Borrowed("")
                };

                let template = if template.contains('&') {
                    template.replace('&', &format!(".group{suffix}"))
                } else {
                    format!(".group{suffix}{template}")
                };

                let template = Cow::Owned(format!("{} &", template));
                let variant = Variant {
                    order: config.last_variant_order(),
                    prefixed: false,
                    template,
                };

                variant_list.push(variant.clone());
                return Ok(());
            }
        }
    } else if let Some(peer_not_variant) = full_variant.strip_prefix("peer-not-") {
        let (peer_not_variant, name) =
            if let Some((variant, name)) = peer_not_variant.split_once('/') {
                (variant, Some(name.to_string()))
            } else {
                (peer_not_variant, None)
            };

        if let Some(mut variant) = BUILTIN_VARIANTS.get(peer_not_variant).cloned() {
            if !variant.template.starts_with('@') {
                variant.order += 2000;

                let suffix = if let Some(name) = name {
                    Cow::Owned(format!("\\/{name}"))
                } else {
                    Cow::Borrowed("")
                };

                variant.template = Cow::Owned(format!(
                    "{}) ~ &",
                    variant
                        .template
                        .replace('&', &format!(".peer{suffix}:not("))
                ));
                variant_list.push(variant.clone());
                return Ok(());
            }
        } else if let Some(arbitrary_variant) = peer_not_variant
            .strip_prefix(ARBITRARY_START)
            .and_then(|p| p.strip_suffix(ARBITRARY_END))
        {
            if !arbitrary_variant.starts_with('@') {
                let template =
                    replace_escape_codes(underscores_to_spaces(Cow::from(arbitrary_variant)));

                let suffix = if let Some(name) = name {
                    Cow::Owned(format!("\\/{name}"))
                } else {
                    Cow::Borrowed("")
                };

                let template = if template.contains('&') {
                    template.replace('&', &format!(".peer{suffix}:not("))
                } else {
                    format!(".peer{suffix}:not({template}")
                };

                let template = Cow::Owned(format!("{}) ~ &", template));
                let variant = Variant {
                    order: config.last_variant_order(),
                    prefixed: false,
                    template,
                };

                variant_list.push(variant.clone());
                return Ok(());
            }
        }
    } else if let Some(peer_variant) = full_variant.strip_prefix("peer-") {
        let (peer_variant, name) = if let Some((variant, name)) = peer_variant.split_once('/') {
            (variant, Some(name.to_string()))
        } else {
            (peer_variant, None)
        };

        if let Some(mut variant) = BUILTIN_VARIANTS.get(peer_variant).cloned() {
            if !variant.template.starts_with('@') {
                variant.order += 3000;

                let suffix = if let Some(name) = name {
                    Cow::Owned(format!("\\/{name}"))
                } else {
                    Cow::Borrowed("")
                };

                variant.template = Cow::Owned(format!(
                    "{} ~ &",
                    variant.template.replace('&', &format!(".peer{suffix}"))
                ));
                variant_list.push(variant.clone());
                return Ok(());
            }
        } else if let Some(arbitrary_variant) = peer_variant
            .strip_prefix(ARBITRARY_START)
            .and_then(|p| p.strip_suffix(ARBITRARY_END))
        {
            if !arbitrary_variant.starts_with('@') {
                let template =
                    replace_escape_codes(underscores_to_spaces(Cow::from(arbitrary_variant)));

                let suffix = if let Some(name) = name {
                    Cow::Owned(format!("\\/{name}"))
                } else {
                    Cow::Borrowed("")
                };

                let template = if template.contains('&') {
                    template.replace('&', &format!(".peer{suffix}"))
                } else {
                    format!(".peer{suffix}{template}")
                };

                let template = Cow::Owned(format!("{} ~ &", template));
                let variant = Variant {
                    order: config.last_variant_order(),
                    prefixed: false,
                    template,
                };

                variant_list.push(variant.clone());
                return Ok(());
            }
        }
    } else if let Some((prefix, value)) = full_variant.split_once(ARBITRARY_START) {
        if let Some(prefix) = prefix.strip_suffix("-") {
            if let Some(value) = value.strip_suffix(ARBITRARY_END) {
                let value = replace_escape_codes(underscores_to_spaces(Cow::Borrowed(value)));
                let mut variant = if let Some(variant) = BUILTIN_VARIANTS.get(prefix).cloned() {
                    variant
                } else if let Some(variant) = config
                    .custom_variants
                    .iter()
                    .find(|(n, _)| n == prefix)
                    .cloned()
                {
                    variant.1
                } else {
                    return Err(ParseError::new(
                        span.clone(),
                        ParseErrorKind::UnknownVariant(full_variant, val),
                    ));
                };

                variant.template = Cow::Owned(variant.template.replace("{}", &value));
                variant_list.push(variant.clone());
                return Ok(());
            }
        }
    } else if let Some(layer_name) = full_variant.strip_prefix("l-") {
        if let Some(layer_index) = config.layers.get(layer_name) {
            *forced_variant = Some(*layer_index);
            return Ok(());
        }
    }

    Err(ParseError::new(
        span.clone(),
        ParseErrorKind::UnknownVariant(full_variant, val),
    ))
}

#[allow(clippy::too_many_lines)]
fn parse_recursive<'a>(
    val: &'a str,
    span: Option<Range<usize>>,
    full_class: Option<&'a str>,
    parent_forced_layer: Option<i8>,
    config: &Config,
    config_derived_variants: &[(Cow<'static, str>, Variant<'static>)],
) -> Vec<Result<Selector<'a>, ParseError<'a>>> {
    let span = span.unwrap_or(0..val.len());

    // Parse variants
    let (variants, forced_layer, mut remaining) = {
        let mut arbitraries = 0usize;
        let mut groups = 0usize;
        let mut last_index = 0;
        let mut variants = vec![vec![]];
        let mut forced_layer = None;

        for ch in val.char_indices() {
            if last_index > val.len() {
                continue;
            }

            match ch.1 {
                '[' => {
                    arbitraries = arbitraries.saturating_add(1);
                    continue;
                }
                ']' => {
                    arbitraries = arbitraries.saturating_sub(1);
                    continue;
                }
                '(' if arbitraries == 0 => {
                    groups = groups.saturating_add(1);
                    continue;
                }
                ')' if arbitraries == 0 => {
                    groups = groups.saturating_sub(1);
                    continue;
                }
                ':' if arbitraries == 0 && groups == 0 => (),
                _ => {
                    continue;
                }
            }

            let mut part = &val[last_index..ch.0];
            last_index = ch.0 + 1;

            if part.starts_with(GROUP_START) && part.ends_with(GROUP_END) {
                unwrap_string(&mut part);

                let mut errors: Vec<Result<Selector<'a>, ParseError<'a>>> = vec![];
                let mut new_variants = vec![];

                split_ignore_arbitrary(part, ',', true).for_each(|(_, mut sub_variant)| {
                    let (is_arbitrary, variant) = {
                        if sub_variant.starts_with(ARBITRARY_START)
                            && sub_variant.ends_with(ARBITRARY_END)
                        {
                            unwrap_string(&mut sub_variant);
                            (true, sub_variant)
                        } else {
                            (false, sub_variant)
                        }
                    };

                    for variant_list in &variants {
                        // TODO: Maybe use `Cow`s to just reference all the other variants
                        let mut new_variant_list = variant_list.clone();

                        if let Err(e) = push_variant(
                            is_arbitrary,
                            variant,
                            &mut new_variant_list,
                            &mut forced_layer,
                            val,
                            &span,
                            config,
                            config_derived_variants,
                        ) {
                            errors.push(Err(e));
                        }

                        new_variants.push(new_variant_list);
                    }
                });

                variants = new_variants;

                if !errors.is_empty() {
                    return errors;
                }
            } else {
                let (is_arbitrary, variant) = {
                    if part.starts_with(ARBITRARY_START) && part.ends_with(ARBITRARY_END) {
                        unwrap_string(&mut part);
                        (true, part)
                    } else {
                        (false, part)
                    }
                };

                let mut errors = vec![];
                for variant_list in &mut variants {
                    if let Err(e) = push_variant(
                        is_arbitrary,
                        variant,
                        variant_list,
                        &mut forced_layer,
                        val,
                        &span,
                        config,
                        config_derived_variants,
                    ) {
                        errors.push(Err(e));
                    }
                }

                if !errors.is_empty() {
                    return errors;
                }
            }
        }

        (variants, forced_layer, (last_index, &val[last_index..]))
    };

    if remaining.1.is_empty() {
        return vec![Err(ParseError::new(
            span,
            ParseErrorKind::VariantsWithoutModifier(val),
        ))];
    }

    // Parse the namespace and modifier
    if remaining.1.starts_with(GROUP_START) && remaining.1.ends_with(GROUP_END) {
        // Variant group (selectors are separated by `,`), we need to parse child selectors
        unwrap_string(&mut remaining.1);

        if remaining.1.is_empty() {
            return vec![Err(ParseError::new(
                span,
                ParseErrorKind::VariantsWithoutModifier(val),
            ))];
        }

        split_ignore_arbitrary(remaining.1, ',', true)
            .flat_map(|(i, sub_selector)| {
                #[allow(clippy::range_plus_one)]
                let selectors = parse_recursive(
                    sub_selector,
                    Some(
                        span.start + remaining.0 + i + 1
                            ..span.start + remaining.0 + i + sub_selector.len() + 1,
                    ),
                    Some(if let Some(full_class) = full_class {
                        full_class
                    } else {
                        val
                    }),
                    forced_layer,
                    config,
                    config_derived_variants,
                );

                // Merge the common variants with each child selector variant list
                selectors
                    .into_iter()
                    .fold(vec![], |mut new_selectors, selector| {
                        match selector {
                            Ok(mut selector) => {
                                let variant_len = selector.variants.len();

                                for variant_list in &variants {
                                    selector.variants.extend(variant_list.iter().cloned());
                                    new_selectors.push(Ok(selector.clone()));
                                    selector.variants.truncate(variant_len);
                                }
                            }
                            Err(e) => new_selectors.push(Err(e)),
                        }

                        new_selectors
                    })
            })
            .collect()
    } else {
        // Child selector
        let is_important = if let Some(new_remaining) = remaining.1.strip_prefix('!') {
            remaining = (remaining.0, new_remaining);
            true
        } else {
            false
        };

        let is_negative = if let Some(new_remaining) = remaining.1.strip_prefix('-') {
            remaining = (remaining.0, new_remaining);
            true
        } else {
            false
        };

        if remaining.1.starts_with(ARBITRARY_START) && remaining.1.ends_with(ARBITRARY_END) {
            // Arbitrary CSS property (without namespace)
            let plugin = &CssPropertyPlugin;

            variants
                .into_iter()
                .map(|variants| {
                    Ok(Selector {
                        // Arbitrary properties will be placed at the end of the CSS
                        layer: forced_layer
                            .unwrap_or(parent_forced_layer.unwrap_or(LAYER_ARBITRARY)),
                        order: usize::MAX,
                        full: if let Some(full_class) = full_class {
                            full_class
                        } else {
                            val
                        },
                        modifier: Modifier::Arbitrary {
                            prefix: "",
                            hint: "",
                            value: to_css_value(&remaining.1[1..remaining.1.len() - 1]),
                        },
                        variants,
                        is_important,
                        plugin,
                    })
                })
                .collect()
        } else {
            // Find the right plugin for handling this selector
            for (order, layer, (namespace, plugin)) in BUILTIN_PLUGINS
                .iter()
                .enumerate()
                .map(|p| {
                    (
                        p.0,
                        forced_layer.unwrap_or(parent_forced_layer.unwrap_or(LAYER_BUILTIN)),
                        p.1,
                    )
                })
                .chain(
                    // Selectors generated using custom plugins are placed first to be easily
                    // overridden
                    config.custom_plugins.iter().enumerate().map(|p| {
                        (
                            p.0,
                            forced_layer.unwrap_or(parent_forced_layer.unwrap_or(LAYER_CUSTOM)),
                            p.1,
                        )
                    }),
                )
            {
                // Find the modifier
                if let Some(modifier) = remaining
                    .1
                    .strip_prefix(&**namespace)
                    .and_then(|modifier| parse_modifier(modifier, is_negative))
                {
                    let context = ContextCanHandle {
                        config,
                        modifier: &modifier,
                    };

                    if let Modifier::Arbitrary { prefix, .. } = modifier {
                        if !prefix.is_empty() {
                            // If the modifier is arbitrary, the namespace must be strictly parsed
                            // to avoid accepting too much selectors, e.g `flex-test-[]` being
                            // parsed as a `flex` selector
                            continue;
                        }
                    }

                    if plugin.can_handle(context) {
                        return variants
                            .into_iter()
                            .map(|variants| {
                                Ok(Selector {
                                    layer,
                                    order,
                                    full: if let Some(full_class) = full_class {
                                        full_class
                                    } else {
                                        val
                                    },
                                    modifier: modifier.clone(),
                                    variants,
                                    is_important,
                                    plugin: *plugin,
                                })
                            })
                            .collect();
                    }
                }
            }

            vec![Err(ParseError::new(
                span,
                ParseErrorKind::UnknownPlugin(val),
            ))]
        }
    }
}

fn parse_modifier(mut modifier: &str, is_negative: bool) -> Option<Modifier<'_>> {
    if modifier.is_empty() {
        return Some(Modifier::Builtin {
            is_negative: false,
            value: "",
        });
    }

    if modifier.starts_with('-') {
        modifier = &modifier[1..];
    }

    if let Some((prefix, mut value)) = modifier.split_once(ARBITRARY_START) {
        if value.chars().last().is_some_and(|v| v == ARBITRARY_END) {
            value = &value[..value.len() - 1];
        } else {
            return None;
        }

        let (hint, value) = if let Some((maybe_hint, rest)) = value.split_once(':') {
            if VALID_PLUGIN_HINT.contains(&maybe_hint) {
                (maybe_hint, to_css_value(rest))
            } else {
                ("", to_css_value(value))
            }
        } else {
            ("", to_css_value(value))
        };

        Some(Modifier::Arbitrary {
            prefix,
            hint,
            value,
        })
    } else {
        Some(Modifier::Builtin {
            is_negative,
            value: modifier,
        })
    }
}

#[cfg(test)]
mod tests {
    // NOTE: In these tests, the order value of the selectors and variants are arbitrary because
    // the implementation of PartialEq does not take them into account, so instead they are written
    // as Default::default()
    use super::*;

    #[allow(clippy::wildcard_imports)]
    use crate::{config::Config, plugins::*, selector::Selector};

    use pretty_assertions::assert_eq;

    #[test]
    fn underscores_to_spaces_test() {
        assert_eq!(
            underscores_to_spaces(Cow::Borrowed(
                "url('/hello_world.png'),url(\"some__text.txt\")"
            )),
            "url('/hello_world.png'),url(\"some__text.txt\")",
        );
        assert_eq!(
            underscores_to_spaces(Cow::Borrowed(r#"url("[l8""#)),
            r#"url("[l8""#
        );
    }

    #[test]
    fn basic_single() {
        let config = Config::default();
        assert_eq!(
            parse(
                "absolute",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "absolute",
                order: Default::default(), // order is not checked in tests
                plugin: &layout::position::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "absolute",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn basic_multiple() {
        let config = Config::default();
        assert_eq!(
            parse(
                "text-center",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "text-center",
                order: Default::default(),
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "center",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn basic_opacity() {
        let config = Config::default();
        assert_eq!(
            parse(
                "bg-red-500/25",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "bg-red-500/25",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "red-500/25",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn basic_important() {
        let config = Config::default();
        assert_eq!(
            parse("!px-4", None, None, &config, &config.get_derived_variants())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                layer: 0,
                full: "!px-4",
                order: Default::default(),
                plugin: &spacing::padding::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "4",
                },
                is_important: true,
            }
        );
    }

    #[test]
    fn basic_negative() {
        let config = Config::default();
        assert_eq!(
            parse("-px-4", None, None, &config, &config.get_derived_variants())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                layer: 0,
                full: "-px-4",
                order: Default::default(),
                plugin: &spacing::padding::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: true,
                    value: "4",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn basic_important_and_negative() {
        let config = Config::default();
        assert_eq!(
            parse(
                "!-px-4",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "!-px-4",
                order: Default::default(),
                plugin: &spacing::padding::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: true,
                    value: "4",
                },
                is_important: true,
            }
        );
    }

    #[test]
    fn basic_integer() {
        let config = Config::default();
        assert_eq!(
            parse("px-4", None, None, &config, &config.get_derived_variants())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                layer: 0,
                full: "px-4",
                order: Default::default(),
                plugin: &spacing::padding::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "4",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn basic_float() {
        let config = Config::default();
        assert_eq!(
            parse(
                "px-1.5",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "px-1.5",
                order: Default::default(),
                plugin: &spacing::padding::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "1.5",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn variants_single() {
        let config = Config::default();
        assert_eq!(
            parse(
                "hover:text-center",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "hover:text-center",
                order: Default::default(),
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::Borrowed("&:hover")
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "center",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn variants_multiple() {
        let config = Config::default();
        assert_eq!(
            parse(
                "marker:xl:hover:text-center",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "marker:xl:hover:text-center",
                order: Default::default(),
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("& *::marker, &::marker")
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("@media (width >= 80rem)"),
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:hover")
                    }
                ],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "center",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn variants_negative() {
        let config = Config::default();
        assert_eq!(
            parse(
                "marker:xl:hover:-mx-4",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "marker:xl:hover:-mx-4",
                order: Default::default(),
                plugin: &spacing::margin::PluginXDefinition,
                variants: vec![
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("& *::marker, &::marker")
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("@media (width >= 80rem)"),
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:hover")
                    }
                ],
                modifier: Modifier::Builtin {
                    is_negative: true,
                    value: "4",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_variant() {
        let config = Config::default();
        assert_eq!(
            parse(
                "[&>*]:text-center",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "[&>*]:text-center",
                order: Default::default(),
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from("&>*")
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "center",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn group_and_peer() {
        let config = Config::default();
        assert_eq!(
            parse(
                "group-checked:block",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "group-checked:block",
                order: Default::default(),
                plugin: &layout::display::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from(".group:checked &"),
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "block",
                },
                is_important: false,
            }
        );

        assert_eq!(
            parse(
                "peer-checked:block",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "peer-checked:block",
                order: Default::default(),
                plugin: &layout::display::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from(".peer:checked ~ &"),
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "block",
                },
                is_important: false,
            }
        );

        assert_eq!(
            parse(
                "peer-not-checked:block",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "peer-not-checked:block",
                order: Default::default(),
                plugin: &layout::display::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from(".peer:not(:checked) ~ &"),
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "block",
                },
                is_important: false,
            }
        );

        assert_eq!(
            parse(
                "peer-[:focus-within]:block",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "peer-[:focus-within]:block",
                order: Default::default(),
                plugin: &layout::display::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from(".peer:focus-within ~ &"),
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "block",
                },
                is_important: false,
            }
        );

        assert_eq!(
            parse(
                "peer-[:nth-of-type(3)_&]:block",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "peer-[:nth-of-type(3)_&]:block",
                order: Default::default(),
                plugin: &layout::display::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from(":nth-of-type(3) .peer ~ &"),
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "block",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_named_group_and_peer() {
        let config = Config::default();
        assert_eq!(
            parse(
                "group-checked/item:block",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "group-checked/item:block",
                order: Default::default(),
                plugin: &layout::display::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from(r".group\/item:checked &"),
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "block",
                },
                is_important: false,
            }
        );

        assert_eq!(
            parse(
                "peer-checked/item:block",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "peer-checked/item:block",
                order: Default::default(),
                plugin: &layout::display::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from(r".peer\/item:checked ~ &"),
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "block",
                },
                is_important: false,
            }
        );

        assert_eq!(
            parse(
                "peer-not-checked/item:block",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "peer-not-checked/item:block",
                order: Default::default(),
                plugin: &layout::display::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from(r".peer\/item:not(:checked) ~ &"),
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "block",
                },
                is_important: false,
            }
        );

        assert_eq!(
            parse(
                "peer-[:focus-within]/item:block",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "peer-[:focus-within]/item:block",
                order: Default::default(),
                plugin: &layout::display::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from(".peer\\/item:focus-within ~ &"),
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "block",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_variant_at_rule() {
        let config = Config::default();
        assert_eq!(
            parse(
                "[@supports_not_(display:grid)]:grid",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "[@supports_not_(display:grid)]:grid",
                order: Default::default(),
                plugin: &layout::display::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from("@supports not (display:grid)")
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "grid",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_variant_multiple() {
        let config = Config::default();
        assert_eq!(
            parse(
                "xl:[&>*]:focus:text-center",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "xl:[&>*]:focus:text-center",
                order: Default::default(),
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("@media (width >= 80rem)")
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("&>*")
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:focus")
                    }
                ],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "center",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_variant_negative() {
        let config = Config::default();
        assert_eq!(
            parse(
                "xl:[&>*]:focus:-m-4",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "xl:[&>*]:focus:-m-4",
                order: Default::default(),
                plugin: &spacing::margin::PluginDefinition,
                variants: vec![
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("@media (width >= 80rem)")
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("&>*")
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:focus")
                    }
                ],
                modifier: Modifier::Builtin {
                    is_negative: true,
                    value: "4",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value() {
        let config = Config::default();
        assert_eq!(
            parse(
                "mx-[12px]",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "mx-[12px]",
                order: Default::default(),
                plugin: &spacing::margin::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from("12px"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn complex_arbitrary_value() {
        let config = Config::default();
        assert_eq!(
            parse(
                "bg-[url('/hello_world.png')]",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "bg-[url('/hello_world.png')]",
                order: Default::default(),
                plugin: &background::background_image::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from("url('/hello_world.png')"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_hint() {
        let config = Config::default();
        assert_eq!(
            parse(
                "bg-[color:#fff]",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "bg-[color:#fff]",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "color",
                    value: Cow::from("#fff"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_with_variants() {
        let config = Config::default();
        assert_eq!(
            parse(
                "xl:marker:bg-[#fff]",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "xl:marker:bg-[#fff]",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("@media (width >= 80rem)")
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("& *::marker, &::marker")
                    },
                ],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from("#fff"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_with_variants_and_hint() {
        let config = Config::default();
        assert_eq!(
            parse(
                "xl:marker:bg-[color:#fff]",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "xl:marker:bg-[color:#fff]",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("@media (width >= 80rem)")
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("& *::marker, &::marker")
                    },
                ],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "color",
                    value: Cow::from("#fff"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_with_arbitrary_variant() {
        let config = Config::default();
        assert_eq!(
            parse(
                "[&>*]:bg-[#fff]",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "[&>*]:bg-[#fff]",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from("&>*")
                },],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from("#fff"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_variant_escaped() {
        let config = Config::default();
        assert_eq!(
            parse(
                r"[&#91;type=&#39;input&#39;&#93;_&>:*]:bg-red-300",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: r"[&#91;type=&#39;input&#39;&#93;_&>:*]:bg-red-300",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from("[type='input'] &>:*")
                },],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "red-300",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_with_arbitrary_variant_mixed() {
        let config = Config::default();
        assert_eq!(
            parse(
                "xl:[&>*]:hover:bg-[#fff]",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "xl:[&>*]:hover:bg-[#fff]",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("@media (width >= 80rem)")
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("&>*")
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:hover")
                    }
                ],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from("#fff"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_with_arbitrary_variant_and_hint() {
        let config = Config::default();
        assert_eq!(
            parse(
                "xl:[&>*]:hover:bg-[color:#fff]",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: "xl:[&>*]:hover:bg-[color:#fff]",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("@media (width >= 80rem)")
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("&>*")
                    },
                    Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:hover")
                    }
                ],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "color",
                    value: Cow::from("#fff"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_escaped() {
        let config = Config::default();
        assert_eq!(
            parse(
                r"bg-[url(&#34;/url_with_&#93;&#41;&#39;.png&#34;)]",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 0,
                full: r"bg-[url(&#34;/url_with_&#93;&#41;&#39;.png&#34;)]",
                order: Default::default(),
                plugin: &background::background_image::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from(r#"url("/url_with_])'.png")"#),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_css_property() {
        let config = Config::default();
        assert_eq!(
            parse(
                "hover:[mask-type:luminance]",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                layer: 127,
                full: "hover:[mask-type:luminance]",
                order: Default::default(),
                plugin: &CssPropertyPlugin,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::Borrowed("&:hover")
                }],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from("mask-type:luminance"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn variant_grouping() {
        let config = Config::default();
        assert_eq!(
            parse(
                "hover:(focus:bg-gray-500,text-[color:black,])",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    layer: 0,
                    full: "hover:(focus:bg-gray-500,text-[color:black,])",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:hover")
                        }
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "gray-500",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "hover:(focus:bg-gray-500,text-[color:black,])",
                    order: Default::default(),
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:hover")
                    }],
                    modifier: Modifier::Arbitrary {
                        prefix: "",
                        hint: "color",
                        value: Cow::from("black,"),
                    },
                    is_important: false,
                })
            ],
        );
    }

    #[test]
    fn variant_grouping_single() {
        let config = Config::default();
        assert_eq!(
            parse(
                "hover:(bg-gray-500)",
                None,
                None,
                &config,
                &config.get_derived_variants()
            ),
            vec![Ok(Selector {
                layer: 0,
                full: "hover:(bg-gray-500)",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::Borrowed("&:hover")
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "gray-500",
                },
                is_important: false,
            })],
        );
    }

    #[test]
    fn prefixed_variant() {
        let config = Config::default();
        assert_eq!(
            parse(
                "min:visible",
                None,
                None,
                &config,
                &config.get_derived_variants()
            ),
            vec![Err(ParseError::new(
                0..11,
                ParseErrorKind::UnknownVariant("min", "min:visible")
            ))],
        );

        assert_eq!(
            parse(
                "min-[475px]:visible",
                None,
                None,
                &config,
                &config.get_derived_variants()
            ),
            vec![Ok(Selector {
                layer: 0,
                full: "min-[475px]:visible",
                order: Default::default(),
                plugin: &layout::visibility::PluginDefinition,
                variants: vec![Variant {
                    order: Default::default(),
                    prefixed: false,
                    template: Cow::from("@media (width >= 475px)")
                }],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "visible",
                },
                is_important: false,
            })],
        );
    }

    #[test]
    fn variant_grouping_nested() {
        let config = Config::default();
        assert_eq!(
            parse(
                "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    layer: 0,
                    full: "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                    order: Default::default(),
                    plugin: &spacing::margin::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("&>*")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: true,
                        value: "4",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (width >= 80rem)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (prefers-color-scheme: dark)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-100",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                    order: Default::default(),
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("[dir=\"rtl\"] &")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (width >= 80rem)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (prefers-color-scheme: dark)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                    ],
                    modifier: Modifier::Arbitrary {
                        prefix: "",
                        hint: "color",
                        value: Cow::from("black"),
                    },
                    is_important: false,
                }),
            ],
        );
    }

    #[test]
    fn variant_grouping_nested_escaped() {
        let config = Config::default();
        assert_eq!(
            parse(
                r"focus:([&>*]:-m-4,xl:dark:([&#91;type=&#39;text&#39;&#93;.light_&,.foo]:bg-red-100,text-[color:black,]))",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    layer: 0,
                    full: r"focus:([&>*]:-m-4,xl:dark:([&#91;type=&#39;text&#39;&#93;.light_&,.foo]:bg-red-100,text-[color:black,]))",
                    order: Default::default(),
                    plugin: &spacing::margin::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("&>*")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: true,
                        value: "4",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: r"focus:([&>*]:-m-4,xl:dark:([&#91;type=&#39;text&#39;&#93;.light_&,.foo]:bg-red-100,text-[color:black,]))",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from(r"[type='text'].light &,.foo")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (width >= 80rem)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (prefers-color-scheme: dark)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-100",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: r"focus:([&>*]:-m-4,xl:dark:([&#91;type=&#39;text&#39;&#93;.light_&,.foo]:bg-red-100,text-[color:black,]))",
                    order: Default::default(),
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (width >= 80rem)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (prefers-color-scheme: dark)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                    ],
                    modifier: Modifier::Arbitrary {
                        prefix: "",
                        hint: "color",
                        value: Cow::from("black,"),
                    },
                    is_important: false,
                }),
            ],
        );
    }

    #[test]
    fn variant_grouping_complex_nested() {
        let config = Config::default();
        assert_eq!(
            parse(
                r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    layer: 0,
                    full: r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                    order: Default::default(),
                    plugin: &border::outline_style::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (width >= 80rem)")
                        }
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                    order: Default::default(),
                    plugin: &border::outline_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (width >= 80rem)")
                        }
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-200",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (prefers-color-scheme: dark)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (width >= 80rem)")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "black",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                    order: Default::default(),
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (prefers-color-scheme: dark)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (width >= 80rem)")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "white",
                    },
                    is_important: false,
                }),
            ],
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn variant_grouping_without_modifier() {
        let config = Config::default();
        assert_eq!(
            parse(
                "(hover,focus):bg-red-400",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    layer: 0,
                    full: "(hover,focus):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:hover")
                    }],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "(hover,focus):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:focus")
                    },],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                })
            ],
        );

        assert_eq!(
            parse(
                "([@supports_(display:flex)],focus-visible):flex",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    layer: 0,
                    full: "([@supports_(display:flex)],focus-visible):flex",
                    order: Default::default(),
                    plugin: &flexbox::flex::PluginDefinition,
                    variants: vec![Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("@supports (display:flex)")
                    }],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "flex",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "([@supports_(display:flex)],focus-visible):flex",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:focus-visible")
                    },],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "flex",
                    },
                    is_important: false,
                })
            ],
        );

        assert_eq!(
            parse(
                "([@supports_(display:flex)],focus-visible):!-m-4",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    layer: 0,
                    full: "([@supports_(display:flex)],focus-visible):!-m-4",
                    order: Default::default(),
                    plugin: &spacing::margin::PluginDefinition,
                    variants: vec![Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::from("@supports (display:flex)")
                    }],
                    modifier: Modifier::Builtin {
                        is_negative: true,
                        value: "4",
                    },
                    is_important: true,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "([@supports_(display:flex)],focus-visible):!-m-4",
                    order: Default::default(),
                    plugin: &spacing::margin::PluginDefinition,
                    variants: vec![Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:focus-visible")
                    },],
                    modifier: Modifier::Builtin {
                        is_negative: true,
                        value: "4",
                    },
                    is_important: true,
                })
            ],
        );

        assert_eq!(
            parse(
                "xl:(hover,focus):bg-red-400",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    layer: 0,
                    full: "xl:(hover,focus):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("@media (width >= 80rem)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:hover")
                        }
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "xl:(hover,focus):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("@media (width >= 80rem)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                })
            ],
        );

        // Not a variant without a modifier but we need to make sure it is correctly interpreted as
        // variants **with** a modifier
        assert_eq!(
            parse(
                "(hover:bg-red-400,focus:bg-green-400)",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    layer: 0,
                    full: "(hover:bg-red-400,focus:bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:hover")
                    }],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "(hover:bg-red-400,focus:bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:focus")
                    },],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                })
            ],
        );

        assert_eq!(
            parse(
                "xl:(hover,focus):target:(dark:bg-red-400,bg-green-400)",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    layer: 0,
                    full: "xl:(hover,focus):target:(dark:bg-red-400,bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (prefers-color-scheme: dark)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("@media (width >= 80rem)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:hover")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:target")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "xl:(hover,focus):target:(dark:bg-red-400,bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::from("@media (prefers-color-scheme: dark)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("@media (width >= 80rem)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:target")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "xl:(hover,focus):target:(dark:bg-red-400,bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("@media (width >= 80rem)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:hover")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:target")
                        }
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "xl:(hover,focus):target:(dark:bg-red-400,bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("@media (width >= 80rem)")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:target")
                        }
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                }),
            ],
        );

        assert_eq!(
            parse(
                "(hover,focus):(focus-within,target):bg-red-400",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    layer: 0,
                    full: "(hover,focus):(focus-within,target):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:hover")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus-within")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "(hover,focus):(focus-within,target):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus-within")
                        }
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "(hover,focus):(focus-within,target):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:hover")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:target")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "(hover,focus):(focus-within,target):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:target")
                        }
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                })
            ],
        );

        assert_eq!(
            parse(
                "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    layer: 0,
                    full: "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:hover")
                    },],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant {
                        order: Default::default(),
                        prefixed: false,
                        template: Cow::Borrowed("&:focus")
                    },],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:target")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:hover")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:target")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        },
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus-within")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:hover")
                        }
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    layer: 0,
                    full: "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus-within")
                        },
                        Variant {
                            order: Default::default(),
                            prefixed: false,
                            template: Cow::Borrowed("&:focus")
                        }
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                }),
            ],
        );
    }
}
