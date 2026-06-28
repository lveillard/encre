#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::Color {
    prop: SingleProp("caret-color"),
};

pub(crate) const PLUGIN_2: Plugin = Plugin::OnlyArbitrary {
    prop: SingleProp("caret-color"),
    hints: &[PluginArbitraryHint::Color],
    matcher: Color,
};
