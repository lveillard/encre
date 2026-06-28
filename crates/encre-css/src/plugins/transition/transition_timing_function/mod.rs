#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("transition-timing-function"),
    values: phf_map! {
        "linear" => "linear",
        "in" => "cubic-bezier(0.4, 0, 1, 1)",
        "out" => "cubic-bezier(0, 0, 0.2, 1)",
        "in-out" => "cubic-bezier(0.4, 0, 0.2, 1)",
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("transition-timing-function"),
    hints: &[],
    matcher: All,
});
