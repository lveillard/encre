#![doc = include_str!("README.md")]
#![doc(alias = "svg")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: StaticPlugin = Plugin::new(PluginKind::Number {
    namespace: "stroke",
    prop: SingleProp("stroke-width"),
    divide_by: 1.0,
})
.template("{}px");

pub(crate) const PLUGIN_ARBITRARY: StaticPlugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "stroke",
    prop: SingleProp("stroke-width"),
})
.hints(&[ArbitraryHint::Length, ArbitraryHint::Percentage])
.matchers(&[
    Length,
    Percentage,
    LineWidth,
    Number,
], PluginArbitraryMatcherModifier::CommaSeparated);
