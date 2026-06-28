#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::ListValues {
    prop: SingleProp("grid-auto-columns"),
    values: phf_map! {
        "auto" => "auto",
        "min" => "min-content",
        "max" => "max-content",
        "fr" => "minmax(0, 1fr)"
    },
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("grid-auto-columns"),
    hints: &[],
    matcher: All,
};
