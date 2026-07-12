#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Color {
    namespace: "text-shadow",
    prop: SingleProp("--en-text-shadow-color"),
});

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "text-shadow",
    prop: SingleProp("--en-text-shadow-color"),
})
.hints(&[ArbitraryHint::Color])
.matcher(Color);
