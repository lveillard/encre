#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::AnyNumber {
    prop: SingleProp("outline-offset"),
    has_empty: false,
    has_negative: false,
    divide_by: 1.0,
})
.template("{}px");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::Arbitrary {
    prop: SingleProp("outline-offset"),
    hints: &[PluginArbitraryHint::Length],
    matcher: Length,
});
