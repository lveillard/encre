#![doc = include_str!("README.md")]
#![doc(alias = "svg")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Color {
    namespace: "stroke",
    prop: SingleProp("stroke"),
});

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "stroke",
    prop: SingleProp("stroke"),
})
.hints(&[ArbitraryHint::Color])
.matchers(&[Color], PluginArbitraryMatcherModifier::None);
