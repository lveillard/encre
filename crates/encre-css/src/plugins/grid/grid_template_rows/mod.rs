#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::AnyNumber {
    prop: SingleProp("grid-template-rows"),
    has_empty: false,
    has_negative: false,
    divide_by: 1.0,
    template: "repeat({}, minmax(0, 1fr))",
};

pub(crate) const PLUGIN_2: Plugin = Plugin::SamePropValues {
    prop: SingleProp("grid-template-rows"),
    values: &["none"],
};

pub(crate) const PLUGIN_3: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("grid-template-rows"),
    hints: &[],
    matcher: All,
};
