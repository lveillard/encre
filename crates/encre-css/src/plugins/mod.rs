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

use crate::{generator::ContextHandle, plugins::PropertyName::MultipleProps, selector::ArbitraryHint};

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

/// A plugin is a structure capable of generating CSS styles from a modifier (contained in a
/// context structure).
///
/// Each plugin consists of two methods:
/// - [`Plugin::new(PluginKind::can_handle`] to check if it will be able to generate CSS for a specific modifier;
/// - [`Plugin::new(PluginKind::handle`] to generate the CSS needed.
///
/// The [`Plugin::new(PluginKind::can_handle`] method takes a [`ContextCanHandle`] structure containing the
/// modifier and the current configuration.
///
/// The [`Plugin::new(PluginKind::handle`] method takes a [`ContextHandle`] structure containing the modifier,
/// the current configuration and a buffer containing the whole CSS
/// currently generated. You can use the [`Buffer`] structure (especially the [`Buffer::line`]
/// and [`Buffer::lines`] functions) to push CSS declarations to it, they will be automatically
/// indented.
///
/// It is common to use the [`unreachable!`] macro if the [`Plugin::new(PluginKind::handle`] method cannot be
/// called because you are sure that [`Plugin::new(PluginKind::can_handle`] returned `false`.
///
/// # Example (defines the `stroke-width` plugin)
///
/// ```
/// use encre_css::prelude::build_plugin::*;
///
/// #[derive(Debug)]
/// struct StrokeWidth;
///
/// impl Plugin for StrokeWidth {
///     fn can_handle(&self, context: ContextCanHandle) -> bool {
///         match context.modifier {
///             Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
///             Modifier::Arbitrary { hint, value, .. } => {
///                 *hint == "length"
///                     || *hint == "number"
///                     || *hint == "percentage"
///                     || (hint.is_empty()
///                         && (is_matching_length(value) || is_matching_percentage(value)))
///             }
///         }
///     }
///
///     fn handle(&self, context: &mut ContextHandle) {
///         match context.modifier {
///             Modifier::Builtin { value, .. } => {
///                 context.buffer.line(format_args!("stroke-width: {value}px;"));
///             }
///             Modifier::Arbitrary { value, .. } => {
///                 context.buffer.line(format_args!("stroke-width: {value});"));
///             }
///         }
///     }
/// }
/// ```
///
/// # Release a plugin as a crate
///
/// If you want to release your custom plugins as a crate, you can export a `register` function
/// taking a mutable reference to a [`Config`] structure and use the [`Config::register_plugin`]
/// function to register them. The first argument is the namespace namespaceing all the
/// utility classes handled by the plugin.
///
/// ```ignore
/// pub fn register(config: &mut Config) {
///     config.register_plugin("stroke", &StrokeWidth);
/// }
/// ```
///
/// # More powerful usage
///
/// If you need to have full control over the CSS **rule**, you can create a [`needs_wrapping`]
/// method returning false and use [`generator::generate_at_rules`], [`generator::generate_class`]
/// and [`generator::generate_wrapper`] to generate some CSS boilerplate.
///
/// ### Example (roughly defines the `animation` plugin)
///
/// ```
/// use encre_css::prelude::build_plugin::*;
///
/// #[derive(Debug)]
/// struct PluginDefinition;
///
/// impl Plugin for PluginDefinition {
///     fn needs_wrapping(&self) -> bool {
///         false
///     }
///
///     fn can_handle(&self, context: ContextCanHandle) -> bool {
///         match context.modifier {
///             Modifier::Builtin { value, .. } => {
///                 ["spin", "ping", "pulse", "bounce", "none"].contains(value)
///             }
///             Modifier::Arbitrary { value, .. } => is_matching_all(value),
///         }
///     }
///
///     fn handle(&self, context: &mut ContextHandle) {
///         match context.modifier {
///             Modifier::Builtin { value, .. } => {
///                 let animation = match *value {
///                     "none" => "none",
///                     "spin" => {
///                         context.buffer.lines([
///                             "@keyframes spin",
///                             "...",
///                         ]);
///                         "spin 1s linear infinite"
///                     }
///                     "ping" => {
///                         context.buffer.lines([
///                             "@keyframes ping",
///                             "...",
///                         ]);
///                         "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite"
///                     }
///                     "pulse" => {
///                         context.buffer.lines([
///                             "@keyframes pulse",
///                             "...",
///                         ]);
///                         "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite"
///                     }
///                     "bounce" => {
///                         context.buffer.lines([
///                             "@keyframes bounce",
///                             "...",
///                         ]);
///                         "bounce 1s infinite"
///                     }
///                     _ => unreachable!(),
///                 });
///
///                 generate_wrapper(context, |context| {
///                     context.buffer.line(format_args!("animation: {animation});"));
///                 })
///             }
///             Modifier::Arbitrary { value, .. } => generate_wrapper(context, |context| {
///                 context.buffer.line(format_args!("animation: {value});"));
///             }),
///         }
///     }
/// }
/// ```
///
/// Have a look at <https://gitlab.com/encre-org/encre-css/tree/main/crates/encre-css/src/plugins>
/// for more examples.
///
/// [`Buffer`]: crate::utils::buffer::Buffer
/// [`Buffer::line`]: crate::utils::buffer::Buffer::line
/// [`Buffer::lines`]: crate::utils::buffer::Buffer::lines
/// [`Config::register_plugin`]: crate::Config::register_plugin
/// [`Config`]: crate::Config
/// [`needs_wrapping`]: Plugin::new(PluginKind::needs_wrapping
/// [`generator::generate_at_rules`]: crate::generator::generate_at_rules
/// [`generator::generate_class`]: crate::generator::generate_class
/// [`generator::generate_wrapper`]: crate::generator::generate_wrapper
/*
pub trait Plugin: fmt::Debug {
    /// Returns whether the plugin can handle a specific modifier.
    fn can_handle(&self, _context: ContextCanHandle) -> bool;

    /// Returns whether the plugin should be wrapped inside a CSS rule or if it will manually
    /// generate it
    fn needs_wrapping(&self) -> bool {
        true
    }

    /// Get the CSS code from a modifier.
    ///
    ///
    /// The [`Plugin::new(PluginKind::can_handle`] method **must be** called before to know if it can handle
    /// the modifier, otherwise this function **will panic**.
    ///
    /// Various notes:
    /// - The CSS returned should end with a newline;
    /// - Arbitrary values are already normalized (e.g. underscores are replaced by spaces);
    /// - This function is guaranteed to be called only once per selector.
    fn handle(&self, _context: &mut ContextHandle);
}*/

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PluginArbitraryMatcher {
    All,
    Url,
    Var,
    Shadow,
    AbsoluteSize,
    RelativeSize,
    LineWidth,
    LineStyle,
    ComputationalCssFunction,
    Color,
    Length,
    Number,
    Percentage,
    Time,
    Gradient,
    Position,
    Angle,
    Image,
    FontFamilyName,
    Custom(&'static str),
    CustomMultiple(&'static [&'static str]),

    Or(
        &'static PluginArbitraryMatcher,
        &'static PluginArbitraryMatcher,
    ),
    OrMultiple(&'static [&'static PluginArbitraryMatcher]),
    CommaSeparated(&'static PluginArbitraryMatcher),
    SpaceSeparated(&'static PluginArbitraryMatcher),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PropertyName {
    SingleProp(&'static str),
    MultipleProps(&'static [&'static str]),
}

// TODO: Make a StaticPlugin/DynamicPlugin (with Strings and Vecs for ser/de)
// TODO: Think about what items need to be public and/or reexported for Functional kind

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

    pub const fn has_auto(mut self) -> Self {
        if !matches!(self.kind, PluginKind::Spacing { .. } | PluginKind::Number { .. }) {
            panic!("Plugin::has_auto only works with PluginKind::Spacing or PluginKind::Number");
        }

        self.has_auto = true;
        self
    }

    pub const fn has_empty(mut self) -> Self {
        if !matches!(self.kind, PluginKind::Number { .. }) {
            panic!("Plugin::has_empty only works with PluginKind::Number");
        }

        self.has_empty = true;
        self
    }

    pub const fn has_full(mut self) -> Self {
        if !matches!(self.kind, PluginKind::Spacing { .. }) {
            panic!("Plugin::has_full only works with PluginKind::Spacing");
        }

        self.has_full = true;
        self
    }

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

    Arbitrary {
        namespace: &'static str,
        prop: PropertyName,
    },

    Functional {
        class: &'static str,
        handle: fn(&mut ContextHandle),
    },
}
