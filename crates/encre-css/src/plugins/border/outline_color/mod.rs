#![doc = include_str!("README.md")]
#![doc(alias = "outline")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Color {
    namespace: "outline",
    prop: SingleProp("outline-color"),
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "outline",
    prop: SingleProp("outline-color"),
})
.hints(&[ArbitraryHint::Color])
.matcher(Color);
