//! A [`Plugin`] is a handler used to convert utility classes into CSS declarations.
//!
//! A lot of plugins are built in (like the ones from Tailwind CSS) and some others live in
//! their own crates and need to be imported manually. They usually define a `register` function taking
//! a mutable reference to a [`Config`] structure.
//!
//! # Example (with `encre-css-typography`)
//!
//! ```ignore
//! use encre_css::Config;
//!
//! # fn main() -> encre_css::Result<()> {
//! let mut config = Config::from_file("encre-css.toml")?;
//! // Or let mut config = Config::default();
//!
//! encre_css_typography::register(&mut config);
//!
//! let _css = encre_css::generate(
//!     [r#"<div class="prose prose-headings:text-blue-500 prose-slate lg:prose-lg dark:prose-invert"></div>"#],
//!     &config,
//! );
//! // Do something with the CSS
//! # Ok(())
//! # }
//! ```
//!
//! # Official plugins
//!
//! - [`encre-css-typography`](https://gitlab.com/encre-org/encre-css/tree/main/crates/encre-css-typography): used to define beautiful typographic defaults for HTML you don't control.
//! - [`encre-css-icons`](https://gitlab.com/encre-org/encre-css/tree/main/crates/encre-css-icons): used to quickly add pure CSS icons to your website.
//!
//! If you want to write your own plugins, see [`Plugin`].
//!
//! [`Config`]: crate::Config

use serde::{Deserialize, Serialize};

use crate::{generator::Context, plugins::PropertyName::MultipleProps, selector::ArbitraryHint};

pub mod accessibility;
pub mod background;
pub mod border;
pub mod css_property;
pub mod effect;
pub mod filter;
pub mod flexbox;
pub mod grid;
pub mod interactivity;
pub mod layout;
pub mod sizing;
pub mod spacing;
pub mod svg;
pub mod table;
pub mod transform;
pub mod transition;
pub mod typography;

pub(crate) mod parsed;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum CustomPlugin {
    #[serde(skip)]
    Static(&'static Plugin),
    Parsed(parsed::ParsedPlugin),
}

/// Accepted inferred CSS arbitrary value types for a specific plugin.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PluginArbitraryMatcher {
    /// Match a [`shadow`](crate::utils::value_matchers::is_matching_shadow) CSS property value.
    Shadow,

    /// Match an [`absolute size`](crate::utils::value_matchers::is_matching_absolute_size) CSS property value.
    AbsoluteSize,

    /// Match an [`relative size`](crate::utils::value_matchers::is_matching_relative_size) CSS property value.
    RelativeSize,

    /// Match an [`line width`](crate::utils::value_matchers::is_matching_line_width`) CSS property value.
    LineWidth,

    /// Match an [`line style`](crate::utils::value_matchers::is_matching_line_style`) CSS property value.
    LineStyle,

    /// Match a [`<color>`](crate::utils::value_matchers::is_matching_color`) CSS property value.
    Color,

    /// Match a [`<length>`](crate::utils::value_matchers::is_matching_length`) CSS property value.
    Length,

    /// Match a [`<number>`](crate::utils::value_matchers::is_matching_number`) CSS property value.
    Number,

    /// Match a [`<percentage>`](crate::utils::value_matchers::is_matching_percentage`) CSS property value.
    Percentage,

    /// Match a [`<time>`](crate::utils::value_matchers::is_matching_time`) CSS property value.
    Time,

    /// Match a [`<gradient>`](crate::utils::value_matchers::is_matching_gradient`) CSS property value.
    Gradient,

    /// Match a [`<position>`](crate::utils::value_matchers::is_matching_position`) CSS property value.
    Position,

    /// Match a [`<angle>`](crate::utils::value_matchers::is_matching_angle`) CSS property value.
    Angle,

    /// Match a [`<image>`](crate::utils::value_matchers::is_matching_image`) CSS property value.
    Image,

    /// Match a [`font family name`](crate::utils::value_matchers::is_matching_font_family_name`) CSS property value.
    FontFamilyName,

    /// Match a single custom value.
    Custom(&'static str),

    /// Match at least one value among a list of custom values.
    CustomMultiple(&'static [&'static str]),

    /// Match a [`PluginArbitraryMatcher`] or another [`PluginArbitraryMatcher`].
    Or(
        &'static PluginArbitraryMatcher,
        &'static PluginArbitraryMatcher,
    ),

    /// Match at least one [`PluginArbitraryMatcher`] among a list of [`PluginArbitraryMatcher`].
    OrMultiple(&'static [&'static PluginArbitraryMatcher]),

    /// Match a list of comma-separated values (e.g `CommaSeparated(&Position)` matches `left, 100px, center`).
    CommaSeparated(&'static PluginArbitraryMatcher),

    /// Match a list of space-separated values (e.g `SpaceSeparated(&LineStyle)` matches `solid none dashed solid`).
    SpaceSeparated(&'static PluginArbitraryMatcher),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PropertyName {
    SingleProp(&'static str),
    MultipleProps(&'static [&'static str]),
}

#[derive(Debug, PartialEq)]
pub enum PluginKind {
    ListCases {
        cases: phf::Map<&'static str, &'static [&'static str]>,
    },
    ListValues {
        prop: PropertyName,
        values: phf::Map<&'static str, &'static str>,
    },

    Spacing {
        namespace: &'static str,
        prop: PropertyName,
    },
    Color {
        namespace: &'static str,
        prop: PropertyName,
    },
    Number {
        namespace: &'static str,
        prop: PropertyName,
        divide_by: f32,
    },

    // TODO(doc): by default every arbitrary value is accepted, used to disambiguate when multiple plugins of
    // the same namespace have arbitrary values
    Arbitrary {
        namespace: &'static str,
        prop: PropertyName,
    },

    /// A powerful kind allowing the use a Rust function to handle all selectors within a single
    /// namespace.
    ///
    /// This plugin kind is (of course) not serializable.
    ///
    /// The [`handle`] field takes a [`Context`] structure containing the modifier, the current
    /// configuration and a buffer containing the whole CSS currently generated. You can use the
    /// [`Buffer`] structure (especially the [`Buffer::line`] and [`Buffer::lines`] functions) to
    /// push CSS declarations to it, they will be automatically indented.
    ///
    /// [`generate_at_rules`] and [`generate_class`] should be called to generate the CSS rule
    /// structure.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::{Config, generate};
    /// use encre_css::prelude::build_plugin::*;
    /// use std::collections::HashMap;
    ///
    /// const PLUGIN: Plugin = Plugin::new(PluginKind::Functional {
    ///     namespace: "emoji",
    ///     handle: |context| {
    ///         // Only accept static modifiers, and dynamically fetch them from the
    ///         // `emoji` extra field of the configuration
    ///         if let Modifier::Builtin { value, .. } = context.modifier
    ///         && let Some(value) = context.config.extra.get("emoji")
    ///             .and_then(|r| r.as_table())
    ///             .and_then(|r| r.get(*value))
    ///             .and_then(|r| r.as_str()) {
    ///             generate_at_rules(context, |context| {
    ///                 generate_class(
    ///                     context,
    ///                     |context| {
    ///                         context.buffer.line(format_args!("content: \"{value}\";"));
    ///                     },
    ///                     "",
    ///                 );
    ///             });
    ///         }
    ///     },
    /// });
    ///
    /// let mut config = Config::default();
    /// config.extra.add(
    ///     "emoji",
    ///     HashMap::from_iter([("tada", "\u{1f389}"), ("rocket", "\u{1f680}")]),
    /// );
    /// config.register_plugin(&PLUGIN);
    ///
    /// let generated = generate(["emoji-tada", "emoji-rocket"], &config);
    /// dbg!(&generated);
    ///
    /// assert!(generated.ends_with(".emoji-rocket {
    ///   content: \"\u{1f680}\";
    /// }
    ///
    /// .emoji-tada {
    ///   content: \"\u{1f389}\";
    /// }"));
    /// ```
    ///
    /// [`Buffer`]: crate::utils::buffer::Buffer
    /// [`Buffer::line`]: crate::utils::buffer::Buffer::line
    /// [`Buffer::lines`]: crate::utils::buffer::Buffer::lines
    /// [`handle`]: PluginKind::Functional::handle
    /// [`generate_at_rules`]: crate::generator::generate_at_rules
    /// [`generate_class`]: crate::generator::generate_class
    Functional {
        namespace: &'static str,
        handle: fn(&mut Context),
    },
}

/// A plugin is a structure capable of generating CSS styles from a selector.
///
/// Several kinds of plugins exist and define what values are accepted as selector or modifier and
/// what CSS is generated based on the input selector. The API is designed to be fully declarative
/// (so that plugin declarations are serializable), except for the
/// [functional kind](PluginKind::Functional).
///
/// Each plugin kind has a set of required parameters which are defined in the [`PluginKind`]
/// enumeration, whereas the [`Plugin`] structure's methods allows overriding some default values
/// for the chosen kind.
///
/// It's common to define several plugins to handle a single utility class, and to define static
/// plugins as constants ([`Plugin::new`] as well as every [`Plugin`] methods are `const fn`s).
///
/// # Simple example (defines the static values of the `font-family` plugin)
///
/// ```
/// use encre_css::prelude::build_plugin::*;
///
/// const PLUGIN: Plugin = Plugin::new(PluginKind::ListValues {
///     prop: SingleProp("font-family"),
///     values: phf_map! {
///         "font-sans" => r#"ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont"#,
///         "font-serif" => r#"Georgia, Cambria, "Times New Roman", Times, serif"#,
///         "font-mono" => r#"Menlo, Monaco, Consolas, "Liberation Mono", monospace"#,
///     },
/// });
/// ```
///
/// # More advanced example (defines the `stroke-width` plugin)
///
/// ```
/// use encre_css::prelude::build_plugin::*;
/// use PluginArbitraryMatcher::*;
///
/// const PLUGIN: Plugin = Plugin::new(PluginKind::Number {
///     namespace: "stroke",
///     prop: SingleProp("stroke-width"),
///     divide_by: 1.0,
/// })
/// .template("{}px");
///
/// const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
///     namespace: "stroke",
///     prop: SingleProp("stroke-width"),
/// })
/// .hints(&[ArbitraryHint::Length, ArbitraryHint::Percentage])
/// .matcher(CommaSeparated(&OrMultiple(&[
///     &Length,
///     &Percentage,
///     &LineWidth,
///     &Number,
/// ])));
/// ```
///
/// # Release a plugin as a crate
///
/// If you want to release your custom plugins as a crate, you can export a `register` function
/// taking a mutable reference to a [`Config`] structure and use the [`Config::register_plugin`]
/// function to register them.
///
/// ```ignore
/// pub fn register(config: &mut Config) {
///     config.register_plugin(&PLUGIN);
///     config.register_plugin(&PLUGIN_ARBITRARY);
/// }
/// ```
///
/// # More powerful usage
///
/// If you need to have full control over the CSS **rule** generated, you can use the [`Functional`]
/// plugin kind. It allows executing a full-blown Rust function for each selector having a specific
/// namespace. However, it's (of course) not serializable, and thus cannot be used in, e.g a TOML
/// configuration.
///
/// ### Example
///
/// ```
/// use encre_css::prelude::build_plugin::*;
///
/// const PLUGIN: Plugin = Plugin::new(PluginKind::Functional {
///     namespace: "emoji",
///     handle: |context| {
///         if let Modifier::Builtin { value, .. } = context.modifier
///         && let Some(value) = context.config.extra.get("emoji")
///             .and_then(|r| r.as_table())
///             .and_then(|r| r.get(*value)) {
///             generate_at_rules(context, |context| {
///                 generate_class(
///                     context,
///                     |context| {
///                         context.buffer.line(format_args!("content: {value};"));
///                     },
///                     "",
///                 );
///             });
///         }
///     },
/// });
/// ```
///
/// Have a look at <https://gitlab.com/encre-org/encre-css/tree/main/crates/encre-css/src/plugins>
/// for more examples.
///
/// [`Config::register_plugin`]: crate::Config::register_plugin
/// [`Config`]: crate::Config
/// [`Functional`]: crate::plugins::PluginKind::Functional
/// [`generator::generate_at_rules`]: crate::generator::generate_at_rules
/// [`generator::generate_class`]: crate::generator::generate_class
#[derive(Debug, PartialEq)]
pub struct Plugin {
    pub(crate) kind: PluginKind,
    pub(crate) has_auto: bool,
    pub(crate) has_empty: bool,
    pub(crate) has_full: bool,
    pub(crate) has_negative: bool,
    pub(crate) extra_lines: Option<&'static [&'static str]>,
    pub(crate) extra_css: Option<phf::Map<&'static str, &'static str>>,
    pub(crate) extra_class: Option<&'static str>,
    pub(crate) template: Option<&'static str>,
    pub(crate) template_multiple: Option<&'static [&'static str]>,
    pub(crate) extra_slash: Option<(phf::Map<&'static str, &'static str>, &'static str)>,
    pub(crate) arbitrary_hints: Option<&'static [ArbitraryHint]>,
    pub(crate) arbitrary_matcher: Option<PluginArbitraryMatcher>,
    pub(crate) arbitrary_shadow_color_replacement: Option<&'static str>,
    pub(crate) list_namespace: Option<&'static str>,
}

impl Plugin {
    /// Make a new [`Plugin`] from a [`PluginKind`] filled with the required values.
    pub const fn new(kind: PluginKind) -> Self {
        Self {
            kind,
            has_auto: false,
            has_empty: false,
            has_full: false,
            has_negative: false,
            extra_lines: None,
            extra_css: None,
            extra_class: None,
            extra_slash: None,
            template: None,
            template_multiple: None,
            arbitrary_hints: None,
            arbitrary_matcher: None,
            arbitrary_shadow_color_replacement: None,
            list_namespace: None,
        }
    }

    /// Automatically add support for the `auto` modifier.
    ///
    /// If this method is called, an `auto` modifier will generate an `auto` CSS property value.
    ///
    /// <div class="warning">
    ///
    /// Only works with [`PluginKind::Spacing`] or [`PluginKind::Number`].
    ///
    /// </div>
    pub const fn has_auto(mut self) -> Self {
        if !matches!(
            self.kind,
            PluginKind::Spacing { .. } | PluginKind::Number { .. }
        ) {
            panic!("Plugin::has_auto only works with PluginKind::Spacing or PluginKind::Number");
        }

        self.has_auto = true;
        self
    }

    /// Automatically add support for an empty modifier.
    ///
    /// If this method is called, an empty modifier will generate a `1` CSS property value.
    ///
    /// <div class="warning">
    ///
    /// Only works with [`PluginKind::Number`].
    ///
    /// </div>
    pub const fn has_empty(mut self) -> Self {
        if !matches!(self.kind, PluginKind::Number { .. }) {
            panic!("Plugin::has_empty only works with PluginKind::Number");
        }

        self.has_empty = true;
        self
    }

    /// Automatically add support for the `full` modifier.
    ///
    /// If this method is called, a `full` modifier will generate a `100%` CSS property value.
    ///
    /// <div class="warning">
    ///
    /// Only works with [`PluginKind::Spacing`].
    ///
    /// </div>
    pub const fn has_full(mut self) -> Self {
        if !matches!(self.kind, PluginKind::Spacing { .. }) {
            panic!("Plugin::has_full only works with PluginKind::Spacing");
        }

        self.has_full = true;
        self
    }

    /// Automatically add support for negative modifiers.
    ///
    /// <div class="warning">
    ///
    /// Only works with [`PluginKind::Number`].
    ///
    /// </div>
    pub const fn has_negative(mut self) -> Self {
        if !matches!(self.kind, PluginKind::Number { .. }) {
            panic!("Plugin::has_negative only works with PluginKind::Number");
        }

        self.has_negative = true;
        self
    }

    pub const fn extra_lines(mut self, extra_lines: &'static [&'static str]) -> Self {
        self.extra_lines = Some(extra_lines);
        self
    }

    pub const fn extra_css(mut self, extra_css: phf::Map<&'static str, &'static str>) -> Self {
        self.extra_css = Some(extra_css);
        self
    }

    pub const fn extra_class(mut self, extra_class: &'static str) -> Self {
        self.extra_class = Some(extra_class);
        self
    }

    pub const fn extra_slash(
        mut self,
        values: phf::Map<&'static str, &'static str>,
        default: &'static str,
    ) -> Self {
        if !matches!(
            self.kind,
            PluginKind::ListValues { .. }
                | PluginKind::Number { .. }
                | PluginKind::Spacing { .. }
                | PluginKind::Color { .. }
        ) {
            panic!(
                "Plugin::extra_slash only works with PluginKind::ListValues or PluginKind::Number or PluginKind::{{Sizing, Spacing, Color}}"
            );
        }

        self.extra_slash = Some((values, default));
        self
    }

    pub const fn template(mut self, template: &'static str) -> Self {
        if !matches!(
            self.kind,
            PluginKind::Arbitrary { .. }
                | PluginKind::Number { .. }
                | PluginKind::Spacing { .. }
                | PluginKind::Color { .. }
        ) {
            panic!(
                "Plugin::template can only be used with PluginKind::Arbitrary or PluginKind::Number or PluginKind::{{Spacing, Sizing, Color}}"
            );
        }

        self.template = Some(template);
        self
    }

    pub const fn template_multiple(mut self, templates: &'static [&'static str]) -> Self {
        if !matches!(
            self.kind,
            PluginKind::Arbitrary { .. }
                | PluginKind::Number { .. }
                | PluginKind::Spacing { .. }
                | PluginKind::Color { .. }
        ) {
            panic!(
                "Plugin::template_multiple can only be used with PluginKind::Arbitrary or PluginKind::Number or PluginKind::{{Spacing, Sizing, Color}}"
            );
        }

        if !matches!(
            self.kind,
            PluginKind::Arbitrary {
                prop: MultipleProps(..),
                ..
            } | PluginKind::Number {
                prop: MultipleProps(..),
                ..
            } | PluginKind::Spacing {
                prop: MultipleProps(..),
                ..
            } | PluginKind::Color {
                prop: MultipleProps(..),
                ..
            }
        ) {
            panic!(
                "Plugin::template can only be used with a MultipleProps property name. To define a template for a single property name, use Plugin::template"
            );
        }

        if !matches!(
            self.kind,
            PluginKind::Arbitrary {
                prop: MultipleProps(p),
                ..
            } | PluginKind::Number {
                prop: MultipleProps(p),
                ..
            } | PluginKind::Spacing {
                prop: MultipleProps(p),
                ..
            } | PluginKind::Color {
                prop: MultipleProps(p),
                ..
            } if p.len() == templates.len()
        ) {
            panic!(
                "Plugin::template_multiple should have as many elements as the number of properties defined in MultipleProps. Each template will be applied for the corresponding property name in the order they are defined"
            );
        }

        self.template_multiple = Some(templates);
        self
    }

    pub const fn hints(mut self, hints: &'static [ArbitraryHint]) -> Self {
        if !matches!(self.kind, PluginKind::Arbitrary { .. }) {
            panic!("Plugin::hints can only be used with PluginKind::Arbitrary");
        }

        self.arbitrary_hints = Some(hints);
        self
    }

    pub const fn matcher(mut self, matcher: PluginArbitraryMatcher) -> Self {
        if !matches!(self.kind, PluginKind::Arbitrary { .. }) {
            panic!("Plugin::matches can only be used with PluginKind::Arbitrary");
        }

        self.arbitrary_matcher = Some(matcher);
        self
    }

    pub const fn shadow_color_replacement(mut self, replacement: &'static str) -> Self {
        if !matches!(self.kind, PluginKind::Arbitrary { .. }) {
            panic!("Plugin::shadow_color_replacement can only be used with PluginKind::Arbitrary");
        }

        self.arbitrary_shadow_color_replacement = Some(replacement);
        self
    }

    pub const fn list_namespace(mut self, list_namespace: &'static str) -> Self {
        if !matches!(
            self.kind,
            PluginKind::ListValues { .. } | PluginKind::ListCases { .. }
        ) {
            panic!(
                "Plugin::list_namespace can only be used with PluginKind::ListValues or PluginKind::ListCases. For other kinds, use the built-in `namespace` field"
            );
        }

        self.list_namespace = Some(list_namespace);
        self
    }
}
