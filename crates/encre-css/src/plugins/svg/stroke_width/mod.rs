#![doc = include_str!("README.md")]
#![doc(alias = "svg")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN: Plugin = Plugin::new(PluginKind::Number {
    namespace: "stroke",
    prop: SingleProp("stroke-width"),
    divide_by: 1.0,
})
.template("{}px");

pub(crate) const PLUGIN_ARBITRARY: Plugin = Plugin::new(PluginKind::Arbitrary {
    namespace: "stroke",
    prop: SingleProp("stroke-width"),
})
.hints(&[ArbitraryHint::Length, ArbitraryHint::Percentage])
.matcher(CommaSeparated(&OrMultiple(&[
    &Length,
    &Percentage,
    &LineWidth,
    &Number,
])));
