#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;
use PluginArbitraryMatcher::*;

pub(crate) const PLUGIN_1: Plugin = Plugin::new(PluginKind::AnyNumber {
    prop: SingleProp("text-decoration-thickness"),
    has_empty: false,
    has_negative: false,
    divide_by: 1.0,
}).template("{}px");

pub(crate) const PLUGIN_2: Plugin = Plugin::new(PluginKind::SamePropValues {
    prop: SingleProp("text-decoration-thickness"),
    values: &["auto", "from-font"],
});

pub(crate) const PLUGIN_3: Plugin = Plugin::new(PluginKind::OnlyArbitrary {
    prop: SingleProp("text-decoration-thickness"),
    hints: &[PluginArbitraryHint::Length, PluginArbitraryHint::Percentage],
    matcher: OrMultiple(&[
        &Length,
        &Percentage,
        &CustomMultiple(&["auto", "from-font"]),
    ]),
});
