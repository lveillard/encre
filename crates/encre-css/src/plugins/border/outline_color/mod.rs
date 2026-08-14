#![doc = include_str!("README.md")]
#![doc(alias = "outline")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Color {
    namespace: "outline",
    prop: SingleProp("outline-color"),
});

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "outline",
    prop: SingleProp("outline-color"),
})
.hints(&[ArbitraryHint::Color])
.matchers(&[Color], PluginArbitraryMatcherSeparation::None);
