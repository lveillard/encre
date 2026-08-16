#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::ListValues(ListValues {
    prop: SingleProp("font-family"),
    values: map! {
        "font-sans" => r#"ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"#,
        "font-serif" => r#"Georgia, Cambria, "Times New Roman", Times, serif"#,
        "font-mono" => r#"Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace"#,
    },
    ..ListValues::default()
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::Arbitrary(Arbitrary {
    namespace: "font",
    prop: SingleProp("font-family"),
    hints: Some(&[ArbitraryHint::GenericName, ArbitraryHint::FamilyName]),
    matchers: Some((
        &[PluginArbitraryMatcher::FontFamilyName],
        PluginArbitraryMatcherSeparation::Comma,
    )),
    ..Arbitrary::default()
});
