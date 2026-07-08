#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Number {
    prefix: "outline",
    prop: SingleProp("outline-width"),
    divide_by: 1.0,
})
.has_empty()
.template("{}px");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "outline",
    prop: SingleProp("outline-width"),
})
.hints(&[PluginArbitraryHint::Length])
.matcher(Or(&Length, &LineWidth));
