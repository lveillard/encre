#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::ListValues {
    prop: SingleProp("container-type"),
    values: phf_map! {
        "" => "inline-size",
    },
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("container-type"),
    hints: &[],
    matcher: All,
};
