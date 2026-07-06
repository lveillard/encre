#![doc = include_str!("README.md")]
#![doc(alias = "svg")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Number {
    prefix: "stroke",
    prop: SingleProp("stroke-width"),
    has_empty: false,
    has_negative: false,
    divide_by: 1.0,
})
.template("{}px");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "stroke",
    prop: SingleProp("stroke-width"),
})
.hints(&[PluginArbitraryHint::Length, PluginArbitraryHint::Percentage])
.matcher(CommaSeparated(&OrMultiple(&[
    &Length,
    &Percentage,
    &LineWidth,
    &Number,
])));
