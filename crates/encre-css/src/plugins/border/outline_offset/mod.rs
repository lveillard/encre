#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Number {
    namespace: "outline-offset",
    prop: SingleProp("outline-offset"),
    divide_by: 1.0,
})
.template("{}px");

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "outline-offset",
    prop: SingleProp("outline-offset"),
})
.hints(&[ArbitraryHint::Length])
.matchers(&[Length], PluginArbitraryMatcherModifier::None);
