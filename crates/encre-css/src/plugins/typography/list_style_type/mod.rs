#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::SamePropValues {
    prop: SingleProp("list-style-type"),
    values: &["disc", "decimal", "none"],
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("list-style-type"),
    hints: &[],
    matcher: All,
};
