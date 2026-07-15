#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Color {
    namespace: "divide",
    prop: SingleProp("border-color"),
})
.extra_class(" > :not([hidden]) ~ :not([hidden])");

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "divide",
    prop: SingleProp("border-color"),
})
.extra_class(" > :not([hidden]) ~ :not([hidden])")
.hints(&[ArbitraryHint::Color])
.matchers(&[Color], PluginArbitraryMatcherModifier::None);
