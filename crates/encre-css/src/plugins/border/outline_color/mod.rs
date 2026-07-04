#![doc = include_str!("README.md")]
#![doc(alias = "outline")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Color {
    prefix: "outline",
    prop: SingleProp("outline-color"),
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "outline",
    prop: SingleProp("outline-color"),
})
.hints(&[PluginArbitraryHint::Color])
.matcher(Color);
