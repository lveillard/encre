#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Color {
    prop: SingleProp("--en-shadow-color"),
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("--en-shadow-color"),
    hints: &[PluginArbitraryHint::Color],
    matcher: Color,
});

pub(crate) const PLUGIN_INSET_1: Plugin = Plugin::new(PluginKind::Color {
    prop: SingleProp("--en-inset-shadow-color"),
});

pub(crate) const PLUGIN_INSET_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("--en-inset-shadow-color"),
    hints: &[PluginArbitraryHint::Color],
    matcher: Color,
});
