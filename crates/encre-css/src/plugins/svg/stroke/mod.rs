#![doc = include_str!("README.md")]
#![doc(alias = "svg")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Color {
    namespace: "stroke",
    prop: SingleProp("stroke"),
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "stroke",
    prop: SingleProp("stroke"),
})
.hints(&[ArbitraryHint::Color])
.matcher(Color);
