#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "outline-offset",
    prop: SingleProp("outline-offset"),
})
.template("{}px");

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "outline-offset",
    prop: SingleProp("outline-offset"),
})
.hints(&[ArbitraryHint::Length])
.matchers(&[Length], PluginArbitraryMatcherModifier::None);
