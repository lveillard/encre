#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::AnyNumber {
    prop: SingleProp("transition-delay"),
    has_empty: false,
    has_negative: false,
    divide_by: 1.0,
})
.template("{}ms");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("transition-delay"),
    hints: &[],
    matcher: Time,
});
