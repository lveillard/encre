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

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    generator::{ContextCanHandle, ContextHandle},
    selector::ArbitraryHint,
};

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

/// An alias to a [`Plugin`] which can easily be defined in Rust, e.g in const environments.
///
/// It requires using `&'static str` for all configuration. If you need to use `String`s for some
/// dynamic configuration, use [`DynamicPlugin`] instead.
pub type StaticPlugin = Plugin<
    &'static str,
    &'static [&'static str],
    phf::Map<&'static str, &'static str>,
    phf::Map<&'static str, &'static [&'static str]>,
    &'static [ArbitraryHint],
    &'static [PluginArbitraryMatcher<&'static str, &'static [&'static str]>],
>;

/// An alias to a [`Plugin`] which contain configuration defined using `String`s instead of static
/// string references, likely deserialized from a configuration file.
///
/// If you need to define a plugin using Rust without needing any heap-allocated `String`s, use
/// [`StaticPlugin`] instead.
pub type DynamicPlugin = Plugin<
    String,
    Vec<String>,
    HashMap<String, String>,
    HashMap<String, Vec<String>>,
    Vec<ArbitraryHint>,
    Vec<PluginArbitraryMatcher<String, Vec<String>>>,
>;

/// An alias to a [`PropertyName`] which is defined using `&'static str`, adapted for use in const
/// environments.
pub type StaticPropertyName = PropertyName<&'static str, &'static [&'static str]>;

/// An alias to a [`PropertyName`] which is defined using `String`, adapted for use when a name
/// needs to be dynamic or deserialized.
pub type DynamicPropertyName = PropertyName<String, Vec<String>>;

/// An alias to a [`PluginKind`] which is defined using `&'static str`, adapted for use in const
/// environments.
pub type StaticPluginKind = PluginKind<
    &'static str,
    &'static [&'static str],
    phf::Map<&'static str, &'static str>,
    phf::Map<&'static str, &'static [&'static str]>,
>;

/// An alias to a [`PluginKind`] which is defined using `String`, adapted for use when a kind
/// configuration needs to be dynamic or deserialized.
pub type DynamicPluginKind =
    PluginKind<String, Vec<String>, HashMap<String, String>, HashMap<String, Vec<String>>>;

/// An alias to a [`PluginArbitraryMatcher`] which is defined using `&'static str`, adapted for use in const
/// environments.
pub type StaticPluginArbitraryMatcher =
    PluginArbitraryMatcher<&'static str, &'static [&'static str]>;

/// An alias to a [`PluginArbitraryMatcher`] which is defined using `String`, adapted for use when a matcher
/// configuration needs to be dynamic or deserialized.
pub type DynamicPluginArbitraryMatcher = PluginArbitraryMatcher<String, Vec<String>>;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub(crate) enum CustomPlugin {
    #[serde(skip_serializing)]
    Static(&'static StaticPlugin),
    Dynamic(DynamicPlugin),
}

impl<'de> Deserialize<'de> for CustomPlugin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self::Dynamic(DynamicPlugin::deserialize(deserializer)?))
    }
}

/// When defining a [`PluginArbitraryMatcher`] for a [`PluginKind::Arbitrary`], defines how values
/// are separated.
///
/// A lot of CSS properties allow specifying several values of a single type separated by a
/// character, e.g `margin` allows [`<length>`](crate::utils::value_matchers::is_matching_length`)
/// or [`<percentage>`](crate::utils::value_matchers::is_matching_percentage`)
/// values separated by spaces, to define a specific margin for each side of the CSS layout box.
///
/// This enumeration helps matching these values when using a [`PluginArbitraryMatcher`], e.g for
/// the `margin` example, you would use
///
/// ```ignore
/// Plugin::new(...)
///     .matchers(&[Length, Percentage], PluginArbitraryMatcherSeparation::Space)
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum PluginArbitraryMatcherSeparation {
    /// No separation, a single value is matched.
    None,

    /// Separated by commas (`,`).
    ///
    /// Example: `33px, 42%, 6em`.
    Comma,

    /// Separated by spaces (` `).
    ///
    /// Example: `left top`.
    Space,

    /// Separated by commas (`,`) then spaces (` `).
    ///
    /// Example: `left, 12% 33px, right center`.
    Both,
}

/// Accepted inferred CSS arbitrary value types for a specific plugin.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub enum PluginArbitraryMatcher<Str, ArrayStr> {
    /// Match a [`shadow`](crate::utils::value_matchers::is_matching_shadow) CSS property value.
    Shadow,

    /// Match an [`absolute size`](crate::utils::value_matchers::is_matching_absolute_size) CSS property value.
    AbsoluteSize,

    /// Match a [`relative size`](crate::utils::value_matchers::is_matching_relative_size) CSS property value.
    RelativeSize,

    /// Match a [`line width`](crate::utils::value_matchers::is_matching_line_width`) CSS property value.
    LineWidth,

    /// Match a [`line style`](crate::utils::value_matchers::is_matching_line_style`) CSS property value.
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

    /// Match an [`<angle>`](crate::utils::value_matchers::is_matching_angle`) CSS property value.
    Angle,

    /// Match an [`<image>`](crate::utils::value_matchers::is_matching_image`) CSS property value.
    Image,

    /// Match a [`font family name`](crate::utils::value_matchers::is_matching_font_family_name`) CSS property value.
    FontFamilyName,

    /// Match a single custom value.
    Custom(Str),

    /// Match at least one value among a list of custom values.
    CustomMultiple(ArrayStr),
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyName<Str, ArrayStr> {
    SingleProp(Str),
    MultipleProps(ArrayStr),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum PluginKind<Str, ArrayStr, MapStr, MapArrayStr> {
    ListProperties {
        props: MapArrayStr,
    },
    ListValues {
        /// The CSS property name of the generated CSS rule.
        ///
        /// It can be a single property using [`PropertyName::SingleProp`] or a list of properties
        /// using [`PropertyName::MultipleProps`], in which case the value will be copied for all
        /// properties.
        prop: PropertyName<Str, ArrayStr>,
        values: MapStr,
    },

    Spacing {
        /// The namespace (i.e common prefix) that all classes need to start with in order to be
        /// matched by this plugin.
        namespace: Str,

        /// The CSS property name of the generated CSS rule.
        ///
        /// It can be a single property using [`PropertyName::SingleProp`] or a list of properties
        /// using [`PropertyName::MultipleProps`], in which case the value will be copied for all
        /// properties.
        prop: PropertyName<Str, ArrayStr>,
    },
    Color {
        /// The namespace (i.e common prefix) that all classes need to start with in order to be
        /// matched by this plugin.
        namespace: Str,

        /// The CSS property name of the generated CSS rule.
        ///
        /// It can be a single property using [`PropertyName::SingleProp`] or a list of properties
        /// using [`PropertyName::MultipleProps`], in which case the value will be copied for all
        /// properties.
        prop: PropertyName<Str, ArrayStr>,
    },
    Number {
        /// The namespace (i.e common prefix) that all classes need to start with in order to be
        /// matched by this plugin.
        namespace: Str,

        /// The CSS property name of the generated CSS rule.
        ///
        /// It can be a single property using [`PropertyName::SingleProp`] or a list of properties
        /// using [`PropertyName::MultipleProps`], in which case the value will be copied for all
        /// properties.
        prop: PropertyName<Str, ArrayStr>,
    },

    /// Define a plugin supporting [`arbitrary values`], i.e all selectors in the form
    /// `<namespace>-[...]`
    ///
    /// It directly copies the contents given inside brackets as the value of the `<prop>` CSS
    /// propertie(s).
    ///
    /// By default, all values are allowed by the plugin and it's up to the final user to only use
    /// valid CSS values for the property. However, if several [`PluginKind::Arbitrary`] plugins
    /// share the same namespace, it's *required* to disambiguate which plugins should handle the
    /// selector. In this case, [`Plugin::matchers`] and [`Plugin::hints`] should be used to
    /// only handle the selector if the arbitrary CSS value has a specific CSS type.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::{Config, generate};
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    ///     namespace: "mask",
    ///     prop: SingleProp("mask-position"),
    /// });
    ///
    /// let mut config = Config::default();
    /// config.register_plugin(&PLUGIN);
    ///
    /// let generated = generate(["mask-[25%]", "mask-[left_center]"], &config);
    ///
    /// assert!(generated.ends_with(r".mask-\[25\%\] {
    ///   mask-position: 25%;
    /// }
    ///
    /// .mask-\[left_center\] {
    ///   mask-position: left center;
    /// }"));
    /// ```
    ///
    /// [`arbitrary values`]: crate::selector
    Arbitrary {
        /// The namespace (i.e common prefix) that all classes need to start with in order to be
        /// matched by this plugin.
        namespace: Str,

        /// The CSS property name of the generated CSS rule.
        ///
        /// It can be a single property using [`PropertyName::SingleProp`] or a list of properties
        /// using [`PropertyName::MultipleProps`], in which case the value will be copied for all
        /// properties.
        prop: PropertyName<Str, ArrayStr>,
    },

    /// A powerful kind allowing the use a Rust function to handle all selectors in the form
    /// `<namespace>-...`.
    ///
    /// This plugin kind is (of course) not serializable.
    ///
    /// The [`can_handle`] field function takes a [`ContextCanHandle`] structure and returns whether
    /// the plugin is capable of handling the utility class given in the context.
    ///
    /// The [`handle`] field function takes a [`ContextHandle`] structure containing the modifier, the current
    /// configuration and a buffer containing the whole CSS currently generated. You can use the
    /// [`Buffer`] structure (especially the [`Buffer::line`] and [`Buffer::lines`] functions) to
    /// push CSS declarations to it, they will be automatically indented.
    ///
    /// [`generate_wrapper`] (and the more powerful [`generate_at_rules`] and [`generate_class`])
    /// should be called to generate the CSS rule wrapping.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::{Config, generate};
    /// use encre_css::prelude::build_plugin::*;
    /// use std::collections::HashMap;
    ///
    /// /// Reads the `emoji` extra field of the configuration to find the replacement emoji.
    /// fn extract_emoji_value<'a>(config: &'a Config, value: &str) -> Option<&'a str> {
    ///     config.extra.get("emoji")
    ///         .and_then(|r| r.as_table())
    ///         .and_then(|r| r.get(value))
    ///         .and_then(|r| r.as_str())
    /// }
    ///
    /// const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Functional {
    ///     namespace: "emoji",
    ///     can_handle: |context| matches!(context.modifier, Modifier::Builtin {
    ///         value,
    ///         ..
    ///     } if extract_emoji_value(context.config, value).is_some()),
    ///     handle: |context| {
    ///         // Only accept static modifiers, and dynamically fetch them from the
    ///         // `emoji` extra field of the configuration
    ///         if let Modifier::Builtin { value, .. } = context.modifier
    ///         && let Some(value) = extract_emoji_value(&context.config, value) {
    ///             generate_wrapper(context, |context| {
    ///                 context.buffer.line(format_args!("content: \"{value}\";"));
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
    /// [`can_handle`]: PluginKind::Functional::can_handle
    /// [`handle`]: PluginKind::Functional::handle
    /// [`generate_at_rules`]: crate::generator::generate_at_rules
    /// [`generate_class`]: crate::generator::generate_class
    /// [`generate_wrapper`]: crate::generator::generate_wrapper
    #[serde(skip)]
    Functional {
        /// The namespace (i.e common prefix) that all classes need to start with in order to be
        /// matched by this plugin.
        namespace: Str,

        /// A function returning whether a specific class (passed inside the context) is matched by
        /// this plugin.
        can_handle: fn(&ContextCanHandle) -> bool,

        /// A function called to generate the CSS of a matched class.
        ///
        /// It can use [`generate_wrapper`] (and the more powerful [`generate_at_rules`] and [`generate_class`])
        /// to generate the CSS rule wrapping.
        ///
        /// Various notes:
        ///
        /// - The CSS written should end with a newline
        /// - Arbitrary values are already normalized (e.g. underscores are replaced by spaces)
        /// - This function is guaranteed to be called only once per selector
        ///
        /// [`generate_wrapper`]: crate::generator::generate_wrapper
        /// [`generate_at_rules`]: crate::generator::generate_at_rules
        /// [`generate_class`]: crate::generator::generate_class
        handle: fn(&mut ContextHandle),
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
/// After you have defined a plugin, you need to register it in the [`Config`] structure by calling
/// [`Config::register_plugin`].
///
/// # Simple example (defines the static values of the `font-family` plugin)
///
/// ```
/// use encre_css::prelude::build_plugin::*;
///
/// const PLUGIN: StaticPlugin = Plugin::new(PluginKind::ListValues {
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
/// const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Number {
///     namespace: "stroke",
///     prop: SingleProp("stroke-width"),
/// })
/// .template("{}px");
///
/// const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
///     namespace: "stroke",
///     prop: SingleProp("stroke-width"),
/// })
/// .hints(&[ArbitraryHint::Length, ArbitraryHint::Percentage])
/// .matchers(&[
///     Length,
///     Percentage,
///     LineWidth,
///     Number,
/// ], PluginArbitraryMatcherModifier::CommaSeparated);
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
/// use encre_css::Config;
/// use encre_css::prelude::build_plugin::*;
///
/// /// Reads the `emoji` extra field of the configuration to find the replacement emoji.
/// fn extract_emoji_value<'a>(config: &'a Config, value: &str) -> Option<&'a str> {
///     config.extra.get("emoji")
///         .and_then(|r| r.as_table())
///         .and_then(|r| r.get(value))
///         .and_then(|r| r.as_str())
/// }
///
/// const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Functional {
///     namespace: "emoji",
///     can_handle: |context| matches!(context.modifier, Modifier::Builtin {
///         value,
///         ..
///     } if extract_emoji_value(context.config, value).is_some()),
///     handle: |context| {
///         // Only accept static modifiers, and dynamically fetch them from the
///         // `emoji` extra field of the configuration
///         if let Modifier::Builtin { value, .. } = context.modifier
///         && let Some(value) = extract_emoji_value(&context.config, value) {
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
/// ```
///
/// Have a look at <https://gitlab.com/encre-org/encre-css/tree/main/crates/encre-css/src/plugins>
/// for more examples.
///
/// # Define a plugin in TOML
///
/// All plugin kinds except [`PluginKind::Functional`] are serializable, thus can be defined in
/// `encre-css`'s TOML configuration (or every other language that uses a `serde` deserializer).
///
/// To do that, you need to add a new entry in the [`Config::custom_plugins`] list. You can use
/// every [`Plugin`] configuration option, just use the method name as a key.
///
/// ### Example
///
/// ```toml
/// [[custom_plugins]]
/// template = "{}px"
///
/// [custom_plugins.kind.Number]
/// namespace = "stroke"
/// prop = "stroke-width"
/// ```
///
/// # Quirks
///
/// - Only some [`Plugin`] methods are useful to use with a specific [`PluginKind`],
/// e.g [`Plugin::has_full`] has no sense with a [`PluginKind::ListValues`]. When defining plugins in a
/// Rust const environment, every methods of [`Plugin`] contains a compile-time check that the
/// method has a sense with the chosen [`PluginKind`], however, **this check does not happen when
/// declaring plugins in TOML and these options will silently be ignored**
/// - `encre-css` builds a [trie structure](https://en.wikipedia.org/wiki/Trie) based on the
/// namespace of the plugins to optimize matching a utility class to a specific plugin, so it's
/// **highly discouraged to leave the namespace of a plugin empty**, otherwise the performances will
/// decrease heavily.
///
/// [`Config::register_plugin`]: crate::Config::register_plugin
/// [`Config`]: crate::Config
/// [`Functional`]: PluginKind::Functional
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Plugin<Str, ArrayStr, MapStr, MapArrayStr, ArrayHints, ArrayMatchers> {
    pub(crate) kind: PluginKind<Str, ArrayStr, MapStr, MapArrayStr>,

    #[serde(default)]
    pub(crate) has_auto: bool,

    #[serde(default)]
    pub(crate) has_empty: bool,

    #[serde(default)]
    pub(crate) has_full: bool,

    #[serde(default)]
    pub(crate) has_negative: bool,

    pub(crate) extra_lines: Option<ArrayStr>,
    pub(crate) extra_css: Option<MapStr>,
    pub(crate) extra_class: Option<Str>,
    pub(crate) template: Option<Str>,
    pub(crate) template_multiple: Option<ArrayStr>,
    pub(crate) extra_slash: Option<(MapStr, Str)>,
    pub(crate) arbitrary_hints: Option<ArrayHints>,
    pub(crate) arbitrary_matchers: Option<(ArrayMatchers, PluginArbitraryMatcherSeparation)>,
    pub(crate) arbitrary_shadow_color_replacement: Option<Str>,
    pub(crate) namespace: Option<Str>,
    pub(crate) divide_by: Option<f32>,
}

impl StaticPlugin {
    /// Make a new [`Plugin`] from a [`PluginKind`] filled with the required values.
    ///
    /// It should be used from a const context, e.g
    ///
    /// ```ignore
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::new(...);
    /// ```
    #[must_use]
    pub const fn new(kind: StaticPluginKind) -> Self {
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
            arbitrary_matchers: None,
            arbitrary_shadow_color_replacement: None,
            namespace: None,
            divide_by: None,
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
    #[must_use]
    pub const fn has_auto(mut self) -> Self {
        assert!(
            matches!(
                self.kind,
                PluginKind::Spacing { .. } | PluginKind::Number { .. }
            ),
            "Plugin::has_auto only works with PluginKind::Spacing or PluginKind::Number"
        );

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
    #[must_use]
    pub const fn has_empty(mut self) -> Self {
        assert!(
            matches!(self.kind, PluginKind::Number { .. }),
            "Plugin::has_empty only works with PluginKind::Number"
        );

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
    #[must_use]
    pub const fn has_full(mut self) -> Self {
        assert!(
            matches!(self.kind, PluginKind::Spacing { .. }),
            "Plugin::has_full only works with PluginKind::Spacing"
        );

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
    #[must_use]
    pub const fn has_negative(mut self) -> Self {
        assert!(
            matches!(self.kind, PluginKind::Number { .. }),
            "Plugin::has_negative only works with PluginKind::Number"
        );

        self.has_negative = true;
        self
    }

    #[must_use]
    pub const fn extra_lines(mut self, extra_lines: &'static [&'static str]) -> Self {
        self.extra_lines = Some(extra_lines);
        self
    }

    #[must_use]
    pub const fn extra_css(mut self, extra_css: phf::Map<&'static str, &'static str>) -> Self {
        self.extra_css = Some(extra_css);
        self
    }

    #[must_use]
    pub const fn extra_class(mut self, extra_class: &'static str) -> Self {
        self.extra_class = Some(extra_class);
        self
    }

    #[must_use]
    pub const fn extra_slash(
        mut self,
        values: phf::Map<&'static str, &'static str>,
        default: &'static str,
    ) -> Self {
        assert!(
            matches!(
                self.kind,
                PluginKind::ListValues { .. }
                    | PluginKind::Number { .. }
                    | PluginKind::Spacing { .. }
                    | PluginKind::Color { .. }
            ),
            "Plugin::extra_slash only works with PluginKind::ListValues or PluginKind::Number or PluginKind::{{Sizing, Spacing, Color}}"
        );

        self.extra_slash = Some((values, default));
        self
    }

    #[must_use]
    pub const fn template(mut self, template: &'static str) -> Self {
        assert!(
            matches!(
                self.kind,
                PluginKind::Arbitrary { .. }
                    | PluginKind::Number { .. }
                    | PluginKind::Spacing { .. }
                    | PluginKind::Color { .. }
            ),
            "Plugin::template can only be used with PluginKind::Arbitrary or PluginKind::Number or PluginKind::{{Spacing, Sizing, Color}}"
        );

        self.template = Some(template);
        self
    }

    #[must_use]
    pub const fn template_multiple(mut self, templates: &'static [&'static str]) -> Self {
        assert!(
            matches!(
                self.kind,
                PluginKind::Arbitrary { .. }
                    | PluginKind::Number { .. }
                    | PluginKind::Spacing { .. }
                    | PluginKind::Color { .. }
            ),
            "Plugin::template_multiple can only be used with PluginKind::Arbitrary or PluginKind::Number or PluginKind::{{Spacing, Sizing, Color}}"
        );

        assert!(
            matches!(
                self.kind,
                PluginKind::Arbitrary {
                    prop: PropertyName::MultipleProps(..),
                    ..
                } | PluginKind::Number {
                    prop: PropertyName::MultipleProps(..),
                    ..
                } | PluginKind::Spacing {
                    prop: PropertyName::MultipleProps(..),
                    ..
                } | PluginKind::Color {
                    prop: PropertyName::MultipleProps(..),
                    ..
                }
            ),
            "Plugin::template can only be used with a MultipleProps property name. To define a template for a single property name, use Plugin::template"
        );

        assert!(
            matches!(
                self.kind,
                PluginKind::Arbitrary {
                    prop: PropertyName::MultipleProps(p),
                    ..
                } | PluginKind::Number {
                    prop: PropertyName::MultipleProps(p),
                    ..
                } | PluginKind::Spacing {
                    prop: PropertyName::MultipleProps(p),
                    ..
                } | PluginKind::Color {
                    prop: PropertyName::MultipleProps(p),
                    ..
                } if p.len() == templates.len()
            ),
            "Plugin::template_multiple should have as many elements as the number of properties defined in MultipleProps. Each template will be applied for the corresponding property name in the order they are defined"
        );

        self.template_multiple = Some(templates);
        self
    }

    #[must_use]
    pub const fn hints(mut self, hints: &'static [ArbitraryHint]) -> Self {
        assert!(
            matches!(self.kind, PluginKind::Arbitrary { .. }),
            "Plugin::hints can only be used with PluginKind::Arbitrary"
        );

        self.arbitrary_hints = Some(hints);
        self
    }

    // TODO(doc): complete example with several Arbitrary plugins sharing the same namespace
    #[must_use]
    pub const fn matchers(
        mut self,
        matchers: &'static [PluginArbitraryMatcher<&'static str, &'static [&'static str]>],
        modifier: PluginArbitraryMatcherSeparation,
    ) -> Self {
        assert!(
            matches!(self.kind, PluginKind::Arbitrary { .. }),
            "Plugin::matchers can only be used with PluginKind::Arbitrary"
        );

        self.arbitrary_matchers = Some((matchers, modifier));
        self
    }

    #[must_use]
    pub const fn shadow_color_replacement(mut self, replacement: &'static str) -> Self {
        assert!(
            matches!(self.kind, PluginKind::Arbitrary { .. }),
            "Plugin::shadow_color_replacement can only be used with PluginKind::Arbitrary"
        );

        self.arbitrary_shadow_color_replacement = Some(replacement);
        self
    }

    #[must_use]
    pub const fn namespace(mut self, namespace: &'static str) -> Self {
        assert!(
            matches!(
                self.kind,
                PluginKind::ListValues { .. } | PluginKind::ListProperties { .. }
            ),
            "Plugin::namespace can only be used with PluginKind::ListValues or PluginKind::ListProperties. For other kinds, use the built-in `namespace` field"
        );

        self.namespace = Some(namespace);
        self
    }

    #[must_use]
    pub const fn divide_by(mut self, factor: f32) -> Self {
        assert!(
            matches!(
                self.kind,
                PluginKind::Number { .. }
            ),
            "Plugin::divide_by can only be used with PluginKind::Number."
        );

        self.divide_by = Some(factor);
        self
    }
}

impl DynamicPlugin {
    /// Make a new [`Plugin`] from a [`PluginKind`] filled with the required values.
    ///
    /// It should be used from a non-const context, e.g
    ///
    /// ```ignore
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// fn main() {
    ///   let plugin: DynamicPlugin = Plugin::new_dynamic(...);
    /// }
    /// ```
    pub fn new_dynamic(kind: DynamicPluginKind) -> Self {
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
            arbitrary_matchers: None,
            arbitrary_shadow_color_replacement: None,
            namespace: None,
            divide_by: None,
        }
    }
}
