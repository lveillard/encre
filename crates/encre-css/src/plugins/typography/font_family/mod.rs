#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("font-family"),
    values: phf_map! {
        "sans" => r#"ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji"#,
        "serif" => r#"Georgia, Cambria, "Times New Roman", Times, serif"#,
        "mono" => r#"Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace"#,
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("font-family"),
    hints: &[
        PluginArbitraryHint::GenericName,
        PluginArbitraryHint::FamilyName,
    ],
    matcher: CommaSeparated(&FontFamilyName),
});
