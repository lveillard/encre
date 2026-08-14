#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Color {
    namespace: "shadow",
    prop: SingleProp("--en-shadow-color"),
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "shadow",
    prop: SingleProp("--en-shadow-color"),
})
.hints(&[ArbitraryHint::Color])
.matchers(&[Color], PluginArbitraryMatcherSeparation::None);

pub(crate) const PLUGIN_INSET_1: StaticPlugin = Plugin::new(PluginKind::Color {
    namespace: "inset-shadow",
    prop: SingleProp("--en-inset-shadow-color"),
});

pub(crate) const PLUGIN_INSET_2: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "inset-shadow",
    prop: SingleProp("--en-inset-shadow-color"),
})
.hints(&[ArbitraryHint::Color])
.matchers(&[Color], PluginArbitraryMatcherSeparation::None);
