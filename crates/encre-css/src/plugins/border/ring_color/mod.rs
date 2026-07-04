#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Color {
    prefix: "ring",
    prop: SingleProp("--en-ring-color"),
});

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "ring",
    prop: SingleProp("--en-ring-color"),
})
.hints(&[PluginArbitraryHint::Color])
.matcher(Color);

pub(crate) const PLUGIN_INSET_1: Plugin = Plugin::new(PluginKind::Color {
    prefix: "inset-ring",
    prop: SingleProp("--en-inset-ring-color"),
});

pub(crate) const PLUGIN_INSET_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "inset-ring",
    prop: SingleProp("--en-inset-ring-color"),
})
.hints(&[PluginArbitraryHint::Color])
.matcher(Color);
