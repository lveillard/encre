#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::ListValues {
    prop: SingleProp("aspect-ratio"),
    values: phf_map! {
        "auto" => "auto",
        "square" => "1 / 1",
        "video" => "16 / 9",
    },
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("aspect-ratio"),
    hints: &[],
    matcher: All,
};
