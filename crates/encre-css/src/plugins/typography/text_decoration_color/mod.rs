#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Color {
    namespace: "decoration",
    prop: MultipleProps(&["-webkit-text-decoration-color", "text-decoration-color"]),
});

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "decoration",
    prop: MultipleProps(&["-webkit-text-decoration-color", "text-decoration-color"]),
})
.hints(&[ArbitraryHint::Color])
.matchers(&[Color], PluginArbitraryMatcherModifier::None);
