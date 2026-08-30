//! Define a structure used to scan content.
use std::{collections::BTreeSet, sync::Arc};

/// A structure responsible for scanning some content and returning a list of possible classes.
///
/// By default, it splits the content by spaces, double quotes, single quotes, backticks and new
/// lines, while ignoring arbitrary the content inside values/variants and variant groups
/// by using [`split_ignore_arbitrary`].
/// It is recommended to use this function when splitting classes with characters which can be
/// included inside arbitrary strings.
///
/// # Example
///
/// The following code snippet defines a scanner for extracting classes listed in the `data-en`
/// HTML attribute.
///
/// ```
/// use encre_css::{Config, Scanner};
/// use std::collections::BTreeSet;
///
/// let mut config = Config::default();
/// config.scanner = Scanner::from_fn(|content| content.split(r#"data-en=""#)
///     .filter_map(|v| v.split_once("\"").map(|(classes, _)| classes.split_whitespace()))
///     .flatten()
///     .collect::<BTreeSet<&str>>());
///
/// let generated = encre_css::generate(
///     [r#"<h1 data-en="underline"></h1><p data-en="bg-red-200 text-blue-300"></p>"#],
///     &config,
/// );
///
/// assert!(generated.ends_with(".bg-red-200 {
///   background-color: oklch(88.5% .062 18.334);
/// }
///
/// .text-blue-300 {
///   color: oklch(80.9% .105 251.813);
/// }
///
/// .underline {
///   -webkit-text-decoration-line: underline;
///   text-decoration-line: underline;
/// }"));
/// ```
///
/// [`split_ignore_arbitrary`]: crate::utils::split_ignore_arbitrary
#[allow(missing_debug_implementations)]
#[allow(clippy::type_complexity)]
#[derive(Clone)]
pub struct Scanner {
    scan_fn: Arc<dyn Fn(&str) -> BTreeSet<&str> + Send + Sync>,
}

impl Scanner {
    /// Build a [`Scanner`] from a closure taking some content and returning a list of possible
    /// classes.
    pub fn from_fn<T: 'static + Fn(&str) -> BTreeSet<&str> + Send + Sync>(scan_fn: T) -> Self {
        Self {
            scan_fn: Arc::new(scan_fn),
        }
    }

    pub(crate) fn scan<'a>(&self, val: &'a str) -> BTreeSet<&'a str> {
        (self.scan_fn)(val)
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self {
            scan_fn: Arc::new(|val| {
                val.split([' ', '\n', '\'', '"', '`', '\\'])
                    .collect::<BTreeSet<&str>>()
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    #[test]
    fn default_scanner_test() {
        assert_eq!(
            Scanner::default().scan("test bg-red-500 'hello' content-[some_[_square]_brackets] foo-bar sm:focus:ring hover:bg-black border-[#333] text-[color:var(--hello)]"),
            BTreeSet::from([
                "",
                "test",
                "bg-red-500",
                "hello",
                "content-[some_[_square]_brackets]",
                "foo-bar",
                "sm:focus:ring",
                "hover:bg-black",
                "border-[#333]",
                "text-[color:var(--hello)]",
            ])
        );
    }

    #[test]
    fn custom_scanner_test() {
        let scanner = Scanner::from_fn(|val| val.split(|ch| ch == '|').collect::<BTreeSet<&str>>());

        assert_eq!(
            scanner.scan("test|bg-red-500|'hello'"),
            BTreeSet::from(["test", "bg-red-500", "'hello'"])
        );
    }

    #[test]
    fn utf8_scan() {
        assert_eq!(
            Scanner::default().scan("<div class=\"before:content-[J\u{e4}s\u{f8}n_Doe] content-[\u{2192}]\">\u{306}</div>"),
            BTreeSet::from([
                "<div",
                ">\u{306}</div>",
                "before:content-[J\u{e4}s\u{f8}n_Doe]",
                "class=",
                "content-[\u{2192}]",
            ])
        );
    }

    #[test]
    fn scan_prevent_splitting_arbitrary_values() {
        assert_eq!(
            Scanner::default()
                .scan(r#"<div class="bg-red-300 content-[&#39;hello:>&#34;&#39;]"></div>"#),
            BTreeSet::from([
                "<div",
                "></div>",
                "bg-red-300",
                "class=",
                "content-[&#39;hello:>&#34;&#39;]",
            ])
        );
    }

    #[test]
    fn scan_with_arbitrary_variant() {
        assert_eq!(
            Scanner::default().scan(r#"<div class="[input[type=&#39;text&#39;]]:block"></div>"#),
            BTreeSet::from([
                "<div",
                "></div>",
                "class=",
                "[input[type=&#39;text&#39;]]:block",
            ])
        );
    }
}
