//! Define some utility functions for quickly doing things.
use crate::{
    config::Config,
    error::{ParseError, ParseErrorKind},
    selector::{
        Selector,
        parser::{ARBITRARY_END, ARBITRARY_START, ESCAPE, GROUP_END, GROUP_START, parse},
    },
};

use std::{cmp::Ordering, iter, str::CharIndices};

pub mod buffer;
pub mod color;
pub mod shadow;
pub mod spacing;
pub mod value_matchers;

#[cfg(test)]
pub(crate) mod testing;

/// While <https://github.com/rust-lang/rust/issues/27721> is pending we need to define
/// our own minimal [`Pattern`] trait.
///
/// | Pattern type             | Match condition                           |
/// |--------------------------|-------------------------------------------|
/// | `char`                   | is contained in string                    |
/// | `&[char]`                | any char in slice is contained in string  |
/// | `F: FnMut(char) -> bool` | `F` returns `true` for a char in string   |
///
/// [`Pattern`]: std::str::pattern::Pattern
pub trait Pattern {
    /// Returns whether the character is matching the pattern.
    fn is_matching(&self, val: char) -> bool;
}

impl Pattern for char {
    fn is_matching(&self, val: char) -> bool {
        val == *self
    }
}

impl Pattern for &[char] {
    fn is_matching(&self, val: char) -> bool {
        #[allow(clippy::manual_contains)]
        self.iter().any(|ch| val == *ch)
    }
}

impl<F: Fn(char) -> bool> Pattern for F {
    fn is_matching(&self, val: char) -> bool {
        self(val)
    }
}

/// An iterator ignoring values wrapped in parenthesis and brackets.
///
/// This structure is created by the [`split_ignore_arbitrary`] function. See its documentation for
/// more.
#[derive(Debug)]
pub struct SplitIgnoreArbitrary<'a, P: Pattern> {
    val: &'a str,
    iter: CharIndices<'a>,
    searched_pattern: P,
    ignore_parenthesis: bool,
    is_next_escaped: bool,
    last_slice_returned: bool,
    parenthesis_level: usize,
    bracket_level: usize,
    last_index: usize,
    seek_index: usize,
}

impl<'a, P: Pattern> Iterator for SplitIgnoreArbitrary<'a, P> {
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.is_next_escaped {
                let _ = self.iter.next()?;
                self.is_next_escaped = false;
                continue;
            }

            let ch = self.iter.next();

            if let Some(ch) = ch {
                match ch.1 {
                    ESCAPE => self.is_next_escaped = true,
                    GROUP_START if self.ignore_parenthesis && self.bracket_level == 0 => {
                        self.parenthesis_level += 1;
                    }
                    GROUP_END if self.ignore_parenthesis && self.bracket_level == 0 => {
                        if self.parenthesis_level > 0 {
                            self.parenthesis_level -= 1;
                            self.seek_index = ch.0 + 1;
                        }
                    }
                    ARBITRARY_START => self.bracket_level += 1,
                    ARBITRARY_END => {
                        if self.bracket_level > 0 {
                            self.bracket_level -= 1;
                            self.seek_index = ch.0 + 1;
                        }
                    }
                    _ => {
                        if self.searched_pattern.is_matching(ch.1)
                            && self.bracket_level == 0
                            && !(self.ignore_parenthesis && self.parenthesis_level > 0)
                        {
                            let last_index = self.last_index;
                            self.last_index = ch.0 + ch.1.len_utf8();
                            self.seek_index = self.last_index;
                            return Some((last_index, &self.val[last_index..ch.0]));
                        }
                    }
                }
            } else if !self.last_slice_returned {
                // The characters are all handled, return the last slice
                let last_index = self.last_index;
                self.last_index = self.val.len();
                self.last_slice_returned = true;
                return Some((last_index, &self.val[last_index..self.val.len()]));
            } else {
                // The characters are all handled, and the last slice was returned if no character is
                // searched, return `None`
                return None;
            }
        }
    }
}

/// Split a value while avoiding arbitrary values/variants (wrapped in brackets) from being split.
///
/// The last argument indicates whether variant groups (wrapped in parentheses) are also ignored.
///
/// # Example
///
/// ```
/// use encre_css::utils::split_ignore_arbitrary;
///
/// let value = "bg-red-500 content-[wrapped in `[]`, will not be split] (words wrapped in parenthesis are not split too)";
/// assert_eq!(split_ignore_arbitrary(value, ' ', true).collect::<Vec<(usize, &str)>>(), vec![(0, "bg-red-500"), (11, "content-[wrapped in `[]`, will not be split]"), (56, "(words wrapped in parenthesis are not split too)")]);
/// ```
pub fn split_ignore_arbitrary<P: Pattern>(
    val: &str,
    searched_pattern: P,
    ignore_parenthesis: bool,
) -> impl Iterator<Item = (usize, &str)> {
    SplitIgnoreArbitrary {
        val,
        iter: val.char_indices(),
        searched_pattern,
        ignore_parenthesis,
        is_next_escaped: false,
        last_slice_returned: false,
        parenthesis_level: 0,
        bracket_level: 0,
        last_index: 0,
        seek_index: 0,
    }
}

fn sort_selectors_recursive<'a>(
    val: impl Iterator<Item = &'a str>,
    separator: &str,
    config: &Config,
) -> String {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum FoundSelector<'a> {
        UnknownSelector(&'a str),
        KnownSelector(Selector<'a>),
        Group(String),
    }

    fn split_map_closure(s: (usize, &str)) -> &str {
        s.1
    }

    fn dedup_key<'a>(s: &'a FoundSelector<'a>) -> &'a str {
        match s {
            FoundSelector::KnownSelector(s) => s.full,
            FoundSelector::UnknownSelector(s) => s,
            FoundSelector::Group(g) => g,
        }
    }

    let trie = crate::selector::trie::build_trie(&config);
    let config_derived_variants = config.get_derived_variants();
    let mut selectors = val
        .filter_map(|v| {
            let selectors = parse(
                v.trim(),
                None,
                None,
                config,
                &config_derived_variants,
                &trie,
            );

            if selectors.len() > 1 {
                // Sort variant groups
                let start = split_ignore_arbitrary(v.trim(), '(', false)
                    .nth(1)
                    .map(|(n, _s)| n)
                    .unwrap_or_default();

                Some(FoundSelector::Group(format!(
                    "{}{})",
                    &v[..start],
                    sort_selectors_recursive(
                        split_ignore_arbitrary(v[start..v.len() - 1].trim(), ',', true)
                            .map(split_map_closure),
                        ",",
                        config,
                    )
                )))
            } else {
                match selectors.into_iter().next()? {
                    Ok(selector) => Some(FoundSelector::KnownSelector(selector)),
                    Err(ParseError {
                        kind:
                            ParseErrorKind::TooShort(selector)
                            | ParseErrorKind::VariantsWithoutModifier(selector)
                            | ParseErrorKind::UnknownPlugin(selector)
                            | ParseErrorKind::UnknownVariant(_, selector),
                        ..
                    }) => Some(FoundSelector::UnknownSelector(selector)),
                }
            }
        })
        .collect::<Vec<FoundSelector>>();

    // Sort selectors
    selectors.sort_unstable_by(|a, b| match (a, b) {
        (FoundSelector::KnownSelector(_), FoundSelector::UnknownSelector(_))
        | (FoundSelector::Group(_), _) => Ordering::Greater,
        (FoundSelector::UnknownSelector(_), FoundSelector::KnownSelector(_))
        | (_, FoundSelector::Group(_)) => Ordering::Less,
        (FoundSelector::KnownSelector(a), FoundSelector::KnownSelector(b)) => a.cmp(b),
        (FoundSelector::UnknownSelector(a), FoundSelector::UnknownSelector(b)) => a.cmp(b),
    });

    // Deduplicate selectors
    selectors.dedup_by(|a, b| dedup_key(&*a) == dedup_key(&*b));

    selectors
        .iter()
        .map(|s| match s {
            FoundSelector::KnownSelector(s) => s.full,
            FoundSelector::UnknownSelector(s) => s,
            FoundSelector::Group(g) => g,
        })
        .collect::<Vec<&str>>()
        .join(separator)
}

/// Sort a list of selectors (separated by spaces) according to `encre-css` rules.
///
/// Note: selectors are also deduplicated.
///
/// # Example
///
/// ```
/// use encre_css::{Config, utils::sort_selectors};
///
/// let value = "foo text-white px-4 sm:px-8 py-2 qux:(bg-green-500,dark:bar:foo) sm:py-3 bar bg-sky-700 foo focus:(md:text-white,lg:text-gray-500) hover:bg-sky-800";
/// assert_eq!(sort_selectors(value, &Config::default()), "bar foo qux:(bg-green-500,dark:bar:foo) bg-sky-700 px-4 py-2 text-white hover:bg-sky-800 sm:py-3 sm:px-8 focus:(lg:text-gray-500,md:text-white)".to_string());
/// ```
pub fn sort_selectors(val: &str, config: &Config) -> String {
    sort_selectors_recursive(val.split_whitespace(), " ", config)
}

/// Return the list of errors encountered when parsing a list of selectors
///
/// # Example
///
/// ```
/// use encre_css::{Config, error::{ParseError, ParseErrorKind}, utils::check_selectors};
///
/// let value = "bg text-red hover:a lg: focus:() dark:(md:,shadow-8xl) bar:text-black md:foo:flex";
/// assert_eq!(check_selectors(value, &Config::default()), vec![
///     ParseError { span: 0..2, kind: ParseErrorKind::UnknownPlugin("bg") },
///     ParseError { span: 3..11, kind: ParseErrorKind::UnknownPlugin("text-red") },
///     ParseError { span: 12..19, kind: ParseErrorKind::UnknownPlugin("hover:a") },
///     ParseError { span: 20..23, kind: ParseErrorKind::VariantsWithoutModifier("lg:") },
///     ParseError { span: 24..32, kind: ParseErrorKind::VariantsWithoutModifier("focus:()") },
///     ParseError { span: 39..42, kind: ParseErrorKind::VariantsWithoutModifier("md:") },
///     ParseError { span: 43..53, kind: ParseErrorKind::UnknownPlugin("shadow-8xl") },
///     ParseError { span: 55..69, kind: ParseErrorKind::UnknownVariant("bar", "bar:text-black") },
///     ParseError { span: 70..81, kind: ParseErrorKind::UnknownVariant("foo", "md:foo:flex") }
/// ]);
/// ```
pub fn check_selectors<'a>(val: &'a str, config: &Config) -> Vec<ParseError<'a>> {
    let trie = crate::selector::trie::build_trie(&config);
    let config_derived_variants = config.get_derived_variants();
    val.char_indices()
        .chain(iter::once((val.len(), ' ')))
        .filter(|(_, ch)| ch.is_whitespace())
        .scan(0, |last_i, (i, _)| {
            let old_i = *last_i;
            *last_i = i + 1;
            Some((old_i..i, &val[old_i..i]))
        })
        .filter(|(_, v)| !v.is_empty())
        .flat_map(|(span, v)| {
            parse(
                v.trim(),
                Some(span),
                None,
                config,
                &config_derived_variants,
                &trie,
            )
        })
        .filter_map(Result::err)
        .collect::<Vec<ParseError>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::ParseErrorKind;

    #[test]
    fn sort_selectors_with_variant_groups() {
        assert_eq!(
            sort_selectors(
                "hover:(text-white,bg-sky-800) focus-within:bg-red-100 text-blue-500 md:flex [()())):]:checked:([))]:text-white,[))]:bg-red-500) hover:(focus:(focus-within:bg-red-500,checked:text-black),active:bg-red-500)",
                &Config::default()
            ),
            "text-blue-500 focus-within:bg-red-100 md:flex hover:(bg-sky-800,text-white) [()())):]:checked:([))]:bg-red-500,[))]:text-white) hover:(active:bg-red-500,focus:(checked:text-black,focus-within:bg-red-500))"
                .to_string()
        );
    }

    #[test]
    fn sort_selectors_deduplicate() {
        assert_eq!(sort_selectors("text-blue-100 text-blue-100 md:flex lg:block content-['hover:(md:text-white)'] md:flex focus:(hover:md:flex,lg:flex)", &Config::default()), "text-blue-100 content-['hover:(md:text-white)'] lg:block md:flex focus:(hover:md:flex,lg:flex)".to_string());
    }

    #[test]
    fn check_selectors_ignore_newlines_and_spaces() {
        assert_eq!(
            check_selectors(
                "text-blue-100   text-blue-100  md:flex   lg:block
content-['hover:(md:text-white)'] md:blue-flex

focus:(hover:md:flex,lg:flex)
  lg:bg-red-500",
                &Config::default()
            ),
            vec![ParseError {
                span: 84..96,
                kind: ParseErrorKind::UnknownPlugin("md:blue-flex")
            }]
        );
    }
}
