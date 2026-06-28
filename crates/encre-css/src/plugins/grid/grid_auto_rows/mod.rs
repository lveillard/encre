#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::ListValues {
    prop: SingleProp("grid-auto-rows"),
    values: phf_map! {
        "auto" => "auto",
        "min" => "min-content",
        "max" => "max-content",
        "fr" => "minmax(0, 1fr)"
    },
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("grid-auto-rows"),
    hints: &[],
    matcher: All,
});
