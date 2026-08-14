#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Color {
    namespace: "text",
    prop: SingleProp("color"),
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "text",
    prop: SingleProp("color"),
})
.hints(&[ArbitraryHint::Color])
.matchers(&[Color], PluginArbitraryMatcherSeparation::None);
