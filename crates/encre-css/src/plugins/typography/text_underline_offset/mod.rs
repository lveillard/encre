#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::SamePropValues {
    prop: SingleProp("text-underline-offset"),
    values: &["auto"],
};

pub(crate) const PLUGIN_2: Plugin = Plugin::AnyNumber {
    prop: SingleProp("text-underline-offset"),
    has_empty: false,
    has_negative: false,
    divide_by: 1.0,
    template: "{}px",
};

pub(crate) const PLUGIN_3: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("text-underline-offset"),
    hints: &[PluginArbitraryHint::Length, PluginArbitraryHint::Percentage],
    matcher: OrMultiple(&[&Length, &Percentage, &Custom("auto")]),
};
