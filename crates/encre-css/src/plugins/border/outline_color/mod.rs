#![doc = include_str!("README.md")]
#![doc(alias = "outline")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::Color {
    prop: SingleProp("outline-color"),
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("outline-color"),
    hints: &[PluginArbitraryHint::Color],
    matcher: Color,
};
