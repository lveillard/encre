#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::Color {
    prop: SingleProp("--en-text-shadow-color"),
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("--en-text-shadow-color"),
    hints: &[PluginArbitraryHint::Color],
    matcher: Color,
};
