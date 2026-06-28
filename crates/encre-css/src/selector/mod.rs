//! Define the structures used to parse scanned classes.
//!
//! ## Discover what is possible to do with classes by learning some vocabulary
//!
//! <style>
//! .with-hints {
//!   font-family: sans-serif;
//!   overflow: visible !important;
//!   height: 3rem;
//!   font-size: 1rem;
//! }
//!
//! .with-hints > b > span:nth-child(even) {
//!   padding: 0 0.35rem;
//! }
//!
//! .with-hints > b > span:nth-child(odd) {
//!   position: relative;
//!   text-decoration: underline;
//!   text-underline-offset: 6px;
//! }
//!
//! .with-hints span:nth-child(odd)::after {
//!   content: counter(hints);
//!   position: absolute;
//!   bottom: -1.75rem;
//!   left: 50%;
//!   transform: translateX(-50%);
//!   font-size: 0.7rem;
//!   border: 2px solid currentColor;
//!   border-radius: 50%;
//!   width: 1.1rem;
//!   height: 1.1rem;
//!   display: flex;
//!   justify-content: center;
//!   align-items: center;
//!   counter-increment: hints;
//! }
//! </style>
//!
//! <p class="with-hints" style="counter-reset: hints; margin-top: 2rem;"><b><span>hover:xl</span><span>:</span><span>bg</span><span>-</span><span>red-500</span></b></p>
//!
//! 1. The **[variants](crate::selector::Variant)** (used to add pseudo-selectors, pseudo-elements, pseudo classes, media queries), in this case
//!    the class will be applied only on a screen larger than 1280px (see [`BUILTIN_SCREENS`]) and
//!    if hovered;
//! 2. The **namespace** (basically the name of the plugin), in this case `bg` for changing the background;
//! 3. The **[modifier](crate::selector::Modifier)** (used to clarify the CSS needed to be generated), in this case the
//!    background color will become `rgb(239 68 68)` (see [`BUILTIN_COLORS`]).
//!
//! <p class="with-hints" style="margin-top: 3rem;"><b><span style="counter-set: hints 3;">[&>*]</span><span>:</span><span style="counter-set: hints 4;">[@supports_(display:flex)]</span><span>:</span><span style="counter-set: hints 1;">flex</span><span>-</span><span style="counter-set: hints 5;">[2_2_10%]</span></b></p>
//!
//! 4. The **arbitrary variant** (used to modify the class generated), in this case the class
//!    will be `.\[\&\>\*]\:flex-\[2_2_10\%\]>*`.
//!
//! 5. Another **arbitrary variant** (with another syntax used to add [at rules](https://developer.mozilla.org/en-US/docs/Web/CSS/At-rule)),
//!    in this case the rule will become `@supports (display:flex) { <rule content> }` (spaces need to be replaced with underscores in arbitrary
//!    variants).
//!
//! 6. The **[arbitrary value](crate::selector::Modifier::Arbitrary)**
//!    (used to specify a value not included in your design system), in this case the background
//!    color will become `2 2 10%` (spaces need to be replaced with underscores in arbitrary
//!    values).
//!
//! <p class="with-hints" style="margin-top: 3rem;"><b><span>[mask-type:luminance]</span></b></p>
//!
//! 7. The **arbitrary CSS property** (used to use a CSS property not supported by `encre-css`), in
//!    this case the rule content will be `.\[mask-type\:luminance\] { mask-type: luminance; }`.
//!
//! <p class="with-hints" style="margin-top: 3rem;"><b><span>dark:(text-white,bg-gray-500)</span></b></p>
//!
//! 8. The **variant group** (used to group together several classes conditionally enabled by the
//!    same variant), in this case the class will be expanded to `dark:text-white` and
//!    `dark:bg-gray-500`.
//!
//! <p class="with-hints" style="margin-top: 3rem;"><b><span>(hover,focus-visible):bg-blue-400</span></b></p>
//!
//! 9. The **variant group without any namespace or modifier** (used to group together several classes conditionally enabled by the
//!    same variant but sharing the same modifier), in this case the class will be expanded to `hover:bg-blue-400` and
//!    `focus-visible:bg-blue-400`.
//!
//! As you can see, by default variants are separated by `:`, modifiers by `-` (the dash after the
//! first modifier can be omitted, e.g. `m1` instead of `m-1`), arbitrary values/variants are surrounded by `[]` and variant
//! groups are surrounded by `()`.
//!
//! ### Automatic replacements
//!
//! Because HTML classes can't contain several symbols (e.g spaces), replacement characters must be
//! used **in arbitrary values, variants and CSS properties**.
//!
//! - `_` is replaced by a space (e.g instead of writing `content-[hello world]` which is
//!   considered two different classes by browsers, use `content-[hello_world]`)
//! - `&#34;` is replaced by `"`
//! - `&#39;` is replaced by `'`
//! - `&#40;` is replaced by `(`
//! - `&#41;` is replaced by `)`
//! - `&#91;` is replaced by `[`
//! - `&#92;` is replaced by `\`
//! - `&#93;` is replaced by `]`
//! - `&#95;` is replaced by `_` (useful when a real `_` is needed instead of a space)
//! - `&#96;` is replaced by `` ` ``
//!
//! [`BUILTIN_SCREENS`]: crate::config::BUILTIN_SCREENS
//! [`BUILTIN_COLORS`]: crate::config::BUILTIN_COLORS
pub(crate) mod parser;

use crate::plugins::Plugin;

use std::{borrow::Cow, cmp::Ordering};

pub(crate) use parser::parse;

/// The modifier is the rest of the selector after the namespace, it is used to clarify the
/// CSS needed to be generated.
#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Modifier<'a> {
    /// A builtin static modifier (e.g. `bg-red-500`).
    Builtin {
        /// Whether the value is negative (e.g. `-translate-2` is negative).
        is_negative: bool,

        /// The inner value of the modifier.
        value: &'a str,
    },

    /// A dynamic modifier capable of automatically generating a rule from a CSS value
    /// (e.g. `bg-[rgb(12_12_12)]`).
    ///
    /// All underscores in the value will be replaced by spaces except in `url()`, if you really
    /// want to keep one of them, you can prefix it with a backslash `\_` and it will be used as
    /// is.
    ///
    /// Sometimes the value is ambiguous, for example `bg-[var(--foo)]` can be handled by either
    /// the [`background color`](crate::plugins::background::background_color) or the
    /// [`background size`](crate::plugins::background::background_size) utility. In this case,
    /// you need to provide a [CSS type](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Types)
    /// hint (see the list of hints below) before the arbitrary value. For example
    /// `bg-[length:var(--foo)]` will generate `background-size: var(--foo);` (using the
    /// [`background size`](crate::plugins::background::background_size) utility).
    ///
    /// List of all type hints:
    /// - `color`
    /// - `length`
    /// - `line-width`
    /// - `image`
    /// - `url`
    /// - `position`
    /// - `percentage`
    /// - `number`
    /// - `generic-name`
    /// - `family-name`
    /// - `absolute-size`
    /// - `relative-size`
    /// - `shadow`
    Arbitrary {
        /// The rest of the modifier without the arbitrary value (e.g. `bg` in `bg-[rgb(12_12_12)]`).
        prefix: &'a str,

        /// The type hint needed for ambiguous values.
        hint: &'a str,

        /// The inner value of the modifier.
        ///
        /// All escaped characters (prefixed by a backslash) are already unescaped.
        value: Cow<'a, str>,
    },
}

/// A selector variant.
#[derive(Debug, Clone, Eq)]
pub struct Variant<'a> {
    pub(crate) order: usize,
    pub(crate) prefixed: bool,
    pub(crate) template: Cow<'a, str>,
}

impl<'a> Variant<'a> {
    pub(crate) const fn new_const(counter: &mut usize, template: &'static str) -> Self {
        *counter += 1;

        Self {
            order: *counter - 1,
            prefixed: false,
            template: Cow::Borrowed(template),
        }
    }

    /// Create a new variant.
    ///
    /// The order is used to decide where the generated class having this variant will be placed in
    /// the generated CSS. [`Config::last_variant_order`] can be used to insert a variant after all
    /// the others.
    ///
    /// The template is a string which defines how the class will be modified. If it starts with `@`,
    /// a CSS block will wrap the inner class (like media queries), otherwise just the class name will
    /// be modified.
    ///
    /// The template should contain `&` which will be replaced by the complete class name.
    ///
    /// # Example
    ///
    /// ```
    /// use encre_css::{Config, selector::Variant};
    /// use std::borrow::Cow;
    ///
    /// let mut config = Config::default();
    /// config.register_variant(
    ///     "headings",
    ///     // Insert the classes having this variant after all the other variants
    ///     Variant::new(config.last_variant_order(), "& :where(h1, h2, h3, h4, h5, h6)")
    /// );
    ///
    /// let generated = encre_css::generate(
    ///     ["headings:text-gray-700"],
    ///     &config,
    /// );
    ///
    /// assert!(generated.ends_with(".headings\\:text-gray-700 :where(h1, h2, h3, h4, h5, h6) {
    ///   color: oklch(37.3% .034 259.733);
    /// }"));
    /// ```
    ///
    /// [`Config::last_variant_order`]: crate::Config::last_variant_order
    pub fn new<T: Into<Cow<'a, str>>>(order: usize, template: T) -> Self {
        Self {
            order,
            prefixed: false,
            template: template.into(),
        }
    }

    /// Defines a prefixed variant which is composed of a prefix and an arbitrary value which will
    /// be inserted into the variant template.
    ///
    /// A prefixed variant can have `{}` in its template which will be replaced by the arbitrary
    /// value.
    ///
    /// # Example
    ///
    /// ```
    /// use encre_css::{Config, selector::Variant};
    /// use std::borrow::Cow;
    ///
    /// let mut config = Config::default();
    /// config.register_variant(
    ///     "media",
    ///     // Insert the classes having this variant after all the other variants
    ///     Variant::new(config.last_variant_order(), "@media {}").with_prefixed()
    /// );
    ///
    /// let generated = encre_css::generate(
    ///     ["media-[print]:flex"],
    ///     &config,
    /// );
    ///
    /// assert!(generated.ends_with(r"@media print {
    ///   .media-\[print\]\:flex {
    ///     display: flex;
    ///   }
    /// }"));
    /// ```
    #[must_use]
    pub const fn with_prefixed(mut self) -> Self {
        self.prefixed = true;
        self
    }
}

impl PartialEq for Variant<'_> {
    fn eq(&self, other: &Self) -> bool {
        // Does not test order because it can change
        self.template == other.template
    }
}

/// A parsed selector, aka a utility class, containing the variants, the namespace and the modifier.
///
/// See [`crate::selector`] for more information.
#[derive(Clone, Debug)]
pub(crate) struct Selector<'a> {
    pub(crate) layer: i8,
    pub(crate) order: usize,
    pub(crate) full: &'a str,
    pub(crate) modifier: Modifier<'a>,
    pub(crate) variants: Vec<Variant<'a>>,
    pub(crate) is_important: bool,
    pub(crate) plugin: &'static Plugin,
}

impl PartialEq for Selector<'_> {
    fn eq(&self, other: &Self) -> bool {
        // Does not test order because it can change
        self.full == other.full
            && self.modifier == other.modifier
            && self.variants == other.variants
            && self.is_important == other.is_important
            && self.layer == other.layer
    }
}

impl Eq for Selector<'_> {}

impl PartialOrd for Selector<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Selector<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        // We need to check the order as well as the strict egality because the PartialEq
        // implementation for Selector does not check if two selectors have the same plugin which
        // can lead two selectors having the same modifier to be recognized as the same although
        // they use different plugins
        if self.order == other.order && self == other {
            return Ordering::Equal;
        }

        self.layer.cmp(&other.layer).then_with(|| {
            if self.variants.is_empty() && !other.variants.is_empty() {
                Ordering::Less
            } else if !self.variants.is_empty() && other.variants.is_empty() {
                Ordering::Greater
            } else if !self.variants.is_empty() && !other.variants.is_empty() {
                let mut compared = None;

                // Compare variants in the lexicographic order
                for variant_i in 0..self.variants.len() {
                    if variant_i >= other.variants.len() {
                        compared = Some(Ordering::Greater);
                        break;
                    }

                    let res = self
                        .variants
                        .get(variant_i)
                        .as_ref()
                        .unwrap()
                        .order
                        .cmp(&other.variants.get(variant_i).unwrap().order);

                    if res != Ordering::Equal {
                        compared = Some(res);
                        break;
                    }
                }

                compared.unwrap_or(Ordering::Less).then_with(|| {
                    self.order.cmp(&other.order).then_with(|| {
                        self.full
                            .cmp(other.full)
                            .then_with(|| self.modifier.cmp(&other.modifier))
                    })
                })
            } else {
                self.order.cmp(&other.order).then_with(|| {
                    self.full
                        .cmp(other.full)
                        .then_with(|| self.modifier.cmp(&other.modifier))
                })
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, selector::parse};

    use std::collections::BTreeSet;

    #[test]
    fn sorting_test() {
        let config = Config::default();

        let selectors1 = parse(
            "lg:bg-red-500",
            None,
            None,
            &config,
            &config.get_derived_variants(),
        );
        let selectors2 = parse(
            "bg-red-500",
            None,
            None,
            &config,
            &config.get_derived_variants(),
        );

        let mut selectors = BTreeSet::new();
        selectors.insert(selectors1[0].as_ref().unwrap());
        selectors.insert(selectors2[0].as_ref().unwrap());

        let mut iter = selectors.iter();
        assert!(
            iter.next().unwrap().full == "bg-red-500"
                && iter.next().unwrap().full == "lg:bg-red-500"
        );
    }

    #[test]
    fn layers_test() {
        let config = Config::default();

        let selectors1 = parse(
            "lg:bg-red-500",
            None,
            None,
            &config,
            &config.get_derived_variants(),
        );
        let mut selectors2 = parse(
            "bg-red-500",
            None,
            None,
            &config,
            &config.get_derived_variants(),
        );
        selectors2[0].as_mut().unwrap().layer = 42;

        let mut selectors = BTreeSet::new();
        selectors.insert(selectors1[0].as_ref().unwrap());
        selectors.insert(selectors2[0].as_ref().unwrap());

        let mut iter = selectors.iter();
        assert!(
            iter.next().unwrap().full == "lg:bg-red-500"
                && iter.next().unwrap().full == "bg-red-500"
        );
    }
}
