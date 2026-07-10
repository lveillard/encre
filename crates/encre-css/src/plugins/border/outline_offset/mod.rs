#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::Number {
    prefix: "outline-offset",
    prop: SingleProp("outline-offset"),
    divide_by: 1.0,
})
.template("{}px");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prefix: "outline-offset",
    prop: SingleProp("outline-offset"),
})
.hints(&[ArbitraryHint::Length])
.matcher(Length);
