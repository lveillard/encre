#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Color {
    prop: SingleProp("accent-color"),
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("accent-color"),
    hints: &[PluginArbitraryHint::Color],
    matcher: Color,
});
