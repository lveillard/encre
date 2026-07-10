#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Color {
    prefix: "shadow",
    prop: SingleProp("--en-shadow-color"),
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "shadow",
    prop: SingleProp("--en-shadow-color"),
})
.hints(&[ArbitraryHint::Color])
.matcher(Color);

pub(crate) const PLUGIN_INSET_1: Plugin = Plugin::new(PluginKind::Color {
    prefix: "inset-shadow",
    prop: SingleProp("--en-inset-shadow-color"),
});

pub(crate) const PLUGIN_INSET_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "inset-shadow",
    prop: SingleProp("--en-inset-shadow-color"),
})
.hints(&[ArbitraryHint::Color])
.matcher(Color);
