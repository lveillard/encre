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

use crate::{generator::{ContextCanHandle, ContextHandle}, selector::CssType};

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
    &'static [CssType],
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
    Vec<CssType>,
>;

/// An alias to a [`PropertyName`] which is defined using `&'static str`, adapted for use in const
/// environments.
pub type StaticPropertyName = PropertyName<&'static str, &'static [&'static str]>;

/// An alias to a [`PropertyName`] which is defined using `String`, adapted for use when a name
/// needs to be dynamic or deserialized.
pub type DynamicPropertyName = PropertyName<String, Vec<String>>;

fn can_handle_nop(_: &ContextCanHandle) -> bool { false }
fn handle_nop(_: &mut ContextHandle) {}

#[derive(Debug, Clone, Serialize)]
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

/// Either a single or several CSS property names.
///
/// This enumeration is used when defining plugins to specify which CSS property name should be
/// generated. In the case of several property names (i.e [`MultipleProps`]), the
/// CSS value will be copied to all the properties.
///
/// When using [the `build_plugin` prelude](`crate::prelude::build_plugin`), the variants of this
/// enumeration are reexported so that you can simply write [`SingleProp`] and [`MultipleProps`]
/// without having to prefix them with `PropertyName::`.
///
/// When [defining a plugin using TOML](Plugin#define-a-plugin-in-toml), if you use a string, the
/// [`SingleProp`] variant will automatically be used, and if you use an array, the [`MultipleProps`]
/// variants will be used.
///
/// [`SingleProp`]: PropertyName::SingleProp
/// [`MultipleProps`]: PropertyName::MultipleProps
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyName<Str, ArrayStr> {
    /// A single CSS property name.
    SingleProp(Str),

    /// Several CSS property names in the order they will be generated.
    ///
    /// The CSS value defined by the plugin will be copied to each of the properties.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::{Config, generate};
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::Color(Color {
    ///     namespace: "custom-decoration",
    ///     prop: MultipleProps(&["-webkit-text-decoration-color", "text-decoration-color"]),
    ///     ..Color::default()
    /// });
    ///
    /// let mut config = Config::default();
    /// config.register_plugin(&PLUGIN);
    ///
    /// let generated = generate(["custom-decoration-red-200"], &config);
    ///
    /// assert!(generated.ends_with(r".custom-decoration-red-200 {
    ///   -webkit-text-decoration-color: oklch(88.5% .062 18.334);
    ///   text-decoration-color: oklch(88.5% .062 18.334);
    /// }"));
    /// ```
    MultipleProps(ArrayStr),
}

/// When defining a [`PluginArbitraryMatcher`] for an [`Arbitrary`] kind, defines how values
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
pub enum ArbitraryDisambiguateSeparation {
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

#[doc = include_str!("./doc_extra_slash.md")]
#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct ExtraSlash<Str, MapStr> {
    /// The mapping between the string parsed after the slash (`/`) and the actual values generated
    /// in the CSS value.
    pub values: MapStr,

    /// The key in [`ExtraSlash::values`] which is chosen by default when no slash is present in the
    /// utility class.
    pub default: Str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitraryDisambiguate<ArrayMatched> {
    pub matched: ArrayMatched,
    pub separation: ArbitraryDisambiguateSeparation,
}

/// Define a plugin using a map between utility classes and raw CSS lines.
///
/// It directly generates the CSS of the map value if the utility class as map key is scanned.
///
/// Map values are arrays which represent individual lines of the CSS so that each line can be
/// correctly indented.
///
/// If a utility class maps to an empty array, no class will be generated at all. This behavior can
/// be combined with [`ListProperties::extra_css`] to generate root-level CSS blocks (like
/// `@keyframe` animations).
///
/// If you instead need to map CSS property values to a single CSS property, use [`ListValues`].
///
/// ### Example
///
/// ```
/// use encre_css::{Config, generate};
/// use encre_css::prelude::build_plugin::*;
///
/// const PLUGIN: StaticPlugin = Plugin::ListProperties(ListProperties {
///     props: map! {
///         "overflow-visible" => &["overflow: visible;"],
///         "overflow-hidden" => &["overflow: hidden;"],
///         "overflow-clip" => &["overflow: clip;"],
///         "overflow-scroll" => &["overflow: scroll;"],
///         "overflow-auto" => &["overflow: auto;"],
///     },
///     ..ListProperties::default()
/// });
///
/// let mut config = Config::default();
/// config.register_plugin(&PLUGIN);
///
/// let generated = generate(["overflow-scroll"], &config);
///
/// assert!(generated.ends_with(r".overflow-scroll {
///   overflow: scroll;
/// }"));
/// ```
///
/// ### Example in TOML
///
/// ```toml
/// [[custom_plugins]]
///
/// [custom_plugins.ListProperties]
///
/// [custom_plugins.ListProperties.props]
/// overflow-visible = ["overflow: visible;"]
/// overflow-hidden = ["overflow: hidden;"]
/// overflow-clip = ["overflow: clip;"]
/// overflow-scroll = ["overflow: scroll;"]
/// overflow-auto = ["overflow: auto;"]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProperties<Str, ArrayStr, MapStr, MapArrayStr> {
    /// The map between utility classes and raw CSS lines.
    ///
    /// This field should be assigned separately after calling [`ListProperties::default`] (or
    /// [`ListProperties::default_dynamic`]).
    pub props: MapArrayStr,

    /// Define a [namespace](crate::selector) (i.e a prefix) common to all utility classes declared in the map keys.
    ///
    /// The last dash character (`-`) should be omitted due to the way the parsing of utility classes work
    /// (e.g in the example below, `overflow` is correct while `overflow-` is **incorrect**).
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::{Config, generate};
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::ListProperties(ListProperties {
    ///     namespace: Some("overflow"),
    ///     props: map! {
    ///         "visible" => &["overflow: visible;"],
    ///         "hidden" => &["overflow: hidden;"],
    ///         "clip" => &["overflow: clip;"],
    ///         "scroll" => &["overflow: scroll;"],
    ///         "auto" => &["overflow: auto;"],
    ///     },
    ///     ..ListProperties::default()
    /// });
    ///
    /// let mut config = Config::default();
    /// config.register_plugin(&PLUGIN);
    ///
    /// let generated = generate(["overflow-scroll"], &config);
    ///
    /// assert!(generated.ends_with(r".overflow-scroll {
    ///   overflow: scroll;
    /// }"));
    /// ```
    pub namespace: Option<Str>,

    #[doc = include_str!("./doc_extra_rule_css.md")]
    pub extra_rule_css: Option<ArrayStr>,

    #[doc = include_str!("./doc_extra_css.md")]
    pub extra_css: Option<MapStr>,

    #[doc = include_str!("./doc_extra_class.md")]
    pub extra_class: Option<Str>,
}

impl<Str, ArrayStr, MapStr> ListProperties<Str, ArrayStr, MapStr, phf::Map<&'static str, &'static [&'static str]>> {
    /// Make a default [`ListProperties`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`ListProperties::props`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`ListProperties::default_dynamic`] is that this function can only be used to
    /// build a plugin using static structures like `&[]`s, `&'static str`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::ListProperties(ListProperties {
    ///     props: map! {
    ///         "overflow-visible" => &["overflow: visible;"],
    ///         "overflow-hidden" => &["overflow: hidden;"],
    ///         "overflow-clip" => &["overflow: clip;"],
    ///         "overflow-scroll" => &["overflow: scroll;"],
    ///         "overflow-auto" => &["overflow: auto;"],
    ///     },
    ///     ..ListProperties::default()
    /// });
    /// ```
    pub const fn default() -> Self {
        Self {
            props: phf::Map::new(),
            namespace: None,
            extra_rule_css: None,
            extra_css: None,
            extra_class: None,
        }
    }
}

impl<Str, ArrayStr, MapStr> ListProperties<Str, ArrayStr, MapStr, HashMap<String, Vec<String>>> {
    /// Make a default [`ListProperties`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`ListProperties::props`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`ListProperties::default`] is that this function can only be used to
    /// build a plugin using heap-allocated structures like `String`s, `Vec`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// fn main() {
    ///     let props = HashMap::from_iter(
    ///         ["visible", "hidden", "clip", "scroll", "auto"].iter().map(|v| {
    ///             (format!("overflow-{v}"), vec![format!("overflow: {v};")])
    ///         })
    ///     );
    ///
    ///     // Note: the DynamicPlugin type hint is required to help the compiler
    ///     // find the concrete types of type parameters
    ///     let _plugin: DynamicPlugin = Plugin::ListProperties(ListProperties {
    ///         props,
    ///         ..ListProperties::default_dynamic()
    ///     });
    /// }
    /// ```
    ///
    /// This example is equivalent to the one of [`ListProperties::default`].
    pub fn default_dynamic() -> Self {
        Self {
            props: HashMap::new(),
            namespace: None,
            extra_rule_css: None,
            extra_css: None,
            extra_class: None,
        }
    }
}

/// Define a plugin using a map between utility classes and the values of a single CSS property.
///
/// If you instead need to generate several CSS properties or to have more control on the CSS lines
/// generated, use [`ListProperties`].
///
/// ### Example
///
/// ```
/// use encre_css::{Config, generate};
/// use encre_css::prelude::build_plugin::*;
///
/// const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
///     prop: SingleProp("width"),
///     values: map! {
///         "w-fit" => "fit-content",
///         "w-max" => "max-content",
///         "w-min" => "min-content",
///     },
///     ..ListValues::default()
/// });
///
/// let mut config = Config::default();
/// config.register_plugin(&PLUGIN);
///
/// let generated = generate(["w-max"], &config);
///
/// assert!(generated.ends_with(r".w-max {
///   width: max-content;
/// }"));
/// ```
///
/// ### Example in TOML
///
/// ```toml
/// [[custom_plugins]]
///
/// [custom_plugins.ListValues]
/// prop = "width"
///
/// [custom_plugins.ListValues.values]
/// w-fit = "fit-content"
/// w-max = "max-content"
/// w-min = "min-content"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListValues<Str, ArrayStr, MapStr> {
    /// The CSS property name of the generated CSS rule.
    ///
    /// It can be a single property using [`PropertyName::SingleProp`] or a list of properties
    /// using [`PropertyName::MultipleProps`], in which case the value will be copied for all
    /// properties.
    ///
    /// This field should be assigned separately after calling [`ListValues::default`] (or
    /// [`ListValues::default_dynamic`]).
    pub prop: PropertyName<Str, ArrayStr>,

    /// The map between utility classes and the CSS values of the property [`ListValues::prop`].
    ///
    /// This field should be assigned separately after calling [`ListValues::default`] (or
    /// [`ListValues::default_dynamic`]).
    pub values: MapStr,

    /// Define a [namespace](crate::selector) (i.e a prefix) common to all utility classes declared in the map keys.
    ///
    /// The last dash character (`-`) should be omitted due to the way the parsing of utility classes work
    /// (e.g in the example below, `overflow` is correct while `overflow-` is **incorrect**).
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::{Config, generate};
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    ///     namespace: Some("w"),
    ///     prop: SingleProp("width"),
    ///     values: map! {
    ///         "fit" => "fit-content",
    ///         "max" => "max-content",
    ///         "min" => "min-content",
    ///     },
    ///     ..ListValues::default()
    /// });
    ///
    /// let mut config = Config::default();
    /// config.register_plugin(&PLUGIN);
    ///
    /// let generated = generate(["w-max"], &config);
    ///
    /// assert!(generated.ends_with(r".w-max {
    ///   width: max-content;
    /// }"));
    /// ```
    pub namespace: Option<Str>,

    #[doc = include_str!("./doc_extra_rule_css.md")]
    pub extra_rule_css: Option<ArrayStr>,

    #[doc = include_str!("./doc_extra_css.md")]
    pub extra_css: Option<MapStr>,

    #[doc = include_str!("./doc_extra_class.md")]
    pub extra_class: Option<Str>,

    #[doc = include_str!("./doc_extra_slash.md")]
    pub extra_slash: Option<ExtraSlash<Str, MapStr>>,
}

impl<ArrayStr> ListValues<&'static str, ArrayStr, phf::Map<&'static str, &'static str>> {
    /// Make a default [`ListValues`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`ListValues::prop`] and [`ListValues::values`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`ListValues::default_dynamic`] is that this function can only be used to
    /// build a plugin using static structures like `&[]`s, `&'static str`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    ///     prop: SingleProp("width"),
    ///     values: map! {
    ///         "w-fit" => "fit-content",
    ///         "w-max" => "max-content",
    ///         "w-min" => "min-content",
    ///     },
    ///     ..ListValues::default()
    /// });
    /// ```
    pub const fn default() -> Self {
        Self {
            prop: PropertyName::SingleProp(""),
            values: phf::Map::new(),
            namespace: None,
            extra_rule_css: None,
            extra_css: None,
            extra_class: None,
            extra_slash: None,
        }
    }
}

impl<ArrayStr> ListValues<String, ArrayStr, HashMap<String, String>> {
    /// Make a default [`ListValues`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`ListValues::prop`] and [`ListValues::values`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`ListProperties::default`] is that this function can only be used to
    /// build a plugin using heap-allocated structures like `String`s, `Vec`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// fn main() {
    ///     let values = HashMap::from_iter(
    ///         ["fit", "max", "min"].iter().map(|v| {
    ///             (format!("w-{v}"), format!("width: {v}-content;"))
    ///         })
    ///     );
    ///
    ///     // Note: the DynamicPlugin type hint is required to help the compiler
    ///     // find the concrete types of type parameters
    ///     let _plugin: DynamicPlugin = Plugin::ListValues(ListValues {
    ///         prop: SingleProp("width".to_string()),
    ///         values,
    ///         ..ListValues::default_dynamic()
    ///     });
    /// }
    /// ```
    ///
    /// This example is equivalent to the one of [`ListValues::default`].
    pub fn default_dynamic() -> Self {
        Self {
            prop: PropertyName::SingleProp(String::new()),
            values: HashMap::new(),
            namespace: None,
            extra_rule_css: None,
            extra_css: None,
            extra_class: None,
            extra_slash: None,
        }
    }
}

/// Define a plugin which supports all spacing modifiers,
/// that is a (potentially floating) number, a fraction (e.g `3/4`) or `px`.
///
/// This plugin kind can also support the `auto` and `full` modifiers by setting [`Spacing::has_auto`] and [`Spacing::has_full`].
///
/// ### Example
///
/// ```
/// use encre_css::{Config, generate};
/// use encre_css::prelude::build_plugin::*;
///
/// const PLUGIN: StaticPlugin = Plugin::Spacing(Spacing {
///     namespace: "h",
///     prop: SingleProp("height"),
///     has_auto: Some(true),
///     has_full: Some(true),
///     ..Spacing::default()
/// });
///
/// let mut config = Config::default();
/// config.register_plugin(&PLUGIN);
///
/// let generated = generate(["h-2", "h-3/4", "h-px", "h-auto", "h-full"], &config);
///
/// assert!(generated.ends_with(r".h-2 {
///   height: 0.5rem;
/// }
///
/// .h-3\/4 {
///   height: 75%;
/// }
///
/// .h-auto {
///   height: auto;
/// }
///
/// .h-full {
///   height: 100%;
/// }
///
/// .h-px {
///   height: 1px;
/// }"));
/// ```
///
/// ### Example in TOML
///
/// ```toml
/// [[custom_plugins]]
///
/// [custom_plugins.Spacing]
/// namespace = "h"
/// prop = "height"
/// has_auto = true
/// has_full = true
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spacing<Str, ArrayStr, MapStr> {
    /// The namespace (i.e common prefix) that all classes need to start with in order to be
    /// matched by this plugin.
    pub namespace: Str,

    /// The CSS property name of the generated CSS rule.
    ///
    /// It can be a single property using [`PropertyName::SingleProp`] or a list of properties
    /// using [`PropertyName::MultipleProps`], in which case the value will be copied for all
    /// properties.
    pub prop: PropertyName<Str, ArrayStr>,

    /// Automatically add support for the `auto` modifier.
    ///
    /// If this method is called, an `auto` modifier will generate an `auto` CSS property value.
    pub has_auto: Option<bool>,

    /// Automatically add support for the `full` modifier.
    ///
    /// If this method is called, a `full` modifier will generate a `100%` CSS property value.
    pub has_full: Option<bool>,

    #[doc = include_str!("./doc_template.md")]
    pub template: Option<PropertyName<Str, ArrayStr>>,

    #[doc = include_str!("./doc_extra_rule_css.md")]
    pub extra_rule_css: Option<ArrayStr>,

    #[doc = include_str!("./doc_extra_css.md")]
    pub extra_css: Option<MapStr>,

    #[doc = include_str!("./doc_extra_class.md")]
    pub extra_class: Option<Str>,

    #[doc = include_str!("./doc_extra_slash.md")]
    pub extra_slash: Option<ExtraSlash<Str, MapStr>>,
}

impl<ArrayStr, MapStr> Spacing<&'static str, ArrayStr, MapStr> {
    /// Make a default [`Spacing`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`Spacing::namespace`] and [`Spacing::prop`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`Spacing::default_dynamic`] is that this function can only be used to
    /// build a plugin using static structures like `&[]`s, `&'static str`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::Spacing(Spacing {
    ///     namespace: "h",
    ///     prop: SingleProp("height"),
    ///     ..Spacing::default()
    /// });
    /// ```
    pub const fn default() -> Self {
        Self {
            namespace: "",
            prop: PropertyName::SingleProp(""),
            has_auto: None,
            has_full: None,
            template: None,
            extra_rule_css: None,
            extra_css: None,
            extra_class: None,
            extra_slash: None,
        }
    }
}

impl<ArrayStr, MapStr> Spacing<String, ArrayStr, MapStr> {
    /// Make a default [`Spacing`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`Spacing::namespace`] and [`Spacing::prop`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`Spacing::default`] is that this function can only be used to
    /// build a plugin using heap-allocated structures like `String`s, `Vec`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// fn main() {
    ///     // Note: the DynamicPlugin type hint is required to help the compiler
    ///     // find the concrete types of type parameters
    ///     let _plugin: DynamicPlugin = Plugin::Spacing(Spacing {
    ///         namespace: "h".to_string(),
    ///         prop: SingleProp("height".to_string()),
    ///         ..Spacing::default_dynamic()
    ///     });
    /// }
    /// ```
    ///
    /// This example is equivalent to the one of [`Spacing::default`].
    pub fn default_dynamic() -> Self {
        Self {
            namespace: String::new(),
            prop: PropertyName::SingleProp(String::new()),
            has_auto: None,
            has_full: None,
            template: None,
            extra_rule_css: None,
            extra_css: None,
            extra_class: None,
            extra_slash: None,
        }
    }
}

/// Define a plugin which supports all color modifiers, e.g `red-200` (the list of colors is based
/// on [`BUILTIN_COLORS`] and [`Theme::colors`] which is defined by the [`Config`]).
///
/// [`BUILTIN_COLORS`]: crate::config::BUILTIN_COLORS
/// [`Theme::colors`]: crate::config::Theme::colors
/// [`Config`]: crate::config::Config
///
/// ### Example
///
/// ```
/// use encre_css::{Config, generate};
/// use encre_css::prelude::build_plugin::*;
///
/// const PLUGIN: StaticPlugin = Plugin::Color(Color {
///     namespace: "bg",
///     prop: SingleProp("background-color"),
///     ..Color::default()
/// });
///
/// let mut config = Config::default();
/// config.register_plugin(&PLUGIN);
///
/// let generated = generate(["bg-red-200", "bg-black", "bg-inherit"], &config);
///
/// assert!(generated.ends_with(r".bg-black {
///   background-color: #000;
/// }
///
/// .bg-inherit {
///   background-color: inherit;
/// }
///
/// .bg-red-200 {
///   background-color: oklch(88.5% .062 18.334);
/// }"));
/// ```
///
/// ### Example in TOML
///
/// ```toml
/// [[custom_plugins]]
///
/// [custom_plugins.Color]
/// namespace = "bg"
/// prop = "background-color"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color<Str, ArrayStr, MapStr> {
    /// The namespace (i.e common prefix) that all classes need to start with in order to be
    /// matched by this plugin.
    pub namespace: Str,

    /// The CSS property name of the generated CSS rule.
    ///
    /// It can be a single property using [`PropertyName::SingleProp`] or a list of properties
    /// using [`PropertyName::MultipleProps`], in which case the value will be copied for all
    /// properties.
    pub prop: PropertyName<Str, ArrayStr>,

    #[doc = include_str!("./doc_template.md")]
    pub template: Option<PropertyName<Str, ArrayStr>>,

    #[doc = include_str!("./doc_extra_rule_css.md")]
    pub extra_rule_css: Option<ArrayStr>,

    #[doc = include_str!("./doc_extra_css.md")]
    pub extra_css: Option<MapStr>,

    #[doc = include_str!("./doc_extra_class.md")]
    pub extra_class: Option<Str>,
}

impl<ArrayStr, MapStr> Color<&'static str, ArrayStr, MapStr> {
    /// Make a default [`Color`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`Color::namespace`] and [`Color::prop`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`Color::default_dynamic`] is that this function can only be used to
    /// build a plugin using static structures like `&[]`s, `&'static str`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::Color(Color {
    ///     namespace: "bg",
    ///     prop: SingleProp("background-color"),
    ///     ..Color::default()
    /// });
    /// ```
    pub const fn default() -> Self {
        Self {
            namespace: "",
            prop: PropertyName::SingleProp(""),
            template: None,
            extra_rule_css: None,
            extra_css: None,
            extra_class: None,
        }
    }
}

impl<ArrayStr, MapStr> Color<String, ArrayStr, MapStr> {
    /// Make a default [`Color`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`Color::namespace`] and [`Color::prop`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`Color::default`] is that this function can only be used to
    /// build a plugin using heap-allocated structures like `String`s, `Vec`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// fn main() {
    ///     // Note: the DynamicPlugin type hint is required to help the compiler
    ///     // find the concrete types of type parameters
    ///     let _plugin: DynamicPlugin = Plugin::Color(Color {
    ///         namespace: "bg".to_string(),
    ///         prop: SingleProp("background-color".to_string()),
    ///         ..Color::default_dynamic()
    ///     });
    /// }
    /// ```
    ///
    /// This example is equivalent to the one of [`Color::default`].
    pub fn default_dynamic() -> Self {
        Self {
            namespace: String::new(),
            prop: PropertyName::SingleProp(String::new()),
            template: None,
            extra_rule_css: None,
            extra_css: None,
            extra_class: None,
        }
    }
}

/// Define a plugin which supports any number as modifier.
///
/// The number must be an integer (signed integers can be supported by enabling
/// [`Number::has_negative`]).
///
/// ### Example
///
/// ```
/// use encre_css::{Config, generate};
/// use encre_css::prelude::build_plugin::*;
///
/// const PLUGIN: StaticPlugin = Plugin::Number(Number {
///    namespace: "z",
///    prop: SingleProp("z-index"),
///    has_negative: Some(true),
///    has_auto: Some(true),
///    ..Number::default()
/// });
///
/// let mut config = Config::default();
/// config.register_plugin(&PLUGIN);
///
/// let generated = generate(["z-20", "-z-5", "z-auto"], &config);
///
/// assert!(generated.ends_with(r".-z-5 {
///   z-index: -5;
/// }
///
/// .z-20 {
///   z-index: 20;
/// }
///
/// .z-auto {
///   z-index: auto;
/// }"));
/// ```
///
/// ### Example in TOML
///
/// ```toml
/// [[custom_plugins]]
///
/// [custom_plugins.Number]
/// namespace = "z"
/// prop = "z-index"
/// has_negative = true
/// has_auto = true
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Number<Str, ArrayStr, MapStr> {
    /// The namespace (i.e common prefix) that all classes need to start with in order to be
    /// matched by this plugin.
    pub namespace: Str,

    /// The CSS property name of the generated CSS rule.
    ///
    /// It can be a single property using [`PropertyName::SingleProp`] or a list of properties
    /// using [`PropertyName::MultipleProps`], in which case the value will be copied for all
    /// properties.
    pub prop: PropertyName<Str, ArrayStr>,

    /// A float by which to divide the number given in the utility class.
    ///
    /// It can for example be used to support classes having a percentage between 1-100 but which
    /// need to generate a CSS property value between 0-1.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::{Config, generate};
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::Number(Number {
    ///     namespace: "custom-opacity",
    ///     prop: SingleProp("opacity"),
    ///     divide_by: Some(100.0),
    ///     ..Number::default()
    /// });
    ///
    /// let mut config = Config::default();
    /// config.register_plugin(&PLUGIN);
    ///
    /// let generated = generate(["custom-opacity-80", "custom-opacity-2"], &config);
    ///
    /// assert!(generated.ends_with(r".custom-opacity-2 {
    ///   opacity: 0.02;
    /// }
    ///
    /// .custom-opacity-80 {
    ///   opacity: 0.8;
    /// }"));
    /// ```
    pub divide_by: Option<f32>,

    /// Automatically add support for the `auto` modifier.
    ///
    /// If this method is called, an `auto` modifier will generate an `auto` CSS property value.
    pub has_auto: Option<bool>,

    /// Automatically add support for an empty modifier.
    ///
    /// If this method is called, an empty modifier will generate a `1` CSS property value.
    pub has_empty: Option<bool>,

    /// Automatically add support for negative modifiers.
    pub has_negative: Option<bool>,

    #[doc = include_str!("./doc_template.md")]
    pub template: Option<PropertyName<Str, ArrayStr>>,

    #[doc = include_str!("./doc_extra_rule_css.md")]
    pub extra_rule_css: Option<ArrayStr>,

    #[doc = include_str!("./doc_extra_css.md")]
    pub extra_css: Option<MapStr>,

    #[doc = include_str!("./doc_extra_class.md")]
    pub extra_class: Option<Str>,

    #[doc = include_str!("./doc_extra_slash.md")]
    pub extra_slash: Option<ExtraSlash<Str, MapStr>>,
}

impl<ArrayStr, MapStr> Number<&'static str, ArrayStr, MapStr> {
    /// Make a default [`Number`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`Number::namespace`] and [`Number::prop`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`Number::default_dynamic`] is that this function can only be used to
    /// build a plugin using static structures like `&[]`s, `&'static str`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::Number(Number {
    ///     namespace: "z",
    ///     prop: SingleProp("z-index"),
    ///     ..Number::default()
    /// });
    /// ```
    pub const fn default() -> Self {
        Self {
            namespace: "",
            prop: PropertyName::SingleProp(""),
            divide_by: None,
            has_auto: None,
            has_empty: None,
            has_negative: None,
            template: None,
            extra_rule_css: None,
            extra_css: None,
            extra_class: None,
            extra_slash: None,
        }
    }
}

impl<ArrayStr, MapStr> Number<String, ArrayStr, MapStr> {
    /// Make a default [`Number`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`Number::namespace`] and [`Number::prop`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`Number::default`] is that this function can only be used to
    /// build a plugin using heap-allocated structures like `String`s, `Vec`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// fn main() {
    ///     // Note: the DynamicPlugin type hint is required to help the compiler
    ///     // find the concrete types of type parameters
    ///     let _plugin: DynamicPlugin = Plugin::Number(Number {
    ///         namespace: "z".to_string(),
    ///         prop: SingleProp("z-index".to_string()),
    ///         ..Number::default_dynamic()
    ///     });
    /// }
    /// ```
    ///
    /// This example is equivalent to the one of [`Number::default`].
    pub fn default_dynamic() -> Self {
        Self {
            namespace: String::new(),
            prop: PropertyName::SingleProp(String::new()),
            divide_by: None,
            has_auto: None,
            has_empty: None,
            has_negative: None,
            template: None,
            extra_rule_css: None,
            extra_css: None,
            extra_class: None,
            extra_slash: None,
        }
    }
}

/// Define a plugin supporting [`arbitrary values`], i.e all selectors in the form
/// `<namespace>-[...]` (i.e the modifier is wrapped in square brackets).
///
/// It directly copies the contents given inside brackets as the value of the `<prop>` CSS
/// propertie(s).
///
/// By default, all values are allowed by the plugin and it's up to the final user to only use
/// valid CSS values for the property. However, if several [`Arbitrary`] plugins
/// share the same namespace, it's *required* to disambiguate which plugins should handle the
/// selector. In this case, [`Arbitrary::disambiguate`] should be used to
/// only handle the selector if the arbitrary CSS value has a specific CSS type or a specific manual hint.
///
/// ### Example
///
/// ```
/// use encre_css::{Config, generate};
/// use encre_css::prelude::build_plugin::*;
///
/// const PLUGIN: StaticPlugin = Plugin::Arbitrary(Arbitrary {
///     namespace: "mask",
///     prop: SingleProp("mask-position"),
///     ..Arbitrary::default()
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
/// ### Example in TOML
///
/// ```toml
/// [[custom_plugins]]
///
/// [custom_plugins.Arbitrary]
/// namespace = "mask"
/// prop = "mask-position"
/// ```
///
/// [`arbitrary values`]: crate::selector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arbitrary<Str, ArrayStr, MapStr, ArrayMatched> {
    /// The namespace (i.e common prefix) that all classes need to start with in order to be
    /// matched by this plugin.
    pub namespace: Str,

    /// The CSS property name of the generated CSS rule.
    ///
    /// It can be a single property using [`PropertyName::SingleProp`] or a list of properties
    /// using [`PropertyName::MultipleProps`], in which case the value will be copied for all
    /// properties.
    pub prop: PropertyName<Str, ArrayStr>,

    /// If the arbitrary value is a shadow, replace all the colors used by a single CSS variable
    /// given as string.
    ///
    /// This field should only be used for shadows that need to have their colors separately set
    /// using a dedicated utility class.
    ///
    /// If the value contains a placeholder `{}`, it will be replaced by the previous color value.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::{Config, generate};
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN_SHADOW: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    ///     namespace: "custom-shadow",
    ///     prop: SingleProp("box-shadow"),
    ///     shadow_color_replacement: Some("var(--shadow-color, {})"),
    ///     ..Arbitrary::default()
    /// });
    ///
    /// const PLUGIN_SHADOW_COLOR: StaticPlugin = Plugin::Color(Color {
    ///     namespace: "custom-shadow-color",
    ///     prop: SingleProp("--shadow-color"),
    ///     ..Color::default()
    /// });
    ///
    /// let mut config = Config::default();
    /// config.register_plugin(&PLUGIN_SHADOW);
    /// config.register_plugin(&PLUGIN_SHADOW_COLOR);
    ///
    /// let generated = generate(["custom-shadow-[10px_5px_5px_red]", "custom-shadow-color-blue-100"], &config);
    ///
    /// assert!(generated.ends_with(r"
    /// .custom-shadow-\[10px_5px_5px_red\] {
    ///   box-shadow: 10px 5px 5px var(--shadow-color, red);
    /// }
    ///
    /// .custom-shadow-color-blue-100 {
    ///   --shadow-color: oklch(93.2% .032 255.585);
    /// }"));
    /// ```
    ///
    ///
    pub shadow_color_replacement: Option<Str>,

    /// See [`ArbitraryDisambiguate`].
    pub disambiguate: Option<ArbitraryDisambiguate<ArrayMatched>>,

    #[doc = include_str!("./doc_template.md")]
    pub template: Option<PropertyName<Str, ArrayStr>>,

    #[doc = include_str!("./doc_extra_rule_css.md")]
    pub extra_rule_css: Option<ArrayStr>,

    #[doc = include_str!("./doc_extra_css.md")]
    pub extra_css: Option<MapStr>,

    #[doc = include_str!("./doc_extra_class.md")]
    pub extra_class: Option<Str>,
}

impl<ArrayStr, MapStr, ArrayMatched> Arbitrary<&'static str, ArrayStr, MapStr, ArrayMatched> {
    /// Make a default [`Arbitrary`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`Arbitrary::namespace`] and [`Arbitrary::prop`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`Arbitrary::default_dynamic`] is that this function can only be used to
    /// build a plugin using static structures like `&[]`s, `&'static str`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    ///     namespace: "gap",
    ///     prop: SingleProp("gap"),
    ///     ..Arbitrary::default()
    /// });
    /// ```
    pub const fn default() -> Self {
        Self {
            namespace: "",
            prop: PropertyName::SingleProp(""),
            shadow_color_replacement: None,
            disambiguate: None,
            template: None,
            extra_rule_css: None,
            extra_css: None,
            extra_class: None,

        }
    }
}

impl<ArrayStr, MapStr, ArrayMatched> Arbitrary<String, ArrayStr, MapStr, ArrayMatched> {
    /// Make a default [`Arbitrary`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`Arbitrary::namespace`] and [`Arbitrary::prop`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`Arbitrary::default`] is that this function can only be used to
    /// build a plugin using heap-allocated structures like `String`s, `Vec`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// fn main() {
    ///     // Note: the DynamicPlugin type hint is required to help the compiler
    ///     // find the concrete types of type parameters
    ///     let _plugin: DynamicPlugin = Plugin::Arbitrary(Arbitrary {
    ///         namespace: "gap".to_string(),
    ///         prop: SingleProp("gap".to_string()),
    ///         ..Arbitrary::default_dynamic()
    ///     });
    /// }
    /// ```
    ///
    /// This example is equivalent to the one of [`Arbitrary::default`].
    pub fn default_dynamic() -> Self {
        Self {
            namespace: String::new(),
            prop: PropertyName::SingleProp(String::new()),
            shadow_color_replacement: None,
            disambiguate: None,
            template: None,
            extra_rule_css: None,
            extra_css: None,
            extra_class: None,
        }
    }
}

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
/// const PLUGIN: StaticPlugin = Plugin::Functional(Functional {
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
/// [`can_handle`]: Functional::can_handle
/// [`handle`]: Functional::handle
/// [`generate_at_rules`]: crate::generator::generate_at_rules
/// [`generate_class`]: crate::generator::generate_class
/// [`generate_wrapper`]: crate::generator::generate_wrapper
#[derive(Debug, Clone)]
pub struct Functional<Str> {
    /// The namespace (i.e common prefix) that all classes need to start with in order to be
    /// matched by this plugin.
    pub namespace: Str,

    /// A function returning whether a specific class (passed inside the context) is matched by
    /// this plugin.
    pub can_handle: fn(&ContextCanHandle) -> bool,

    /// A function called to generate the CSS of a matched class.
    ///
    /// It should use [`generate_wrapper`] (and the more powerful [`generate_at_rules`] and [`generate_class`])
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
    pub handle: fn(&mut ContextHandle),
}

impl Functional<&'static str> {
    /// Make a default [`Functional`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`Functional::namespace`], [`Functional::can_handle`] and [`Functional::handle`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`Functional::default_dynamic`] is that this function can only be used to
    /// build a plugin using static structures like `&[]`s, `&'static str`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// const PLUGIN: StaticPlugin = Plugin::Functional(Functional {
    ///     namespace: "emoji",
    ///     can_handle: |context| matches!(context.modifier, Modifier::Builtin { value: "tada", .. }),
    ///     handle: |context| {
    ///         generate_wrapper(context, |context| {
    ///             context.buffer.line(format_args!("content: \"\u{1f389}\";"));
    ///         });
    ///     },
    ///     ..Functional::default()
    /// });
    /// ```
    pub const fn default() -> Self {
        Self {
            namespace: "",
            can_handle: can_handle_nop,
            handle: handle_nop,
        }
    }
}

impl Functional<String> {
    /// Make a default [`Functional`] plugin kind.
    ///
    /// All required fields are initialized with empty values and optional fields are initialized
    /// with `None`.
    ///
    /// You should at least set [`Functional::namespace`], [`Functional::can_handle`] and [`Functional::handle`] after calling this function.
    ///
    /// This function is intended to be used as an automatic filler for default values using the
    /// [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
    ///
    /// The difference with [`Functional::default`] is that this function can only be used to
    /// build a plugin using heap-allocated structures like `String`s, `Vec`s.
    ///
    /// ### Example
    ///
    /// ```
    /// use encre_css::prelude::build_plugin::*;
    ///
    /// fn main() {
    ///     // Note: the DynamicPlugin type hint is required to help the compiler
    ///     // find the concrete types of type parameters
    ///     let _plugin: DynamicPlugin = Plugin::Functional(Functional {
    ///         namespace: "emoji".to_string(),
    ///         can_handle: |context| matches!(context.modifier, Modifier::Builtin { value: "tada", .. }),
    ///         handle: |context| {
    ///             generate_wrapper(context, |context| {
    ///                 context.buffer.line(format_args!("content: \"\u{1f389}\";"));
    ///             });
    ///         },
    ///         ..Functional::default_dynamic()
    ///     });
    /// }
    /// ```
    ///
    /// This example is equivalent to the one of [`Functional::default`].
    pub fn default_dynamic() -> Self {
        Self {
            namespace: String::new(),
            can_handle: can_handle_nop,
            handle: handle_nop,
        }
    }
}

/// A plugin is a structure capable of generating CSS styles from a CSS selector.
///
/// Several kinds of plugins exist and define what values are accepted as selector or modifier and
/// what CSS is generated based on the input selector. The API is designed to be fully declarative
/// (so that plugin declarations are serializable), except for the
/// [functional kind](Plugin::Functional).
///
/// Each plugin kind has a set of required parameters and a set of default parameters which can be
/// automatically filled in Rust using the [struct update syntax](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-with-struct-update-syntax).
///
/// It's common to define several plugins to handle a single utility class, and to define static
/// plugins as constants (the `default` function on each plugin kind is a `const fn`).
///
/// After you have defined a plugin, you need to register it in the [`Config`] structure by calling
/// [`Config::register_plugin`].
///
/// # Simple example (defines the static values of the `font-family` plugin)
///
/// ```
/// use encre_css::prelude::build_plugin::*;
///
/// const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
///     prop: SingleProp("font-family"),
///     values: map! {
///         "font-sans" => r#"ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont"#,
///         "font-serif" => r#"Georgia, Cambria, "Times New Roman", Times, serif"#,
///         "font-mono" => r#"Menlo, Monaco, Consolas, "Liberation Mono", monospace"#,
///     },
///     ..ListValues::default()
/// });
/// ```
///
/// # More advanced example (defines the `stroke-width` plugin)
///
/// ```
/// use encre_css::prelude::build_plugin::*;
///
/// const PLUGIN: StaticPlugin = Plugin::Number(Number {
///     namespace: "stroke",
///     prop: SingleProp("stroke-width"),
///     template: Some(SingleProp("{}px")),
///     ..Number::default()
/// });
///
/// // There's also a plugin sharing the same `stroke` namespace (which helps changing the
/// // stroke color, e.g `stroke-red-500`), so it's required to define `hints` and `matchers`
/// const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
///     namespace: "stroke",
///     prop: SingleProp("stroke-width"),
///     disambiguate: Some(ArbitraryDisambiguate {
///         matched: &[CssType::Length, CssType::Percentage, CssType::LineWidth, CssType::Number],
///         separation: ArbitraryDisambiguateSeparation::Comma,
///     }),
///     ..Arbitrary::default()
/// });
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
/// const PLUGIN: StaticPlugin = Plugin::Functional(Functional {
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
/// Instead of defining plugins in Rust, you can also define them in `encre-css`'s TOML configuration
/// (or every other language that uses a `serde` deserializer).
/// The sole exception is plugins using the [`Functional`] kind which are not serializable.
///
/// To do that, you need to add a new entry in the `custom_plugins` list of the configuration.
/// You can then define plugins as you would do in Rust.
///
/// ### Example
///
/// ```toml
/// [[custom_plugins]]
///
/// [custom_plugins.Number]
/// namespace = "stroke"
/// prop = "stroke-width"
/// template = "{}px"
///
/// [[custom_plugins]]
///
/// [custom_plugins.Arbitrary]
/// namespace = "stroke"
/// prop = "stroke-width"
/// hints = ["Length", "Percentage"]
/// matchers = [["Length", "Percentage", "LineWidth", "Number"], "Comma"]
/// ```
///
/// # Advice
///
/// `encre-css` builds a [trie structure](https://en.wikipedia.org/wiki/Trie) based on the
/// namespace of the plugins to optimize matching a utility class to a specific plugin, so it's
/// **highly discouraged to leave the namespace of a plugin empty**, otherwise the performances will
/// decrease heavily.
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
/// [`Config::register_plugin`]: crate::Config::register_plugin
/// [`Config`]: crate::Config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Plugin<Str, ArrayStr, MapStr, MapArrayStr, ArrayMatched> {
    /// See [`ListProperties`].
    ListProperties(ListProperties<Str, ArrayStr, MapStr, MapArrayStr>),

    /// See [`ListValues`].
    ListValues(ListValues<Str, ArrayStr, MapStr>),

    /// See [`Spacing`].
    Spacing(Spacing<Str, ArrayStr, MapStr>),

    /// See [`Color`].
    Color(Color<Str, ArrayStr, MapStr>),

    /// See [`Number`].
    Number(Number<Str, ArrayStr, MapStr>),

    /// See [`Arbitrary`].
    Arbitrary(Arbitrary<Str, ArrayStr, MapStr, ArrayMatched>),

    /// See [`Functional`].
    ///
    /// Not serializable.
    #[serde(skip)]
    Functional(Functional<Str>),
}
