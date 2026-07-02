//! Define the main [`generate`] function used to scan content and to generate CSS styles.
use crate::{
    config::{Config, MaxShortcutDepth},
    plugins::{Plugin, PluginKind, PropertyName},
    preflight::Preflight,
    selector::{Modifier, Selector, Variant, parse},
    utils::{buffer::Buffer, color, shadow, spacing},
};

use std::{borrow::Cow, collections::BTreeSet};

/// The context used in the [`Plugin::can_handle`] method.
///
/// [`Plugin::can_handle`]: crate::plugins::Plugin::can_handle
#[derive(Debug)]
pub struct ContextCanHandle<'a, 'b, 'c> {
    /// The generator's configuration.
    pub config: &'a Config,

    /// The modifier which will be checked.
    pub modifier: &'b Modifier<'c>,
}

/// The context used in the [`Plugin::handle`] method.
///
/// [`Plugin::handle`]: crate::plugins::Plugin::handle
#[derive(Debug)]
pub struct ContextHandle<'a, 'b, 'c, 'd, 'e> {
    /// The generator's configuration.
    pub config: &'a Config,

    /// The modifier which will have its CSS generated.
    pub modifier: &'b Modifier<'c>,

    /// The buffer containing the whole generated CSS.
    pub buffer: &'d mut Buffer,

    // Private fields used in `generate_class` and `generate_at_rules`
    selector: &'e Selector<'e>,
}

fn push_css_lines(prop: &PropertyName, value: &str, context: &mut ContextHandle) {
    match prop {
        PropertyName::SingleProp(prop) => {
            context.buffer.line(format_args!("{prop}: {value};"));
        }
        PropertyName::MultipleProps(props) => {
            for prop in *props {
                context.buffer.line(format_args!("{prop}: {value};"));
            }
        }
    }
}

fn push_css_lines_with_templating(
    prop: &PropertyName,
    plugin: &Plugin,
    value: &str,
    slash_value: Option<&str>,
    context: &mut ContextHandle,
) {
    let transform_template = |template: &str| {
        if let Some(slash_value) = slash_value {
            template.replace("{}", &value).replace("{/}", slash_value)
        } else {
            template.replace("{}", &value)
        }
    };

    match prop {
        PropertyName::SingleProp(prop) => {
            let value = if let Some(template) = plugin.template {
                Cow::Owned(transform_template(template))
            } else {
                Cow::Borrowed(value)
            };

            context.buffer.line(format_args!("{prop}: {value};"));
        }
        PropertyName::MultipleProps(props) => {
            if let Some(templates) = plugin.template_multiple {
                for (i, prop) in props.iter().enumerate() {
                    // The length of plugin.template_multiple is asserted to be the
                    // same as the length of the list of property names at compile
                    // time in the method Plugin::template_multiple
                    let value = transform_template(templates[i]);
                    context.buffer.line(format_args!("{prop}: {value};",));
                }
            } else if let Some(template) = plugin.template {
                let value = transform_template(template);
                for prop in *props {
                    context.buffer.line(format_args!("{prop}: {value};"));
                }
            } else {
                for prop in *props {
                    context.buffer.line(format_args!("{prop}: {value};"));
                }
            }
        }
    }
}

fn add_extra_css(plugin: &Plugin, context: &mut ContextHandle, value: &str) {
    if let Some(extra_css) = &plugin.extra_css {
        let Some(css) = extra_css.get(value) else {
            return;
        };

        if !css.is_empty() {
            context.buffer.raw(css);
        }
    }
}

fn handle(plugin: &Plugin, context: &mut ContextHandle) {
    match (&plugin.kind, context.modifier) {
        (PluginKind::ListCases { cases }, Modifier::Builtin { value, .. }) => {
            add_extra_css(plugin, context, value);

            generate_at_rules(context, |context| {
                generate_class(
                    context,
                    |context| {
                        context.buffer.lines(
                            *cases
                                .get(value)
                                .expect("key existence was checked in can_handle"),
                        );

                        if let Some(extra_lines) = plugin.extra_lines {
                            context.buffer.lines(extra_lines);
                        }
                    },
                    plugin.extra_class.unwrap_or(""),
                );
            });
        }
        (PluginKind::ListValues { prop, values }, Modifier::Builtin { value, .. }) => {
            let (value, template_value) = if plugin.extra_slash.is_some()
                && let Some(index) = value.find('/')
            {
                let (before, after) = value.split_at(index);
                (before, Some(&after[1..]))
            } else {
                (*value, plugin.extra_slash.as_ref().map(|e| e.1))
            };

            add_extra_css(plugin, context, &*value);
            let value = *values
                .get(&*value)
                .expect("key existence was checked in can_handle");

            let value = if let Some((values, _)) = &plugin.extra_slash {
                Cow::Owned(
                    value.replace(
                        "{/}",
                        values
                            .get(template_value.unwrap())
                            .expect("extra_slash values are checked in can_hamdle"),
                    ),
                )
            } else {
                Cow::Borrowed(value)
            };

            generate_at_rules(context, |context| {
                generate_class(
                    context,
                    |context| {
                        push_css_lines(prop, &*value, context);

                        if let Some(extra_lines) = plugin.extra_lines {
                            context.buffer.lines(extra_lines);
                        }
                    },
                    plugin.extra_class.unwrap_or(""),
                );
            });
        }
        (PluginKind::SamePropValues { prop, .. }, Modifier::Builtin { value, .. }) => {
            generate_at_rules(context, |context| {
                generate_class(
                    context,
                    |context| {
                        push_css_lines(prop, value, context);

                        if let Some(extra_lines) = plugin.extra_lines {
                            context.buffer.lines(extra_lines);
                        }
                    },
                    plugin.extra_class.unwrap_or(""),
                );
            });
        }
        (
            PluginKind::Sizing {
                prop,
                is_horizontal,
                ..
            },
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

            add_extra_css(plugin, context, &*value);

            generate_at_rules(context, |context| {
                generate_class(
                    context,
                    |context| {
                        let value = match &*value {
                            "none" => Cow::Borrowed("none"),
                            "auto" => Cow::Borrowed("auto"),
                            "full" => Cow::Borrowed("100%"),
                            "screen" if *is_horizontal => Cow::Borrowed("100vw"),
                            "screen" if !is_horizontal => Cow::Borrowed("100vh"),
                            "min" => Cow::Borrowed("min-content"),
                            "max" => Cow::Borrowed("max-content"),
                            "fit" => Cow::Borrowed("fit-content"),
                            "svw" if *is_horizontal => Cow::Borrowed("100svw"),
                            "lvw" if *is_horizontal => Cow::Borrowed("100lvw"),
                            "dvw" if *is_horizontal => Cow::Borrowed("100dvw"),
                            "svh" if !is_horizontal => Cow::Borrowed("100svh"),
                            "lvh" if !is_horizontal => Cow::Borrowed("100lvh"),
                            "dvh" if !is_horizontal => Cow::Borrowed("100dvh"),
                            _ => spacing::get(value, *is_negative).unwrap(),
                        };
                        push_css_lines_with_templating(
                            prop,
                            plugin,
                            &value,
                            template_value,
                            context,
                        );

                        if let Some(extra_lines) = plugin.extra_lines {
                            context.buffer.lines(extra_lines);
                        }
                    },
                    plugin.extra_class.unwrap_or(""),
                );
            });
        }
        (
            PluginKind::Spacing { prop, .. },
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

            add_extra_css(plugin, context, &*value);

            generate_at_rules(context, |context| {
                generate_class(
                    context,
                    |context| {
                        let value = if &*value == "auto" {
                            Cow::Borrowed("auto")
                        } else if &*value == "full" {
                            if *is_negative {
                                Cow::Borrowed("-100%")
                            } else {
                                Cow::Borrowed("100%")
                            }
                        } else {
                            spacing::get(value, *is_negative).unwrap()
                        };
                        push_css_lines_with_templating(
                            prop,
                            plugin,
                            &*value,
                            template_value,
                            context,
                        );

                        if let Some(extra_lines) = plugin.extra_lines {
                            context.buffer.lines(extra_lines);
                        }
                    },
                    plugin.extra_class.unwrap_or(""),
                );
            });
        }
        (PluginKind::Color { prop }, Modifier::Builtin { value, .. }) => {
            generate_at_rules(context, |context| {
                generate_class(
                    context,
                    |context| {
                        let value = color::get(context.config, &*value).unwrap();
                        push_css_lines_with_templating(prop, plugin, &*value, None, context);

                        if let Some(extra_lines) = plugin.extra_lines {
                            context.buffer.lines(extra_lines);
                        }
                    },
                    plugin.extra_class.unwrap_or(""),
                );
            });
        }
        (
            PluginKind::AnyNumber {
                prop, divide_by, ..
            },
            Modifier::Builtin { value, is_negative },
        ) => {
            let (value, template_value) = if plugin.extra_slash.is_some()
                && let Some(index) = value.find('/')
            {
                let (before, after) = value.split_at(index);
                (before, Some(&after[1..]))
            } else {
                (*value, plugin.extra_slash.as_ref().map(|e| e.1))
            };

            add_extra_css(plugin, context, &*value);

            generate_at_rules(context, |context| {
                generate_class(
                    context,
                    |context| {
                        let value = if value.is_empty() {
                            1.0
                        } else {
                            value.parse::<usize>().unwrap() as f32 / divide_by
                        };
                        let coeff = if *is_negative { -1.0 } else { 1.0 };
                        let value = (value * coeff).to_string();

                        push_css_lines_with_templating(
                            prop,
                            plugin,
                            &*value,
                            template_value,
                            context,
                        );

                        if let Some(extra_lines) = plugin.extra_lines {
                            context.buffer.lines(extra_lines);
                        }
                    },
                    plugin.extra_class.unwrap_or(""),
                );
            });
        }
        (PluginKind::OnlyArbitrary { prop, .. }, Modifier::Arbitrary { value, .. }) => {
            generate_at_rules(context, |context| {
                generate_class(
                    context,
                    |context| {
                        push_css_lines_with_templating(prop, plugin, &*value, None, context);

                        if let Some(extra_lines) = plugin.extra_lines {
                            context.buffer.lines(extra_lines);
                        }
                    },
                    plugin.extra_class.unwrap_or(""),
                );
            });
        }
        (
            PluginKind::ArbitraryShadow {
                prop,
                color_replacement,
            },
            Modifier::Arbitrary { value, .. },
        ) => {
            generate_at_rules(context, |context| {
                generate_class(
                    context,
                    |context| {
                        let mut shadow = shadow::ShadowList::parse(value).unwrap();
                        shadow.replace_all_colors(color_replacement);

                        push_css_lines(prop, &shadow.to_string(), context);

                        if let Some(extra_lines) = plugin.extra_lines {
                            context.buffer.lines(extra_lines);
                        }
                    },
                    plugin.extra_class.unwrap_or(""),
                );
            });
        }
        (PluginKind::Functional { handle, .. }, _) => {
            handle(context);
        }
        _ => unreachable!(
            "Only plugins which can be handled are supposed to be handled. However {plugin:?} cannot handle {:?} but passed can_handle check. This is a bug in encre-css, please report it.",
            context.modifier
        ),
    }
}

/// Generate the needed CSS at-rules (e.g @media).
///
/// Note: The inner class (e.g. .foo-bar) is not handled by this function, see [`generate_wrapper`].
///
/// The second argument, a closure, is called to generate the CSS content of the rule.
///
/// # Errors
///
/// Returns [`fmt::Error`] indicating whether writing to the buffer succeeded.
///
/// [`fmt::Error`]: std::fmt::Error
pub fn generate_at_rules<T: FnOnce(&mut ContextHandle)>(
    context: &mut ContextHandle,
    rule_content_fn: T,
) {
    let ContextHandle {
        buffer, selector, ..
    } = context;

    if !selector.variants.is_empty() {
        selector.variants.iter().for_each(|variant| {
            if variant.template.starts_with('@') {
                buffer.line(format_args!("{} {{", variant.template));
                buffer.indent();
            }
        });
    }

    rule_content_fn(context);

    let ContextHandle { buffer, .. } = context;
    while !buffer.is_unindented() {
        buffer.unindent();

        if buffer.is_unindented() {
            buffer.raw("}");
        } else {
            buffer.line("}");
        }
    }
}

/// Generate a CSS rule with a class.
///
/// Note: At-rules (e.g. @media) are not handled by this function, see [`generate_wrapper`].
///
/// The second argument, a closure, is called to generate the CSS content of the rule.
/// The third argument is used to add a custom string just after the class (e.g. `> *`).
///
/// # Errors
///
/// Returns [`fmt::Error`] indicating whether writing to the buffer succeeded.
///
/// [`fmt::Error`]: std::fmt::Error
#[allow(clippy::too_many_lines)]
pub fn generate_class<T: FnOnce(&mut ContextHandle)>(
    context: &mut ContextHandle,
    rule_content_fn: T,
    custom_after_class: &str,
) {
    let ContextHandle {
        buffer, selector, ..
    } = context;

    // Write the class
    let mut base_class = String::with_capacity(1 + selector.full.len());
    base_class.push('.');

    // The browser will automatically replace the escape codes in the classes, so we need to also
    // replace them in the generated CSS full selector
    let unescaped_full_selector =
        crate::selector::parser::replace_escape_codes(Cow::Borrowed(selector.full));
    unescaped_full_selector
        .chars()
        .enumerate()
        .for_each(|(i, ch)| {
            if !ch.is_alphanumeric() && ch != '-' && ch != '_' {
                base_class.push('\\');
                base_class.push(ch);
            } else if i == 0 && ch.is_numeric() {
                // CSS classes must not start with a number, we need to escape it
                base_class.push_str("\\3");
                base_class.push(ch);
            } else {
                base_class.push(ch);
            }
        });

    if !selector.variants.is_empty() {
        // Variants are applied from right to left
        // (https://tailwindcss.com/docs/upgrade-guide#variant-stacking-order),
        // so no need to reverse the variants
        selector.variants.iter().for_each(|variant| {
            if !variant.template.starts_with('@') {
                base_class = variant.template.replace('&', &base_class);
            }
        });
    }
    buffer.line(format_args!("{base_class}{custom_after_class} {{"));

    // Store the index of the start of the class content (useful when the `important` flag is present)
    let content_start = buffer.len();

    // Rule content
    buffer.indent();
    rule_content_fn(context);

    let ContextHandle {
        buffer, selector, ..
    } = context;

    // If the rule is selecting the `::before` or `::after` pseudo elements, we need to generate a
    // default `content` property
    if selector
        .variants
        .iter()
        .any(|variant| ["&::before", "&::after"].contains(&&*variant.template))
    {
        buffer.line("content: var(--en-content);");
    }

    // If the `important` flag is present we need to replace all `;\n` or `;\r\n`
    // to ` !important;\n` or ` !important;\r\n`
    if selector.is_important {
        let mut extra_index = 0;
        let positions = buffer[content_start..]
            .match_indices('\n')
            .map(|i| i.0)
            .collect::<Vec<usize>>();

        for index in positions {
            if index - 1 == 0 {
                continue;
            }

            let index = content_start + extra_index + index;
            let index = if &buffer[index - 1..index] == "\r" {
                index - 1
            } else {
                index
            };
            let replace_with = " !important;";
            buffer.replace_range(index - 1..index, replace_with);
            extra_index += replace_with.len() - 1;
        }
    }

    buffer.unindent();
    if buffer.is_unindented() {
        buffer.raw("}");
    } else {
        buffer.line("}");
    }
}

fn resolve_selector<'a>(
    selector: &'a str,
    full_class: Option<&'a str>,
    selectors: &mut BTreeSet<Selector<'a>>,
    config: &'a Config,
    config_derived_variants: &[(Cow<'static, str>, Variant<'static>)],
    depth: MaxShortcutDepth,
) {
    if depth.get() == 0 {
        return;
    }

    if let Some(expanded) = config.shortcuts.get(selector) {
        expanded.split(' ').for_each(|shortcut_target| {
            resolve_selector(
                shortcut_target,
                full_class.or(Some(selector)),
                selectors,
                config,
                config_derived_variants,
                MaxShortcutDepth::new(depth.get() - 1),
            );
        });
    } else {
        selectors.extend(
            parse(selector, None, full_class, config, config_derived_variants)
                .into_iter()
                .filter_map(Result::ok),
        );
    }
}

/// Generate the CSS styles needed based on the given sources.
///
/// Each source will be scanned in order to extract atomic classes, then CSS will be generated for
/// each class found.
///
/// By default, it splits the source by spaces, double quotes, single quotes, backticks and new
/// lines, while ignoring the content inside arbitrary values/variants and variant groups.
///
/// This function also removes duplicated selectors and sorts the generated CSS classes based on
/// the order in which they were defined to avoid conflicts.
pub fn generate<'a>(sources: impl IntoIterator<Item = &'a str>, config: &Config) -> String {
    let config_derived_variants = config.get_derived_variants();
    let mut selectors = BTreeSet::new();

    // Add selectors from the safelist
    for safe_selector in config.safelist.iter() {
        if let Some(expanded) = config.shortcuts.get(&**safe_selector) {
            expanded.split(' ').for_each(|shortcut_target| {
                selectors.extend(
                    parse(
                        shortcut_target,
                        None,
                        Some(safe_selector),
                        config,
                        &config_derived_variants,
                    )
                    .into_iter()
                    .filter_map(Result::ok),
                );
            });
        } else {
            selectors.extend(
                parse(safe_selector, None, None, config, &config_derived_variants)
                    .into_iter()
                    .filter_map(Result::ok),
            );
        }
    }

    for source in sources {
        let new_selectors = config.scanner.scan(source);

        for selector in new_selectors {
            resolve_selector(
                selector,
                None,
                &mut selectors,
                config,
                &config_derived_variants,
                config.max_shortcut_depth,
            );
        }
    }

    let preflight = config.preflight.build();
    let mut buffer = Buffer::with_capacity(10 * selectors.len()); // TODO: More accurate value
    buffer.raw(&preflight);

    for selector in selectors {
        if buffer.len() != preflight.len() || config.preflight != Preflight::None {
            buffer.raw("\n\n");
        }

        let mut context = ContextHandle {
            config,
            modifier: &selector.modifier,
            buffer: &mut buffer,
            selector: &selector,
        };

        handle(selector.plugin, &mut context);
    }

    buffer.into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::DarkMode, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn not_parsing_too_loosely() {
        let generated = generate(["flex-test-[]"], &base_config());
        assert!(generated.is_empty());
    }

    #[test]
    fn divide_and_space_between_special_class() {
        let generated = generate(
            [
                "hover:space-x-1",
                "space-x-2",
                "[&:has(.class)_>_*]:space-y-3",
                "divide-red-100",
                "divide-dashed",
                "divide-x-[11px]",
                "xl:[&_>_*]:divide-y-2",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".space-x-2 > :not(:last-child) {
  margin-inline-start: calc(0.5rem * var(--en-space-x-reverse));
  margin-inline-end: calc(0.5rem * calc(1 - var(--en-space-x-reverse)));
  --en-space-x-reverse: 0;
}

.divide-x-\[11px\] > :not([hidden]) ~ :not([hidden]) {
  border-inline-start-width: calc(11px * var(--en-divide-x-reverse));
  border-inline-end-width: calc(11px * calc(1 - var(--en-divide-x-reverse)));
  --en-divide-x-reverse: 0;
}

.divide-dashed > :not([hidden]) ~ :not([hidden]) {
  border-style: dashed;
}

.divide-red-100 > :not([hidden]) ~ :not([hidden]) {
  border-color: oklch(93.6% .032 17.717);
}

.hover\:space-x-1:hover > :not(:last-child) {
  margin-inline-start: calc(0.25rem * var(--en-space-x-reverse));
  margin-inline-end: calc(0.25rem * calc(1 - var(--en-space-x-reverse)));
  --en-space-x-reverse: 0;
}

@media (width >= 80rem) {
  .xl\:\[\&_\>_\*\]\:divide-y-2 > * > :not([hidden]) ~ :not([hidden]) {
    border-block-start-width: calc(2px * var(--en-divide-y-reverse));
    border-block-end-width: calc(2px * calc(1 - var(--en-divide-y-reverse)));
    --en-divide-y-reverse: 0;
  }
}

.\[\&\:has\(\.class\)_\>_\*\]\:space-y-3:has(.class) > * > :not(:last-child) {
  margin-block-start: calc(0.75rem * var(--en-space-y-reverse));
  margin-block-end: calc(0.75rem * calc(1 - var(--en-space-y-reverse)));
  --en-space-y-reverse: 0;
}"
            )
        );
    }

    #[test]
    fn negative_values() {
        let generated = generate(
            [
                "-top-2",
                "-z-2",
                "-order-2",
                "-mb8",
                "-translate-x-52",
                "-rotate-90",
                "-skew-x-2",
                "-scale-50",
                "-scroll-mt-2",
                "-space-x-2",
                "-indent-2",
                "-hue-rotate-60",
                "hover:-hue-rotate-60",
                "-backdrop-hue-rotate-90",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".-top-2 {
  top: -0.5rem;
}

.-z-2 {
  z-index: -2;
}

.-order-2 {
  order: -2;
}

.-mb8 {
  margin-bottom: -2rem;
}

.-translate-x-52 {
  --en-translate-x: -13rem;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}

.-rotate-90 {
  --en-rotate-x: -90deg;
  --en-rotate-y: -90deg;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}

.-skew-x-2 {
  --en-skew-x: -2deg;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}

.-scale-50 {
  --en-scale-x: -0.5;
  --en-scale-y: -0.5;
  transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));
}

.-scroll-mt-2 {
  scroll-margin-top: -0.5rem;
}

.-space-x-2 > :not(:last-child) {
  margin-inline-start: calc(-0.5rem * var(--en-space-x-reverse));
  margin-inline-end: calc(-0.5rem * calc(1 - var(--en-space-x-reverse)));
  --en-space-x-reverse: 0;
}

.-indent-2 {
  text-indent: -0.5rem;
}

.-hue-rotate-60 {
  --en-hue-rotate: hue-rotate(-60deg);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}

.-backdrop-hue-rotate-90 {
  --en-backdrop-hue-rotate: hue-rotate(-90deg);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}

.hover\:-hue-rotate-60:hover {
  --en-hue-rotate: hue-rotate(-60deg);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"
            )
        );
    }

    #[test]
    fn gen_css_for_simple_selector() {
        let generated = generate(["text-current"], &base_config());

        assert_eq!(
            generated,
            String::from(
                ".text-current {
  color: currentColor;
}"
            )
        );
    }

    #[test]
    fn gen_css_with_important_flag() {
        let generated = generate(
            [
                "!w-full",
                "!-mb-8",
                "!shadow-sm",
                "!-hue-rotate-60",
                "focus:!w-2",
                "focus:!-mb-2",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".\!-mb-8 {
  margin-bottom: -2rem !important;
}

.\!w-full {
  width: 100% !important;
}

.\!shadow-sm {
  --en-shadow: 0 1px 3px 0 var(--en-shadow-color, rgb(0 0 0 / 0.1)), 0 1px 2px -1px var(--en-shadow-color, rgb(0 0 0 / 0.1)) !important;
  box-shadow: var(--en-inset-shadow, 0 0 #0000), var(--en-inset-ring-shadow, 0 0 #0000), var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow) !important;
}

.\!-hue-rotate-60 {
  --en-hue-rotate: hue-rotate(-60deg) !important;
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow) !important;
}

.focus\:\!-mb-2:focus {
  margin-bottom: -0.5rem !important;
}

.focus\:\!w-2:focus {
  width: 0.5rem !important;
}",
            )
        );
    }

    #[test]
    fn gen_css_for_selector_needing_custom_css() {
        let generated = generate(["animate-pulse", "animate-pulse"], &base_config());

        assert_eq!(
            generated,
            String::from(
                "@-webkit-keyframes pulse {
  50% {
    opacity: .5;
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: .5;
  }
}

.animate-pulse {
  -webkit-animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}"
            )
        );
    }

    #[test]
    fn gen_css_for_arbitrary_value() {
        let generated = generate(
            [
                "w[12px]",
                "bg-[red]",
                "bg-[url(../img/image_with_underscores.png)]",
                "mt-[calc(100%-10px)]",
                "2xl:pb-[calc((100%/2)-10px+2rem)]",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".mt-\[calc\(100\%-10px\)\] {
  margin-top: calc(100% - 10px);
}

.w\[12px\] {
  width: 12px;
}

.bg-\[red\] {
  background-color: red;
}

.bg-\[url\(\.\.\/img\/image_with_underscores\.png\)\] {
  background-image: url(../img/image_with_underscores.png);
}

@media (width >= 96rem) {
  .\32xl\:pb-\[calc\(\(100\%\/2\)-10px\+2rem\)\] {
    padding-bottom: calc((100% / 2) - 10px + 2rem);
  }
}"
            )
        );
    }

    #[test]
    fn gen_css_for_arbitrary_value_with_hint() {
        let generated = generate(["bg-[color:red]", "hover:bg-[color:red]"], &base_config());

        assert_eq!(
            generated,
            String::from(
                r".bg-\[color\:red\] {
  background-color: red;
}

.hover\:bg-\[color\:red\]:hover {
  background-color: red;
}"
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_simple_variant() {
        let generated = generate(["focus:w-full"], &base_config());

        assert_eq!(
            generated,
            String::from(
                r".focus\:w-full:focus {
  width: 100%;
}"
            )
        );
    }

    #[test]
    fn gen_selector_css_variants_test() {
        let generated = generate(
            [
                "sm:hover:bg-red-400",
                "focus:hover:bg-red-600",
                "active:rtl:bg-red-800",
                "md:focus:selection:bg-blue-100",
                "rtl:active:focus:lg:underline",
                "print:ltr:xl:hover:focus:active:text-yellow-300",
                "2xl:motion-safe:landscape:focus-within:visited:first:odd:checked:open:rtl:bg-purple-100",
                "hover:file:bg-pink-600",
                "file:hover:bg-pink-600",
                "sm:before:target:content-[&#39;Hello_world!&#39;]",
                "marker:selection:hover:bg-green-200",
                "group-hover:bg-green-300",
                "group-focus:bg-green-400",
                "peer-invalid:bg-red-500",
                "peer-not-invalid:bg-green-500",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r#".marker\:selection\:hover\:bg-green-200 *::marker, .marker\:selection\:hover\:bg-green-200::marker *::selection, .marker\:selection\:hover\:bg-green-200 *::marker, .marker\:selection\:hover\:bg-green-200::marker::selection:hover {
  background-color: oklch(92.5% .084 155.995);
}

.file\:hover\:bg-pink-600::file-selector-button, .file\:hover\:bg-pink-600::-webkit-file-upload-button:hover {
  background-color: oklch(59.2% .249 .584);
}

.hover\:file\:bg-pink-600:hover::file-selector-button, .hover\:file\:bg-pink-600:hover::-webkit-file-upload-button {
  background-color: oklch(59.2% .249 .584);
}

.focus\:hover\:bg-red-600:focus:hover {
  background-color: oklch(57.7% .245 27.325);
}

[dir="rtl"] .active\:rtl\:bg-red-800:active {
  background-color: oklch(44.4% .177 26.899);
}

@media (width >= 64rem) {
  [dir="rtl"] .rtl\:active\:focus\:lg\:underline:active:focus {
    -webkit-text-decoration-line: underline;
    text-decoration-line: underline;
  }
}

@media print {
  @media (width >= 80rem) {
    [dir="ltr"] .print\:ltr\:xl\:hover\:focus\:active\:text-yellow-300:hover:focus:active {
      color: oklch(90.5% .182 98.111);
    }
  }
}

@media (width >= 40rem) {
  .sm\:before\:target\:content-\[\'Hello_world\!\'\]::before:target {
    --en-content: 'Hello world!';
    content: var(--en-content);
  }
}

@media (width >= 40rem) {
  .sm\:hover\:bg-red-400:hover {
    background-color: oklch(70.4% .191 22.216);
  }
}

@media (width >= 48rem) {
  .md\:focus\:selection\:bg-blue-100:focus *::selection, .md\:focus\:selection\:bg-blue-100:focus::selection {
    background-color: oklch(93.2% .032 255.585);
  }
}

@media (width >= 96rem) {
  @media (prefers-reduced-motion: no-preference) {
    @media (orientation: landscape) {
      [dir="rtl"] .\32xl\:motion-safe\:landscape\:focus-within\:visited\:first\:odd\:checked\:open\:rtl\:bg-purple-100:focus-within:visited:first-child:nth-child(odd):checked[open] {
        background-color: oklch(94.6% .033 307.174);
      }
    }
  }
}

.group:hover .group-hover\:bg-green-300 {
  background-color: oklch(87.1% .15 154.449);
}

.group:focus .group-focus\:bg-green-400 {
  background-color: oklch(79.2% .209 151.711);
}

.peer:not(:invalid) ~ .peer-not-invalid\:bg-green-500 {
  background-color: oklch(72.3% .219 149.579);
}

.peer:invalid ~ .peer-invalid\:bg-red-500 {
  background-color: oklch(63.7% .237 25.331);
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_duplicated_selectors() {
        let generated = generate(["bg-red-500 bg-red-500", "bg-red-500"], &base_config());

        assert_eq!(
            generated,
            String::from(
                ".bg-red-500 {
  background-color: oklch(63.7% .237 25.331);
}"
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_arbitrary_property() {
        let generated = generate(["hover:[mask-type:luminance]"], &base_config());

        assert_eq!(
            generated,
            String::from(
                r".hover\:\[mask-type\:luminance\]:hover {
  mask-type: luminance;
}"
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_arbitrary_variant() {
        let generated = generate(
            [
                "[&_>_*]:before:content-[&#39;hello-&#39;]",
                "[&:has(.active)]:bg-blue-500",
                "[@supports_(display:grid)]:grid",
                "[@supports_not_(display:grid)]:float-right",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r"@supports not (display:grid) {
  .\[\@supports_not_\(display\:grid\)\]\:float-right {
    float: right;
  }
}

@supports (display:grid) {
  .\[\@supports_\(display\:grid\)\]\:grid {
    display: grid;
  }
}

.\[\&\:has\(\.active\)\]\:bg-blue-500:has(.active) {
  background-color: oklch(62.3% .214 259.815);
}

.\[\&_\>_\*\]\:before\:content-\[\'hello-\'\] > *::before {
  --en-content: 'hello-';
  content: var(--en-content);
}"
            )
        );
    }

    #[test]
    fn gen_css_for_variant_group() {
        let generated = generate(
            ["xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))"],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r"@media (width >= 80rem) {
  .xl\:\(focus\:\(outline\,outline-red-200\)\,dark\:\(bg-black\,text-white\)\):focus {
    outline-color: oklch(88.5% .062 18.334);
  }
}

@media (width >= 80rem) {
  .xl\:\(focus\:\(outline\,outline-red-200\)\,dark\:\(bg-black\,text-white\)\):focus {
    outline-width: 1px;
  }
}

@media (prefers-color-scheme: dark) {
  @media (width >= 80rem) {
    .xl\:\(focus\:\(outline\,outline-red-200\)\,dark\:\(bg-black\,text-white\)\) {
      color: #fff;
    }
  }
}

@media (prefers-color-scheme: dark) {
  @media (width >= 80rem) {
    .xl\:\(focus\:\(outline\,outline-red-200\)\,dark\:\(bg-black\,text-white\)\) {
      background-color: #000;
    }
  }
}"
            )
        );

        let generated = generate(["(bg-blue-100,bg-blue-200,bg-blue-300)"], &base_config());

        assert_eq!(
            generated,
            String::from(
                r".\(bg-blue-100\,bg-blue-200\,bg-blue-300\) {
  background-color: oklch(93.2% .032 255.585);
}

.\(bg-blue-100\,bg-blue-200\,bg-blue-300\) {
  background-color: oklch(88.2% .059 254.128);
}

.\(bg-blue-100\,bg-blue-200\,bg-blue-300\) {
  background-color: oklch(80.9% .105 251.813);
}"
            ),
        );
    }

    #[test]
    fn default_modifier_values_for_rounded() {
        let generated = generate(
            [
                "rounded-tr-sm rounded-tr-md rounded-sm rounded-md rounded-t-sm rounded-bl-xl border-x border border-4 border-t-2",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                ".rounded-md {
  border-radius: 0.375rem;
}

.rounded-sm {
  border-radius: 0.25rem;
}

.rounded-t-sm {
  border-top-left-radius: 0.25rem;
  border-top-right-radius: 0.25rem;
}

.rounded-tr-md {
  border-top-right-radius: 0.375rem;
}

.rounded-tr-sm {
  border-top-right-radius: 0.25rem;
}

.rounded-bl-xl {
  border-bottom-left-radius: 0.75rem;
}

.border {
  border-width: 1px;
}

.border-4 {
  border-width: 4px;
}

.border-x {
  border-inline-width: 1px;
}

.border-t-2 {
  border-top-width: 2px;
}"
            )
        );
    }

    #[test]
    fn gen_css_for_font_with_spaces() {
        let generated = generate(
            [
                "font-[&#39;Times_New_Roman&#39;,Helvetica,serif]",
                "font-[Roboto,&#39;Open_Sans&#39;,sans-serif]",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".font-\[\'Times_New_Roman\'\,Helvetica\,serif\] {
  font-family: 'Times New Roman',Helvetica,serif;
}

.font-\[Roboto\,\'Open_Sans\'\,sans-serif\] {
  font-family: Roboto,'Open Sans',sans-serif;
}"
            )
        );
    }

    #[test]
    fn gen_css_for_container() {
        let generated = generate(["container"], &base_config());

        assert_eq!(
            generated,
            String::from(
                ".container {
  width: 100%;
}

@media (width >= 40rem) {
  .container {
    max-width: 40rem;
  }
}

@media (width >= 48rem) {
  .container {
    max-width: 48rem;
  }
}

@media (width >= 64rem) {
  .container {
    max-width: 64rem;
  }
}

@media (width >= 80rem) {
  .container {
    max-width: 80rem;
  }
}

@media (width >= 96rem) {
  .container {
    max-width: 96rem;
  }
}"
            )
        );

        let generated = generate(["md:container", "md:mx-auto"], &base_config());

        assert_eq!(
            generated,
            String::from(
                r"@media (width >= 48rem) {
  .md\:mx-auto {
    margin-inline: auto;
  }
}

@media (width >= 48rem) {
  .md\:container {
    width: 100%;
  }
}

@media (width >= 48rem) {
  @media (width >= 40rem) {
    .md\:container {
      max-width: 40rem;
    }
  }

  @media (width >= 48rem) {
    .md\:container {
      max-width: 48rem;
    }
  }

  @media (width >= 64rem) {
    .md\:container {
      max-width: 64rem;
    }
  }

  @media (width >= 80rem) {
    .md\:container {
      max-width: 80rem;
    }
  }

  @media (width >= 96rem) {
    .md\:container {
      max-width: 96rem;
    }
  }
}"
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_before_after_variant() {
        let generated = generate(
            [
                "before:bg-red-500",
                "before:content-[&#39;Hello_world!&#39;]",
                "after:rounded-full",
                "after:content-[counter(foo)]",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".before\:content-\[\'Hello_world\!\'\]::before {
  --en-content: 'Hello world!';
  content: var(--en-content);
}

.before\:bg-red-500::before {
  background-color: oklch(63.7% .237 25.331);
  content: var(--en-content);
}

.after\:content-\[counter\(foo\)\]::after {
  --en-content: counter(foo);
  content: var(--en-content);
}

.after\:rounded-full::after {
  border-radius: 9999px;
  content: var(--en-content);
}"
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_dark_variant() {
        let generated = generate(["dark:mt-px"], &base_config());

        assert_eq!(
            generated,
            String::from(
                r"@media (prefers-color-scheme: dark) {
  .dark\:mt-px {
    margin-top: 1px;
  }
}"
            )
        );

        let mut config = base_config();
        config.theme.dark_mode = DarkMode::new_class(".dark");

        let generated = generate(["dark:mt-px"], &config);

        assert_eq!(
            generated,
            String::from(
                r".dark .dark\:mt-px {
  margin-top: 1px;
}"
            )
        );
    }

    #[test]
    fn variant_ordering() {
        let generated = generate(["*:first:text-green-400"], &base_config());

        assert_eq!(
            generated,
            String::from(
                r".\*\:first\:text-green-400 > *:first-child {
  color: oklch(79.2% .209 151.711);
}"
            )
        );

        let mut config = base_config();
        config.theme.dark_mode = DarkMode::new_class(".dark");

        let generated = generate(["dark:mt-px"], &config);

        assert_eq!(
            generated,
            String::from(
                r".dark .dark\:mt-px {
  margin-top: 1px;
}"
            )
        );
    }

    #[test]
    fn named_group_and_peer() {
        let generated = generate(
            [
                "group-checked/item:block peer-checked/item:block peer-not-checked/item:block",
                "peer-[:focus-within]/item:block",
                "peer-[:nth-of-type(3)_&]/item:block",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r":nth-of-type(3) .peer\/item ~ .peer-\[\:nth-of-type\(3\)_\&\]\/item\:block {
  display: block;
}

.peer\/item:focus-within ~ .peer-\[\:focus-within\]\/item\:block {
  display: block;
}

.group\/item:checked .group-checked\/item\:block {
  display: block;
}

.peer\/item:not(:checked) ~ .peer-not-checked\/item\:block {
  display: block;
}

.peer\/item:checked ~ .peer-checked\/item\:block {
  display: block;
}"
            )
        );
    }

    #[test]
    fn prefixed_variants() {
        let generated = generate(
            ["supports-[display:flex]:flex nth-of-type-[span]:text-red-500 data-[active]:block"],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".data-\[active\]\:block[data-active] {
  display: block;
}

.nth-of-type-\[span\]\:text-red-500:nth-of-type(span) {
  color: oklch(63.7% .237 25.331);
}

@supports (display:flex) {
  .supports-\[display\:flex\]\:flex {
    display: flex;
  }
}"
            )
        );
    }

    #[test]
    fn layers() {
        let mut config = base_config();
        config.layers.add("1", 1);
        config.layers.add("2", 2);
        config.layers.add("3", 3);
        config.layers.add("4", 4);

        let generated = generate(
            [
                "l-1:bg-red-500 l-2:bg-red-100 l-4:inset-12 l-1:(bg-blue-800,l-2:(bg-blue-700,bg-blue-600,l-3:bg-blue-500))",
            ],
            &config,
        );

        assert_eq!(
            generated,
            String::from(
                r".l-1\:\(bg-blue-800\,l-2\:\(bg-blue-700\,bg-blue-600\,l-3\:bg-blue-500\)\) {
  background-color: oklch(42.4% .199 265.638);
}

.l-1\:bg-red-500 {
  background-color: oklch(63.7% .237 25.331);
}

.l-1\:\(bg-blue-800\,l-2\:\(bg-blue-700\,bg-blue-600\,l-3\:bg-blue-500\)\) {
  background-color: oklch(54.6% .245 262.881);
}

.l-1\:\(bg-blue-800\,l-2\:\(bg-blue-700\,bg-blue-600\,l-3\:bg-blue-500\)\) {
  background-color: oklch(48.8% .243 264.376);
}

.l-2\:bg-red-100 {
  background-color: oklch(93.6% .032 17.717);
}

.l-1\:\(bg-blue-800\,l-2\:\(bg-blue-700\,bg-blue-600\,l-3\:bg-blue-500\)\) {
  background-color: oklch(62.3% .214 259.815);
}

.l-4\:inset-12 {
  inset: 3rem;
}"
            )
        );
    }

    #[test]
    fn arbitrary_values_test() {
        use std::fs;

        let file_content = fs::read_to_string("tests/fixtures/arbitrary-values.html").unwrap();
        let _generated = generate([file_content.as_str()], &base_config());
    }
}
