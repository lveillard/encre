#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::Color {
    prop: MultipleProps(&["-webkit-text-decoration-color", "text-decoration-color"]),
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: MultipleProps(&["-webkit-text-decoration-color", "text-decoration-color"]),
    hints: &[PluginArbitraryHint::Color],
    matcher: Color,
};
