#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Color {
    prop: SingleProp("border-color"),
}).extra_class(" > :not([hidden]) ~ :not([hidden])");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("border-color"),
    hints: &[PluginArbitraryHint::Color],
    matcher: Color,
}).extra_class(" > :not([hidden]) ~ :not([hidden])");
