#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "outline",
    prop: SingleProp("outline-width"),
})
.has_empty()
.template("{}px");

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "outline",
    prop: SingleProp("outline-width"),
})
.hints(&[ArbitraryHint::Length])
.matchers(&[Length, LineWidth], PluginArbitraryMatcherSeparation::None);
