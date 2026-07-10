#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Color {
    namespace: "bg",
    prop: SingleProp("background-color"),
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "bg",
    prop: SingleProp("background-color"),
})
.hints(&[ArbitraryHint::Color])
.matcher(Color);
