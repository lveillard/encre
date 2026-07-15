#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Number {
    namespace: "outline",
    prop: SingleProp("outline-width"),
    divide_by: 1.0,
})
.has_empty()
.template("{}px");

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "outline",
    prop: SingleProp("outline-width"),
})
.hints(&[ArbitraryHint::Length])
.matchers(&[Length, LineWidth], PluginArbitraryMatcherModifier::None);
